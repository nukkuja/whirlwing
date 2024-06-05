use wwg_math::{Vec3, Rotor3, Isometry3, Mat4};

pub struct Camera(Isometry3);

impl Camera {
    // To set default rotation use Rotor3::identity()
    pub fn new(position: Vec3, rotation: Rotor3) -> Self {
        Camera(Isometry3::new(position, rotation))
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.0.translation = position;
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.0.prepend_translation(translation);
    }

    pub fn rotate_xz(&mut self, angle: f32) {
        self.0.rotation = Rotor3::from_rotation_xz(angle) * self.0.rotation;
    }

    pub fn rotate_yz(&mut self, angle: f32) {
        self.0.rotation = Rotor3::from_rotation_yz(angle) * self.0.rotation;
    }

    pub(crate) fn create_cam_tmp() -> Self {
        Camera::new(Vec3::new(0.0, 0.0, 5.0), Rotor3::identity())
    }

    pub(crate) fn view_matrix(&self) -> Mat4 {
        self.0.into_homogeneous_matrix().inversed()
    }
}