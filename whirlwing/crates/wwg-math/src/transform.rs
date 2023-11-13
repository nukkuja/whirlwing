use crate::core::*;

#[derive(Debug)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}

impl Transform {
    #[inline]
    pub fn new(position: Vector3, rotation: Quaternion, scale: Vector3) -> Self {
        Transform {
            position: position,
            rotation: rotation,
            scale: scale,
        }
    }

    #[inline]
    pub fn matrix(self) -> Matrix4 {
        #[rustfmt::skip]
        let pos = Matrix4::new(
            1.0, 0.0, 0.0, self.position.x,
            0.0, 1.0, 0.0, self.position.y,
            0.0, 0.0, 1.0, self.position.z,
            0.0, 0.0, 0.0, 1.0
        );
        let rot = self.rotation.to_rotation_matrix();
        #[rustfmt::skip]
        let scale = Matrix4::new(
            self.scale.x, 0.0, 0.0, 0.0,
            0.0, self.scale.y, 0.0, 0.0,
            0.0, 0.0, self.scale.z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );
        pos * rot * scale
    }
}

#[derive(Debug)]
pub struct ProjectionMatrix {
    matrix: Matrix4,
}

impl std::ops::Deref for ProjectionMatrix {
    type Target = Matrix4;
    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

impl ProjectionMatrix {
    #[inline]
    pub fn new(near: f32, far: f32, angle_rad: f32, aspect: f32) -> Self {
        let half_cot = 1.0f32 / f32::tan(angle_rad / 2.0f32);
        ProjectionMatrix {
            matrix: Matrix4::new(
                half_cot / aspect, 0.0, 0.0, 0.0,
                0.0, half_cot, 0.0, 0.0,
                0.0, 0.0, (far + near) / (near - far), (2.0 * far * near) / (near - far),
                0.0, 0.0, -1.0, 0.0
            )
        }
    }
}