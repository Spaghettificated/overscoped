use bevy::{prelude::*, tasks::ParallelSlice};
use itertools::Itertools;
enum Coord{
    X(f32),
    Y(f32),
}
#[derive(PartialEq, Eq, Debug)]
pub enum AxisDirection{
    Horizontal,
    Vertical,
}

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