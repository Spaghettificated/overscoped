use bevy::prelude::*;

pub fn as_rgb(code: i32) -> Color{
    return Color::linear_rgb(((code & 0xFF0000)>>16) as f32 / 255., ((code & 0x00FF00)>>8) as f32 / 255., (code & 0x0000FF) as f32 / 255.);
}
