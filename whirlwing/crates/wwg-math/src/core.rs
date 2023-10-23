#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vector3 {
    pub data: [f32; 3],
}

impl Vector3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { data: [x, y, z] }
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
            (self.x * rhs.y) - (self.y * rhs.x)
        )
    }

    #[inline]
    pub fn normalized(&self) -> Vector3 {
        let len = self.len();
        Vector3::new(
            self.x / len,
            self.y / len,
            self.z / len,
        )
    }

    #[inline]
    pub fn normalize(&mut self) {
        let len = self.len();
        *self /= len;
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vector4 {
    pub data: [f32; 4],
}

impl Vector4 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4 { data: [x, y, z, w] }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Matrix4 {
    pub data: [f32; 16],
}

impl Matrix4 {
    pub fn new(
        x1: f32,
        x2: f32,
        x3: f32,
        x4: f32,
        y1: f32,
        y2: f32,
        y3: f32,
        y4: f32,
        z1: f32,
        z2: f32,
        z3: f32,
        z4: f32,
        w1: f32,
        w2: f32,
        w3: f32,
        w4: f32,
    ) -> Self {
        Matrix4 {
            data: [x1, x2, x3, x4, y1, y2, y3, y4, z1, z2, z3, z4, w1, w2, w3, w4]
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
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
    pub fn to_rotation_matrix(self) -> Matrix4 {
        Matrix4::new(
            1.0 - 2.0 * self.j * self.j - 2.0 * self.k * self.k,
            2.0 * self.i * self.j - 2.0 * self.w * self.k,
            2.0 * self.i * self.k + 2.0 * self.w * self.j,
            0.0,
            2.0 * self.i * self.j + 2.0 * self.w * self.k,
            1.0 - 2.0 * self.i * self.i - 2.0 * self.k * self.k,
            2.0 * self.j * self.k - 2.0 * self.w * self.i,
            0.0,
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