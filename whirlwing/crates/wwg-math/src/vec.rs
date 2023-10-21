#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    pub fn len(&self) -> f32 {
        f32::sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z))
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vector3 {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }

    pub fn normalized(&self) -> Vector3 {
        let len = self.len();
        Vector3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

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

impl std::ops::Add for Matrix4 {
    type Output = Matrix4;
    fn add(self, rhs: Self) -> Self::Output {
        Matrix4 {
            x1: self.x1 + rhs.x1,
            x2: self.x2 + rhs.x2,
            x3: self.x3 + rhs.x3,
            x4: self.x4 + rhs.x4,
            y1: self.y1 + rhs.y1,
            y2: self.y2 + rhs.y2,
            y3: self.y3 + rhs.y3,
            y4: self.y4 + rhs.y4,
            z1: self.z1 + rhs.z1,
            z2: self.z2 + rhs.z2,
            z3: self.z3 + rhs.z3,
            z4: self.z4 + rhs.z4,
            w1: self.w1 + rhs.w1,
            w2: self.w2 + rhs.w2,
            w3: self.w3 + rhs.w3,
            w4: self.w4 + rhs.w4,
        }
    }
}
impl std::ops::Add for &Matrix4 {
    type Output = Matrix4;
    fn add(self, rhs: Self) -> Self::Output {
        Matrix4 {
            x1: self.x1 + rhs.x1,
            x2: self.x2 + rhs.x2,
            x3: self.x3 + rhs.x3,
            x4: self.x4 + rhs.x4,
            y1: self.y1 + rhs.y1,
            y2: self.y2 + rhs.y2,
            y3: self.y3 + rhs.y3,
            y4: self.y4 + rhs.y4,
            z1: self.z1 + rhs.z1,
            z2: self.z2 + rhs.z2,
            z3: self.z3 + rhs.z3,
            z4: self.z4 + rhs.z4,
            w1: self.w1 + rhs.w1,
            w2: self.w2 + rhs.w2,
            w3: self.w3 + rhs.w3,
            w4: self.w4 + rhs.w4,
        }
    }
}
impl std::ops::AddAssign<&Matrix4> for Matrix4 {
    fn add_assign(&mut self, rhs: &Matrix4) {
        self.x1 += rhs.x1;
        self.x2 += rhs.x2;
        self.x3 += rhs.x3;
        self.x4 += rhs.x4;
        self.y1 += rhs.y1;
        self.y2 += rhs.y2;
        self.y3 += rhs.y3;
        self.y4 += rhs.y4;
        self.z1 += rhs.z1;
        self.z2 += rhs.z2;
        self.z3 += rhs.z3;
        self.z4 += rhs.z4;
        self.w1 += rhs.w1;
        self.w2 += rhs.w2;
        self.w3 += rhs.w3;
        self.w4 += rhs.w4;
    }
}
impl std::ops::Sub for Matrix4 {
    type Output = Matrix4;
    fn sub(self, rhs: Self) -> Self::Output {
        Matrix4 {
            x1: self.x1 - rhs.x1,
            x2: self.x2 - rhs.x2,
            x3: self.x3 - rhs.x3,
            x4: self.x4 - rhs.x4,
            y1: self.y1 - rhs.y1,
            y2: self.y2 - rhs.y2,
            y3: self.y3 - rhs.y3,
            y4: self.y4 - rhs.y4,
            z1: self.z1 - rhs.z1,
            z2: self.z2 - rhs.z2,
            z3: self.z3 - rhs.z3,
            z4: self.z4 - rhs.z4,
            w1: self.w1 - rhs.w1,
            w2: self.w2 - rhs.w2,
            w3: self.w3 - rhs.w3,
            w4: self.w4 - rhs.w4,
        }
    }
}
impl std::ops::Sub for &Matrix4 {
    type Output = Matrix4;
    fn sub(self, rhs: Self) -> Self::Output {
        Matrix4 {
            x1: self.x1 - rhs.x1,
            x2: self.x2 - rhs.x2,
            x3: self.x3 - rhs.x3,
            x4: self.x4 - rhs.x4,
            y1: self.y1 - rhs.y1,
            y2: self.y2 - rhs.y2,
            y3: self.y3 - rhs.y3,
            y4: self.y4 - rhs.y4,
            z1: self.z1 - rhs.z1,
            z2: self.z2 - rhs.z2,
            z3: self.z3 - rhs.z3,
            z4: self.z4 - rhs.z4,
            w1: self.w1 - rhs.w1,
            w2: self.w2 - rhs.w2,
            w3: self.w3 - rhs.w3,
            w4: self.w4 - rhs.w4,
        }
    }
}
impl std::ops::SubAssign<&Matrix4> for Matrix4 {
    fn sub_assign(&mut self, rhs: &Matrix4) {
        self.x1 -= rhs.x1;
        self.x2 -= rhs.x2;
        self.x3 -= rhs.x3;
        self.x4 -= rhs.x4;
        self.y1 -= rhs.y1;
        self.y2 -= rhs.y2;
        self.y3 -= rhs.y3;
        self.y4 -= rhs.y4;
        self.z1 -= rhs.z1;
        self.z2 -= rhs.z2;
        self.z3 -= rhs.z3;
        self.z4 -= rhs.z4;
        self.w1 -= rhs.w1;
        self.w2 -= rhs.w2;
        self.w3 -= rhs.w3;
        self.w4 -= rhs.w4;
    }
}
impl std::ops::Mul for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: Self) -> Self::Output {
        Matrix4 {
            x1: (self.x1 * rhs.x1) + (self.x2 * rhs.y1) + (self.x3 * rhs.z1) + (self.x4 * rhs.w1),
            x2: (self.x1 * rhs.x2) + (self.x2 * rhs.y2) + (self.x3 * rhs.z2) + (self.x4 * rhs.w2),
            x3: (self.x1 * rhs.x3) + (self.x2 * rhs.y3) + (self.x3 * rhs.z3) + (self.x4 * rhs.w3),
            x4: (self.x1 * rhs.x4) + (self.x2 * rhs.y4) + (self.x3 * rhs.z4) + (self.x4 * rhs.w4),
            y1: (self.y1 * rhs.x1) + (self.y2 * rhs.y1) + (self.y3 * rhs.z1) + (self.y4 * rhs.w1),
            y2: (self.y1 * rhs.x2) + (self.y2 * rhs.y2) + (self.y3 * rhs.z2) + (self.y4 * rhs.w2),
            y3: (self.y1 * rhs.x3) + (self.y2 * rhs.y3) + (self.y3 * rhs.z3) + (self.y4 * rhs.w3),
            y4: (self.y1 * rhs.x4) + (self.y2 * rhs.y4) + (self.y3 * rhs.z4) + (self.y4 * rhs.w4),
            z1: (self.z1 * rhs.x1) + (self.z2 * rhs.y1) + (self.z3 * rhs.z1) + (self.z4 * rhs.w1),
            z2: (self.z1 * rhs.x2) + (self.z2 * rhs.y2) + (self.z3 * rhs.z2) + (self.z4 * rhs.w2),
            z3: (self.z1 * rhs.x3) + (self.z2 * rhs.y3) + (self.z3 * rhs.z3) + (self.z4 * rhs.w3),
            z4: (self.z1 * rhs.x4) + (self.z2 * rhs.y4) + (self.z3 * rhs.z4) + (self.z4 * rhs.w4),
            w1: (self.w1 * rhs.x1) + (self.w2 * rhs.y1) + (self.w3 * rhs.z1) + (self.w4 * rhs.w1),
            w2: (self.w1 * rhs.x2) + (self.w2 * rhs.y2) + (self.w3 * rhs.z2) + (self.w4 * rhs.w2),
            w3: (self.w1 * rhs.x3) + (self.w2 * rhs.y3) + (self.w3 * rhs.z3) + (self.w4 * rhs.w3),
            w4: (self.w1 * rhs.x4) + (self.w2 * rhs.y4) + (self.w3 * rhs.z4) + (self.w4 * rhs.w4),
        }
    }
}
impl std::ops::Mul for &Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: Self) -> Self::Output {
        Matrix4 {
            x1: (self.x1 * rhs.x1) + (self.x2 * rhs.y1) + (self.x3 * rhs.z1) + (self.x4 * rhs.w1),
            x2: (self.x1 * rhs.x2) + (self.x2 * rhs.y2) + (self.x3 * rhs.z2) + (self.x4 * rhs.w2),
            x3: (self.x1 * rhs.x3) + (self.x2 * rhs.y3) + (self.x3 * rhs.z3) + (self.x4 * rhs.w3),
            x4: (self.x1 * rhs.x4) + (self.x2 * rhs.y4) + (self.x3 * rhs.z4) + (self.x4 * rhs.w4),
            y1: (self.y1 * rhs.x1) + (self.y2 * rhs.y1) + (self.y3 * rhs.z1) + (self.y4 * rhs.w1),
            y2: (self.y1 * rhs.x2) + (self.y2 * rhs.y2) + (self.y3 * rhs.z2) + (self.y4 * rhs.w2),
            y3: (self.y1 * rhs.x3) + (self.y2 * rhs.y3) + (self.y3 * rhs.z3) + (self.y4 * rhs.w3),
            y4: (self.y1 * rhs.x4) + (self.y2 * rhs.y4) + (self.y3 * rhs.z4) + (self.y4 * rhs.w4),
            z1: (self.z1 * rhs.x1) + (self.z2 * rhs.y1) + (self.z3 * rhs.z1) + (self.z4 * rhs.w1),
            z2: (self.z1 * rhs.x2) + (self.z2 * rhs.y2) + (self.z3 * rhs.z2) + (self.z4 * rhs.w2),
            z3: (self.z1 * rhs.x3) + (self.z2 * rhs.y3) + (self.z3 * rhs.z3) + (self.z4 * rhs.w3),
            z4: (self.z1 * rhs.x4) + (self.z2 * rhs.y4) + (self.z3 * rhs.z4) + (self.z4 * rhs.w4),
            w1: (self.w1 * rhs.x1) + (self.w2 * rhs.y1) + (self.w3 * rhs.z1) + (self.w4 * rhs.w1),
            w2: (self.w1 * rhs.x2) + (self.w2 * rhs.y2) + (self.w3 * rhs.z2) + (self.w4 * rhs.w2),
            w3: (self.w1 * rhs.x3) + (self.w2 * rhs.y3) + (self.w3 * rhs.z3) + (self.w4 * rhs.w3),
            w4: (self.w1 * rhs.x4) + (self.w2 * rhs.y4) + (self.w3 * rhs.z4) + (self.w4 * rhs.w4),
        }
    }
}
impl std::ops::Mul<Vector4> for Matrix4 {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            x: (self.x1 * rhs.x) + (self.x2 * rhs.y) + (self.x3 * rhs.z) + (self.x4 * rhs.w),
            y: (self.y1 * rhs.x) + (self.y2 * rhs.y) + (self.y3 * rhs.z) + (self.y4 * rhs.w),
            z: (self.z1 * rhs.x) + (self.z2 * rhs.y) + (self.z3 * rhs.z) + (self.z4 * rhs.w),
            w: (self.w1 * rhs.x) + (self.w2 * rhs.y) + (self.w3 * rhs.z) + (self.w4 * rhs.w),
        }
    }
}
impl std::ops::Mul<&Vector4> for &Matrix4 {
    type Output = Vector4;
    fn mul(self, rhs: &Vector4) -> Self::Output {
        Vector4 {
            x: (self.x1 * rhs.x) + (self.x2 * rhs.y) + (self.x3 * rhs.z) + (self.x4 * rhs.w),
            y: (self.y1 * rhs.x) + (self.y2 * rhs.y) + (self.y3 * rhs.z) + (self.y4 * rhs.w),
            z: (self.z1 * rhs.x) + (self.z2 * rhs.y) + (self.z3 * rhs.z) + (self.z4 * rhs.w),
            w: (self.w1 * rhs.x) + (self.w2 * rhs.y) + (self.w3 * rhs.z) + (self.w4 * rhs.w),
        }
    }
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
