use crate::core::*;

#[derive(Debug)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}

impl Transform {
    #[inline]
    pub fn new(position: &Vector3, rotation: &Quaternion, scale: &Vector3) -> Self {
        Transform {
            position: *position,
            rotation: *rotation,
            scale: *scale,
        }
    }

    #[inline]
    pub fn matrix(self) -> Matrix4 {
        let pos = Matrix4::new(
            1.0, 0.0, 0.0, self.position.x,
            0.0, 1.0, 0.0, self.position.y,
            0.0, 0.0, 1.0, self.position.z,
            0.0, 0.0, 0.0, 1.0
        );
        let rot = self.rotation.to_rotation_matrix();
        let scale = Matrix4::new(
            self.scale.x, 0.0, 0.0, 0.0,
            0.0, self.scale.y, 0.0, 0.0,
            0.0, 0.0, self.scale.z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        // scale
        &pos * &rot * &scale
    }
}