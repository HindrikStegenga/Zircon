use super::*;

macro_rules! define_magnitude_impl {
    ($t:ty, $v2_name:ident, $v3_name:ident, $v4_name:ident) => {
        impl $v2_name {
            pub fn magnitude(&self) -> $t {
                (self.x() * self.x() + self.y() * self.y()).sqrt()
            }
        }

        impl $v3_name {
            pub fn magnitude(&self) -> $t {
                (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
            }
        }
        impl $v4_name {
            pub fn magnitude(&self) -> $t {
                (self.x() * self.x()
                    + self.y() * self.y()
                    + self.z() * self.z()
                    + self.w() * self.w())
                .sqrt()
            }
        }
    };
}

define_magnitude_impl!(f32, Vector2f32, Vector3f32, Vector4f32);
define_magnitude_impl!(f64, Vector2f64, Vector3f64, Vector4f64);
