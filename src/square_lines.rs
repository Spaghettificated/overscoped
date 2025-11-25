use bevy::{asset::RenderAssetUsages, math::bounding::{Aabb2d, Aabb3d, Bounded2d, BoundedExtrusion, BoundingCircle, BoundingVolume}, mesh::{Indices, PrimitiveTopology}, prelude::*};
use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Clone, Reflect, Default)]
pub enum AxisDirection{
    #[default]
    Horizontal,
    Vertical,
}

#[derive(PartialEq, Debug, Clone, Reflect, Default)]
pub struct SqLine{
    coords: Vec<f32>,
    first_axis: AxisDirection,
}

impl SqLine {
    pub fn new(coords: Vec<f32>, first_axis: AxisDirection) -> Self {
        Self { coords, first_axis }
    }
    pub fn points(&self) -> Vec<Vec2>{
        let mut points = Vec::with_capacity(self.coords.len()-1);
        for (&x,&y) in self.coords.iter().tuple_windows() {
            let (x, y) = if self.first_axis == AxisDirection::Vertical {(x, y)} else {(y, x)};
            points.push(Vec2 { x, y });
        }
        points
    }
    pub fn x_values(&self) -> Vec<f32>{
        let mut iter = self.coords.iter().map(|x| *x);
        if self.first_axis == AxisDirection::Vertical{
            iter.next();
        }
        iter.step_by(2).collect_vec()
    }
    pub fn y_values(&self) -> Vec<f32>{
        let mut iter = self.coords.iter().map(|x| *x);
        if self.first_axis == AxisDirection::Horizontal{
            iter.next();
        }
        iter.step_by(2).collect_vec()
    }
}

impl Primitive2d for SqLine {}

impl Bounded2d for SqLine {
    fn aabb_2d(&self, isometry: impl Into<Isometry2d>) -> Aabb2d {
        Aabb2d::from_point_cloud(isometry, &self.points())
    }

    fn bounding_circle(&self, isometry: impl Into<Isometry2d>) -> BoundingCircle {
        BoundingCircle::from_point_cloud(isometry, &self.points())
    }
}

impl BoundedExtrusion for SqLine {
    fn extrusion_aabb_3d(&self, half_depth: f32, isometry: impl Into<Isometry3d>) -> Aabb3d {
        let isometry = isometry.into();
        let aabb = Aabb3d::from_point_cloud(isometry, self.points().iter().map(|v| v.extend(0.)));
        let depth = isometry.rotation * Vec3A::new(0., 0., half_depth);

        aabb.grow(depth.abs())
    }
}

#[derive(Clone, Debug, Default, Reflect)]
#[reflect(Default, Debug, Clone)]
pub struct SqLineMeshBuilder {
    polyline: SqLine,
}

impl MeshBuilder for SqLineMeshBuilder {
    fn build(&self) -> Mesh {
        let positions: Vec<_> = self
            .polyline
            .points()
            .iter()
            .map(|v| v.extend(0.0))
            .collect();

        let indices = Indices::U32(
            (0..self.polyline.points().len() as u32 - 1)
                .flat_map(|i| [i, i + 1])
                .collect(),
        );

        Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default())
            .with_inserted_indices(indices)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    }
}


impl Meshable for SqLine {
    type Output = SqLineMeshBuilder;

    fn mesh(&self) -> Self::Output {
        SqLineMeshBuilder {
            polyline: self.clone(),
        }
    }
}

impl From<SqLine> for Mesh {
    fn from(polyline: SqLine) -> Self {
        polyline.mesh().build()
    }
}
