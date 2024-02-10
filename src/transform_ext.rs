use bevy::prelude::*;

pub trait TransformExt {
    fn get_z_rotation(&self) -> f32;
}

impl TransformExt for GlobalTransform {
    fn get_z_rotation(&self) -> f32 {
        self.compute_transform().rotation.to_euler(EulerRot::XYZ).2
    }
}

impl TransformExt for Transform {
    fn get_z_rotation(&self) -> f32 {
        self.rotation.to_euler(EulerRot::XYZ).2
    }
}
