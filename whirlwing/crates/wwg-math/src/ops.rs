macro_rules! impl_op_with_refs {
    (
        $trait:ident for $type: ty {
            fn $method:ident($self: ident, $rhs: ident) -> $output:ty
                $body:block
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
        $trait:ident<$rhs_type:ty> for $type:ty {
            fn $method:ident($self:ident, $rhs:ident) -> $output:ty
                $body:block
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

macro_rules! impl_op_assign_with_refs {
    (
        $trait:ident for $type: ty {
            fn $method:ident($self: ident, $rhs: ident)
                $body:block
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
        $trait:ident<$rhs_type:ty> for $type: ty {
            fn $method:ident($self: ident, $rhs: ident)
                $body:block
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

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use crate::vec::*;

impl_op_with_refs!(Add for Vector3 {
    fn add(self, rhs) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
});
impl_op_assign_with_refs!(AddAssign for Vector3 {
    fn add_assign(self, rhs) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
});
impl_op_with_refs!(Sub for Vector3 {
    fn sub(self, rhs) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
});
impl_op_assign_with_refs!(SubAssign for Vector3 {
    fn sub_assign(self, rhs) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
});
impl_op_with_refs!(Mul<f32> for Vector3 {
    fn mul(self, rhs) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
});
impl_op_assign_with_refs!(MulAssign<f32> for Vector3 {
    fn mul_assign(self, rhs) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
});
impl_op_with_refs!(Div<f32> for Vector3 {
    fn div(self, rhs) -> Vector3 {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
});
impl_op_assign_with_refs!(DivAssign<f32> for Vector3 {
    fn div_assign(self, rhs) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
});

impl_op_with_refs!(Add for Vector4 {
    fn add(self, rhs) -> Vector4 {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
});
impl_op_assign_with_refs!(AddAssign for Vector4 {
    fn add_assign(self, rhs) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
});
impl_op_with_refs!(Sub for Vector4 {
    fn sub(self, rhs) -> Vector4 {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
});
impl_op_assign_with_refs!(SubAssign for Vector4 {
    fn sub_assign(self, rhs) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
});
impl_op_with_refs!(Mul<f32> for Vector4 {
    fn mul(self, rhs) -> Vector4 {
        Vector4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
});
impl_op_assign_with_refs!(MulAssign<f32> for Vector4 {
    fn mul_assign(self, rhs) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
});
impl_op_with_refs!(Div<f32> for Vector4 {
    fn div(self, rhs) -> Vector4 {
        Vector4 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
});
impl_op_assign_with_refs!(DivAssign<f32> for Vector4 {
    fn div_assign(self, rhs) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
});