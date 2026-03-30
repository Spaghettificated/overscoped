use bevy::{ecs::system::SystemParam, prelude::*, window::PrimaryWindow};

pub fn as_rgb(code: i32) -> Color{
    return Color::linear_rgb(((code & 0xFF0000)>>16) as f32 / 255., ((code & 0x00FF00)>>8) as f32 / 255., (code & 0x0000FF) as f32 / 255.);
}

#[derive(SystemParam)]
pub struct MouseQuery<'w, 's>{
    pub window: Single<'w,'s,&'static Window, With<PrimaryWindow>>,
    pub camera: Single<'w,'s,(&'static Camera, &'static GlobalTransform)>,
}

impl<'w, 's> MouseQuery<'w, 's> {
    pub fn position(&self) -> Option<Vec2> {
        let (camera, camera_transform) = *self.camera;
        self.window.cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
        .map(|ray| ray.expect("cannot read mouse position").origin.truncate())
    }
}