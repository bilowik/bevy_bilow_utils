use std::f32::consts::PI;

use bevy::{
    ecs::system::Command,
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};

pub mod modifier;
pub mod mouse;
pub mod seed;
pub mod transform_ext;

// Returns 4 shapes and their positions that will form the box.
pub fn create_box(bounds: Rect, line_width: f32) -> Vec<(Rectangle, Vec2)> {
    let center = bounds.center();
    let mut box_shapes = Vec::with_capacity(4);
    let side = Rectangle::new(line_width, bounds.height() + line_width);
    let top_bottom = Rectangle::new(bounds.width() + line_width, line_width);
    for x_pos in [bounds.min.x, bounds.max.x] {
        box_shapes.push((side, Vec2::new(x_pos, center.y)));
    }
    for y_pos in [bounds.min.y, bounds.max.y] {
        box_shapes.push((top_bottom, Vec2::new(center.x, y_pos)));
    }

    box_shapes
}

pub struct SpawnBox<T: Material2d> {
    pub material: Handle<T>,
    pub center: Vec3,
    pub bounds: Rect,
    pub line_width: f32,
}

impl<T: Material2d> Default for SpawnBox<T> {
    fn default() -> Self {
        Self {
            material: Default::default(),
            center: Default::default(),
            bounds: Default::default(),
            line_width: 1.0,
        }
    }
}

impl<T: Material2d> Command for SpawnBox<T> {
    fn apply(self, world: &mut World) {
        for (r, offset) in create_box(self.bounds, self.line_width) {
            let mesh = if let Some(mut meshes) = world.get_resource_mut::<Assets<Mesh>>() {
                meshes.add(r).into()
            } else {
                error!("Tried spawning box but could not get Assets<Mesh> resource");
                return;
            };
            world.spawn(MaterialMesh2dBundle {
                material: self.material.clone(),
                mesh,
                transform: Transform::from_translation(self.center + offset.extend(0.0)),
                ..default()
            });
        }
    }
}

pub const DOUBLE_PI: f32 = PI * 2.0;
pub fn ang_norm(angle: f32) -> f32 {
    let angle = ((angle + PI) % DOUBLE_PI) - PI;
    -(if angle < -PI {
        angle + (2.0 * PI)
    } else {
        angle
    })
}
