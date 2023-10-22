#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
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
        Vector3 {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    #[inline]
    pub fn normalized(&self) -> Vector3 {
        let len = self.len();
        Vector3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    #[inline]
    pub fn normalize(&mut self) {
        let len = self.len();
        *self /= len;
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4 { x, y, z, w }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Matrix4 {
    pub x1: f32,
    pub x2: f32,
    pub x3: f32,
    pub x4: f32,
    pub y1: f32,
    pub y2: f32,
    pub y3: f32,
    pub y4: f32,
    pub z1: f32,
    pub z2: f32,
    pub z3: f32,
    pub z4: f32,
    pub w1: f32,
    pub w2: f32,
    pub w3: f32,
    pub w4: f32,
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
            x1,
            x2,
            x3,
            x4,
            y1,
            y2,
            y3,
            y4,
            z1,
            z2,
            z3,
            z4,
            w1,
            w2,
            w3,
            w4,
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
        Matrix4 {
            x1: 1.0 - 2.0 * self.j * self.j - 2.0 * self.k * self.k,
            x2: 2.0 * self.i * self.j - 2.0 * self.w * self.k,
            x3: 2.0 * self.i * self.k + 2.0 * self.w * self.j,
            x4: 0.0,
            y1: 2.0 * self.i * self.j + 2.0 * self.w * self.k,
            y2: 1.0 - 2.0 * self.i * self.i - 2.0 * self.k * self.k,
            y3: 2.0 * self.j * self.k - 2.0 * self.w * self.i,
            y4: 0.0,
            z1: 2.0 * self.i * self.k - 2.0 * self.w * self.j,
            z2: 2.0 * self.j * self.k + 2.0 * self.w * self.i,
            z3: 1.0 - 2.0 * self.i * self.i - 2.0 * self.j * self.j,
            z4: 0.0,
            w1: 0.0,
            w2: 0.0,
            w3: 0.0,
            w4: 1.0,
        }
    }
}