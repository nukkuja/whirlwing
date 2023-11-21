use crate::core::*;

#[repr(C)]
pub struct Camera {
    position: Vector3,
    rotation: Quaternion,
}

impl Camera {
    #[inline]
    pub fn new(position: &Vector3, rotation: &Quaternion) -> Self {
        Camera { position: position.clone(), rotation: rotation.clone(), }
    }

    #[inline]
    pub fn look_at(position: &Vector3, target: &Vector3, up: &Vector3) -> Self {
        Camera { position: position.clone(), rotation: Quaternion::look_at(position, target, up) }
    }

    // TODO: Optimize this function
    #[inline]
    pub fn view(&self) -> Matrix4 {
        let inv_rot = self.rotation.inverse();
        let f = unsafe { Vector3::from_quaternion(&(inv_rot * Vector3::forward() * &self.rotation)) };
        let r = unsafe { Vector3::from_quaternion(&(inv_rot * Vector3::right() * &self.rotation)) };
        let u = unsafe { Vector3::from_quaternion(&(inv_rot * Vector3::up() * &self.rotation)) };

        let view = Matrix4::new(
            r.x, r.y, r.z, Vector3::dot(&r, &-&self.position),
            u.x, u.y, u.z, Vector3::dot(&u, &-&self.position),
            -f.x, -f.y, -f.z, Vector3::dot(&f, &self.position),
            0.0, 0.0, 0.0, 1.0,
        );
        view
    }
}