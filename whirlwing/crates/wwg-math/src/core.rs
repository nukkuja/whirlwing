#[repr(C)]
#[derive(Debug, PartialEq, Clone)]
pub struct Vector3 {
    data: [f32; 3],
}

impl Vector3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { data: [x, y, z] }
    }

    #[inline]
    pub fn zero() -> Self {
        Vector3 { data: [0.0, 0.0, 0.0] }
    }

    #[inline]
    pub fn one() -> Self {
        Vector3 { data: [1.0, 1.0, 1.0] }
    }

    #[inline]
    pub fn up() -> Self {
        Vector3 { data: [0.0, 1.0, 0.0] }
    }

    #[inline]
    pub fn len(&self) -> f32 {
        f32::sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z))
    }

    #[inline]
    pub fn dot(&self, rhs: &Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    #[inline]
    pub fn cross(&self, rhs: &Self) -> Self {
        Vector3::new(
            (self.y * rhs.z) - (self.z * rhs.y),
            (self.z * rhs.x) - (self.x * rhs.z),
            (self.x * rhs.y) - (self.y * rhs.x),
        )
    }

    #[inline]
    pub fn normalized(&self) -> Vector3 {
        let len = self.len();
        Vector3::new(self.x / len, self.y / len, self.z / len)
    }

    #[inline]
    pub fn normalize(&mut self) {
        let len = self.len();
        *self /= len;
    }

    #[inline]
    pub fn ptr(&self) -> *const f32 {
        self.data.as_ptr()
    }

    #[inline]
    pub fn mut_ptr(&mut self) -> *mut f32 {
        self.data.as_mut_ptr()
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Vector4 {
    data: [f32; 4],
}

impl Vector4 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4 { data: [x, y, z, w] }
    }

    #[inline]
    pub fn ptr(&self) -> *const f32 {
        self.data.as_ptr()
    }

    #[inline]
    pub fn mut_ptr(&mut self) -> *mut f32 {
        self.data.as_mut_ptr()
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Matrix4 {
    data: [f32; 16],
}

impl Matrix4 {
    /// OpenGL uses matrices with column major order so I defined it this way.
    /// If I need to change it I would also need to change the order in deref.rs file.
    #[rustfmt::skip]
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn new(
        x1: f32, x2: f32, x3: f32, x4: f32,
        y1: f32, y2: f32, y3: f32, y4: f32,
        z1: f32, z2: f32, z3: f32, z4: f32,
        w1: f32, w2: f32, w3: f32, w4: f32,
    ) -> Self {
        Matrix4 {
            data: [
                x1, y1, z1, w1,
                x2, y2, z2, w2,
                x3, y3, z3, w3,
                x4, y4, z4, w4,
            ]
        }
    }

    #[rustfmt::skip]
    #[inline]
    pub fn one() -> Self {
        Matrix4 {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ]
        }
    }

    #[inline]
    pub fn zero() -> Self {
        Matrix4 { data: [0.0; 16] }
    }

    #[inline]
    pub fn ptr(&self) -> *const f32 {
        self.data.as_ptr()
    }

    #[inline]
    pub fn mut_ptr(&mut self) -> *mut f32 {
        self.data.as_mut_ptr()
    }

    #[inline]
    pub fn transpose(&mut self) {
        (self.x2, self.y1) = (self.y1, self.x2);
        (self.x3, self.z1) = (self.z1, self.x3);
        (self.x4, self.w1) = (self.w1, self.x4);
        (self.y3, self.z2) = (self.z2, self.y3);
        (self.y4, self.w2) = (self.w2, self.y4);
        (self.z4, self.w3) = (self.w3, self.z4);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub w: f32,
    pub i: f32,
    pub j: f32,
    pub k: f32,
}

impl Quaternion {
    #[inline]
    pub fn from_axis_angle(axis: &Vector3, angle: f32) -> Self {
        let angle = angle / 2.0;
        let sin = angle.sin();
        Quaternion {
            w: angle.cos(),
            i: axis.x * sin,
            j: axis.y * sin,
            k: axis.z * sin,
        }
    }

    #[inline]
    pub unsafe fn new_unchecked(w: f32, i: f32, j: f32, k: f32) -> Self {
        Quaternion { w, i, j, k }
    }

    #[inline]
    pub fn inverse(&self) -> Quaternion {
        unsafe {
            Quaternion::new_unchecked(self.w, -self.i, -self.j, -self.k)
        }
    }

    /// I'm not sure about this function but docs say next:
    /// Active rotation is when the point rotated with respect to the coordinate system:
    /// `p' = q^-1 * p * q`
    /// Passive rotation is when the coordinate system is rotated with respect to the point:
    /// `p' = q * p * q^-1`
    #[inline]
    pub fn rotate_point(&self, point: &Vector3) -> Vector3 {
        let qp = unsafe { Quaternion::new_unchecked(0.0f32, point.x, point.y, point.z) };
        let qres = self * qp * self.inverse();
        Vector3::new(qres.i, qres.j, qres.k)
    }

    #[inline]
    pub fn to_rotation_matrix(self) -> Matrix4 {
        Matrix4::new(
            // self.w * self.w + self.i * self.i - self.j * self.j - self.k * self.k,
            1.0 - 2.0 * self.j * self.j - 2.0 * self.k * self.k,
            2.0 * self.i * self.j - 2.0 * self.w * self.k,
            2.0 * self.i * self.k + 2.0 * self.w * self.j,
            0.0,
            // self.w * self.w - self.i * self.i + self.j * self.j - self.k * self.k,
            2.0 * self.i * self.j + 2.0 * self.w * self.k,
            1.0 - 2.0 * self.i * self.i - 2.0 * self.k * self.k,
            2.0 * self.j * self.k - 2.0 * self.w * self.i,
            0.0,
            // self.w * self.w - self.i * self.i - self.j * self.j + self.k * self.k,
            2.0 * self.i * self.k - 2.0 * self.w * self.j,
            2.0 * self.j * self.k + 2.0 * self.w * self.i,
            1.0 - 2.0 * self.i * self.i - 2.0 * self.j * self.j,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }
}
