use bevy::{color::palettes::css::MAGENTA, prelude::*};

#[derive(Component, Deref, DerefMut)]
struct Curve2d(Box<dyn Curve<Vec2> + Send + Sync>);


fn draw_curve(
    curves: Query<&Curve2d>, 
    mut gizmos: Gizmos,
) {
    for curve in curves {
        gizmos.curve_2d(curve, (0..=100).map(|n| n as f32 / 100.0), MAGENTA);
    }
}