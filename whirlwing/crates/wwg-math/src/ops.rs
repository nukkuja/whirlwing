macro_rules! impl_op {
    (
        $trait: ident for $type: ty {
            fn $method: ident($self: ident, $rhs: ident) -> $output: ty
                $body: block
        }
    ) => {
        impl $trait for $type {
            type Output = $output;
            #[inline]
            fn $method($self, $rhs: Self) -> $output {
                $body
            }
        }
        impl $trait<&$type> for $type {
            type Output = $output;
            #[inline]
            fn $method($self, $rhs: &$type) -> $output {
                $body
            }
        }
        impl $trait<$type> for &$type {
            type Output = $output;
            #[inline]
            fn $method($self, $rhs: $type) -> $output {
                $body
            }
        }
        impl $trait for &$type {
            type Output = $output;
            #[inline]
            fn $method($self, $rhs: &$type) -> $output {
                $body
            }
        }
    };
    (
        $trait: ident<$rhs_type: ty> for $type: ty {
            fn $method: ident($self: ident, $rhs: ident) -> $output:ty
                $body: block
        }
    ) => {
        impl $trait<$rhs_type> for $type {
            type Output = $output;
            #[inline]
            fn $method($self, $rhs: $rhs_type) -> $output {
                $body
            }
        }
        impl $trait<&$rhs_type> for $type {
            type Output = $output;
            #[inline]
            fn $method($self, $rhs: &$rhs_type) -> $output {
                $body
            }
        }
        impl $trait<$rhs_type> for &$type {
            type Output = $output;
            #[inline]
            fn $method($self, $rhs: $rhs_type) -> $output {
                $body
            }
        }
        impl $trait<&$rhs_type> for &$type {
            type Output = $output;
            #[inline]
            fn $method($self, $rhs: &$rhs_type) -> $output {
                $body
            }
        }
    }
}

macro_rules! impl_op_assign {
    (
        $trait: ident for $type: ty {
            fn $method: ident($self: ident, $rhs: ident)
                $body: block
        }
    ) => {
        impl $trait for $type {
            #[inline]
            fn $method(&mut $self, $rhs: $type) {
                $body
            }
        }
        impl $trait<&$type> for $type {
            #[inline]
            fn $method(&mut $self, $rhs: &$type) {
                $body
            }
        }
    };
    (
        $trait: ident<$rhs_type: ty> for $type: ty {
            fn $method: ident($self: ident, $rhs: ident)
                $body: block
        }
    ) => {
        impl $trait<$rhs_type> for $type {
            #[inline]
            fn $method(&mut $self, $rhs: $rhs_type) {
                $body
            }
        }
        impl $trait<&$rhs_type> for $type {
            #[inline]
            fn $method(&mut $self, $rhs: &$rhs_type) {
                $body
            }
        }
    }
}
macro_rules! impl_un {
    (
        $trait:ident for $type:ty {
            fn $method:ident ($self:ident) -> $output:ty
                $body:block
        }
    ) => {
        impl $trait for $type {
            type Output = $output;
            #[inline]
            fn $method($self) -> Self::Output {
                $body
            }
        }
        impl $trait for &$type {
            type Output = $output;
            #[inline]
            fn $method($self) -> Self::Output {
                $body
            }
        }
    };
}

use crate::core::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Neg};

// ----- Vector3 -----

impl_op!(Add for Vector3 {
    fn add(self, rhs) -> Vector3 {
        Vector3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
});
impl_op_assign!(AddAssign for Vector3 {
    fn add_assign(self, rhs) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
});
impl_op!(Sub for Vector3 {
    fn sub(self, rhs) -> Vector3 {
        Vector3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
        )
    }
});
impl_op_assign!(SubAssign for Vector3 {
    fn sub_assign(self, rhs) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
});
impl_op!(Mul<f32> for Vector3 {
    fn mul(self, rhs) -> Vector3 {
        Vector3::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
        )
    }
});
impl_op_assign!(MulAssign<f32> for Vector3 {
    fn mul_assign(self, rhs) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
});
impl_op!(Div<f32> for Vector3 {
    fn div(self, rhs) -> Vector3 {
        Vector3::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
        )
    }
});
impl_op_assign!(DivAssign<f32> for Vector3 {
    fn div_assign(self, rhs) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
});
impl_un!(Neg for Vector3 {
    fn neg(self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }
});

// ----- Vector4 -----

impl_op!(Add for Vector4 {
    fn add(self, rhs) -> Vector4 {
        Vector4::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
});
impl_op_assign!(AddAssign for Vector4 {
    fn add_assign(self, rhs) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
});
impl_op!(Sub for Vector4 {
    fn sub(self, rhs) -> Vector4 {
        Vector4::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
});
impl_op_assign!(SubAssign for Vector4 {
    fn sub_assign(self, rhs) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
});
impl_op!(Mul<f32> for Vector4 {
    fn mul(self, rhs) -> Vector4 {
        Vector4::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs,
            self.w * rhs,
        )
    }
});
impl_op_assign!(MulAssign<f32> for Vector4 {
    fn mul_assign(self, rhs) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
});
impl_op!(Div<f32> for Vector4 {
    fn div(self, rhs) -> Vector4 {
        Vector4::new(
            self.x / rhs,
            self.y / rhs,
            self.z / rhs,
            self.w / rhs,
        )
    }
});
impl_op_assign!(DivAssign<f32> for Vector4 {
    fn div_assign(self, rhs) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
});

// ----- Matrix4 -----

impl_op!(Add for Matrix4 {
    fn add(self, rhs) -> Matrix4 {
        Matrix4::new(
            self.x1 + rhs.x1,
            self.x2 + rhs.x2,
            self.x3 + rhs.x3,
            self.x4 + rhs.x4,
            self.y1 + rhs.y1,
            self.y2 + rhs.y2,
            self.y3 + rhs.y3,
            self.y4 + rhs.y4,
            self.z1 + rhs.z1,
            self.z2 + rhs.z2,
            self.z3 + rhs.z3,
            self.z4 + rhs.z4,
            self.w1 + rhs.w1,
            self.w2 + rhs.w2,
            self.w3 + rhs.w3,
            self.w4 + rhs.w4,
        )
    }
});
impl_op_assign!(AddAssign for Matrix4 {
    fn add_assign(self, rhs) {
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
});
impl_op!(Sub for Matrix4 {
    fn sub(self, rhs) -> Matrix4 {
        Matrix4::new(
            self.x1 - rhs.x1,
            self.x2 - rhs.x2,
            self.x3 - rhs.x3,
            self.x4 - rhs.x4,
            self.y1 - rhs.y1,
            self.y2 - rhs.y2,
            self.y3 - rhs.y3,
            self.y4 - rhs.y4,
            self.z1 - rhs.z1,
            self.z2 - rhs.z2,
            self.z3 - rhs.z3,
            self.z4 - rhs.z4,
            self.w1 - rhs.w1,
            self.w2 - rhs.w2,
            self.w3 - rhs.w3,
            self.w4 - rhs.w4,
        )
    }
});
impl_op_assign!(SubAssign for Matrix4 {
    fn sub_assign(self, rhs) {
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
});
impl_op!(Mul for Matrix4 {
    fn mul(self, rhs) -> Matrix4 {
        Matrix4::new(
            (self.x1 * rhs.x1) + (self.x2 * rhs.y1) + (self.x3 * rhs.z1) + (self.x4 * rhs.w1),
            (self.x1 * rhs.x2) + (self.x2 * rhs.y2) + (self.x3 * rhs.z2) + (self.x4 * rhs.w2),
            (self.x1 * rhs.x3) + (self.x2 * rhs.y3) + (self.x3 * rhs.z3) + (self.x4 * rhs.w3),
            (self.x1 * rhs.x4) + (self.x2 * rhs.y4) + (self.x3 * rhs.z4) + (self.x4 * rhs.w4),
            (self.y1 * rhs.x1) + (self.y2 * rhs.y1) + (self.y3 * rhs.z1) + (self.y4 * rhs.w1),
            (self.y1 * rhs.x2) + (self.y2 * rhs.y2) + (self.y3 * rhs.z2) + (self.y4 * rhs.w2),
            (self.y1 * rhs.x3) + (self.y2 * rhs.y3) + (self.y3 * rhs.z3) + (self.y4 * rhs.w3),
            (self.y1 * rhs.x4) + (self.y2 * rhs.y4) + (self.y3 * rhs.z4) + (self.y4 * rhs.w4),
            (self.z1 * rhs.x1) + (self.z2 * rhs.y1) + (self.z3 * rhs.z1) + (self.z4 * rhs.w1),
            (self.z1 * rhs.x2) + (self.z2 * rhs.y2) + (self.z3 * rhs.z2) + (self.z4 * rhs.w2),
            (self.z1 * rhs.x3) + (self.z2 * rhs.y3) + (self.z3 * rhs.z3) + (self.z4 * rhs.w3),
            (self.z1 * rhs.x4) + (self.z2 * rhs.y4) + (self.z3 * rhs.z4) + (self.z4 * rhs.w4),
            (self.w1 * rhs.x1) + (self.w2 * rhs.y1) + (self.w3 * rhs.z1) + (self.w4 * rhs.w1),
            (self.w1 * rhs.x2) + (self.w2 * rhs.y2) + (self.w3 * rhs.z2) + (self.w4 * rhs.w2),
            (self.w1 * rhs.x3) + (self.w2 * rhs.y3) + (self.w3 * rhs.z3) + (self.w4 * rhs.w3),
            (self.w1 * rhs.x4) + (self.w2 * rhs.y4) + (self.w3 * rhs.z4) + (self.w4 * rhs.w4),
        )
    }
});
impl_op_assign!(MulAssign for Matrix4 {
    fn mul_assign(self, rhs) {
        self.x1 = (self.x1 * rhs.x1) + (self.x2 * rhs.y1) + (self.x3 * rhs.z1) + (self.x4 * rhs.w1);
        self.x2 = (self.x1 * rhs.x2) + (self.x2 * rhs.y2) + (self.x3 * rhs.z2) + (self.x4 * rhs.w2);
        self.x3 = (self.x1 * rhs.x3) + (self.x2 * rhs.y3) + (self.x3 * rhs.z3) + (self.x4 * rhs.w3);
        self.x4 = (self.x1 * rhs.x4) + (self.x2 * rhs.y4) + (self.x3 * rhs.z4) + (self.x4 * rhs.w4);
        self.y1 = (self.y1 * rhs.x1) + (self.y2 * rhs.y1) + (self.y3 * rhs.z1) + (self.y4 * rhs.w1);
        self.y2 = (self.y1 * rhs.x2) + (self.y2 * rhs.y2) + (self.y3 * rhs.z2) + (self.y4 * rhs.w2);
        self.y3 = (self.y1 * rhs.x3) + (self.y2 * rhs.y3) + (self.y3 * rhs.z3) + (self.y4 * rhs.w3);
        self.y4 = (self.y1 * rhs.x4) + (self.y2 * rhs.y4) + (self.y3 * rhs.z4) + (self.y4 * rhs.w4);
        self.z1 = (self.z1 * rhs.x1) + (self.z2 * rhs.y1) + (self.z3 * rhs.z1) + (self.z4 * rhs.w1);
        self.z2 = (self.z1 * rhs.x2) + (self.z2 * rhs.y2) + (self.z3 * rhs.z2) + (self.z4 * rhs.w2);
        self.z3 = (self.z1 * rhs.x3) + (self.z2 * rhs.y3) + (self.z3 * rhs.z3) + (self.z4 * rhs.w3);
        self.z4 = (self.z1 * rhs.x4) + (self.z2 * rhs.y4) + (self.z3 * rhs.z4) + (self.z4 * rhs.w4);
        self.w1 = (self.w1 * rhs.x1) + (self.w2 * rhs.y1) + (self.w3 * rhs.z1) + (self.w4 * rhs.w1);
        self.w2 = (self.w1 * rhs.x2) + (self.w2 * rhs.y2) + (self.w3 * rhs.z2) + (self.w4 * rhs.w2);
        self.w3 = (self.w1 * rhs.x3) + (self.w2 * rhs.y3) + (self.w3 * rhs.z3) + (self.w4 * rhs.w3);
        self.w4 = (self.w1 * rhs.x4) + (self.w2 * rhs.y4) + (self.w3 * rhs.z4) + (self.w4 * rhs.w4);
    }
});
impl_op!(Mul<Vector4> for Matrix4 {
    fn mul(self, rhs) -> Vector4 {
        Vector4::new(
            (self.x1 * rhs.x) + (self.x2 * rhs.y) + (self.x3 * rhs.z) + (self.x4 * rhs.w),
            (self.y1 * rhs.x) + (self.y2 * rhs.y) + (self.y3 * rhs.z) + (self.y4 * rhs.w),
            (self.z1 * rhs.x) + (self.z2 * rhs.y) + (self.z3 * rhs.z) + (self.z4 * rhs.w),
            (self.w1 * rhs.x) + (self.w2 * rhs.y) + (self.w3 * rhs.z) + (self.w4 * rhs.w),
        )
    }
});

impl_op!(Mul for Quaternion {
    fn mul(self, rhs) -> Quaternion {
        unsafe {
            Quaternion::new_unchecked(
                (self.w * rhs.w) - (self.i * rhs.i) - (self.j * rhs.j) - (self.k * rhs.k),
                (self.w * rhs.i) + (self.i * rhs.w) - (self.j * rhs.k) + (self.k * rhs.j),
                (self.w * rhs.j) + (self.i * rhs.k) + (self.j * rhs.w) - (self.k * rhs.i),
                (self.w * rhs.k) - (self.i * rhs.j) + (self.j * rhs.i) + (self.k * rhs.w),
            )
        }
    }
});
impl_op!(Mul<Vector3> for Quaternion {
    fn mul(self, rhs) -> Quaternion {
        unsafe {
            Quaternion::new_unchecked(
                - (self.i * rhs.x) - (self.j * rhs.y) - (self.k * rhs.z),
                (self.w * rhs.x) - (self.j * rhs.z) + (self.k * rhs.y),
                (self.w * rhs.y) + (self.i * rhs.z) - (self.k * rhs.x),
                (self.w * rhs.z) - (self.i * rhs.y) + (self.j * rhs.x),
            )
        }
    }
});
impl_op_assign!(MulAssign for Quaternion {
    fn mul_assign(self, rhs) {
        self.w = (self.w * rhs.w) - (self.i * rhs.i) - (self.j * rhs.j) - (self.k * rhs.k);
        self.i = (self.w * rhs.i) + (self.i * rhs.w) - (self.j * rhs.k) + (self.k * rhs.j);
        self.j = (self.w * rhs.j) + (self.i * rhs.k) + (self.j * rhs.w) - (self.k * rhs.i);
        self.k = (self.w * rhs.k) - (self.i * rhs.j) + (self.j * rhs.i) + (self.k * rhs.w);
    }
});