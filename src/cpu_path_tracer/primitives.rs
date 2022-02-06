pub mod vec3 {
    use std::fmt;

    #[derive(Debug, Copy, Clone)]
    pub struct Vec3 {
        x: f32,
        y: f32,
        z: f32,
    }

    impl fmt::Display for Vec3 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
        }
    }

    impl Vec3 {
        pub fn zero() -> Vec3 {
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }

        pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
            Vec3 { x: x, y: y, z: z }
        }

        pub fn x(self) -> f32 {
            return self.x;
        }

        pub fn y(self) -> f32 {
            return self.y;
        }

        pub fn z(self) -> f32 {
            return self.z;
        }

        pub fn r(self) -> f32 {
            return self.x;
        }

        pub fn g(self) -> f32 {
            return self.y;
        }

        pub fn b(self) -> f32 {
            return self.z;
        }

        pub fn squared_length(self) -> f32 {
            return self.x * self.x + self.y * self.y + self.z * self.z;
        }

        pub fn length(self) -> f32 {
            return self.squared_length().sqrt();
        }

        pub fn dot(a: Vec3, b: Vec3) -> f32 {
            return a.x * b.x + a.y * b.y + a.z * b.z;
        }

        pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
            return Vec3::new(
                a.y * b.z - a.z * b.y,
                -(a.x * b.z - a.z * b.x),
                a.x * b.y - a.y * b.x,
            );
        }

        pub fn unit_vector(self) -> Vec3 {
            return self / self.length();
        }

        pub fn sqrt(self) -> Vec3 {
            return Vec3::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt());
        }
    }

    use std::ops;
    impl ops::Add<Vec3> for Vec3 {
        type Output = Vec3;

        fn add(self, other: Vec3) -> Vec3 {
            Vec3 {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    impl ops::Sub<Vec3> for Vec3 {
        type Output = Vec3;

        fn sub(self, other: Vec3) -> Vec3 {
            Vec3 {
                x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z,
            }
        }
    }

    impl ops::Mul<Vec3> for Vec3 {
        type Output = Vec3;

        fn mul(self, other: Vec3) -> Vec3 {
            Vec3 {
                x: self.x * other.x,
                y: self.y * other.y,
                z: self.z * other.z,
            }
        }
    }

    impl ops::Mul<f32> for Vec3 {
        type Output = Vec3;

        fn mul(self, other: f32) -> Vec3 {
            Vec3 {
                x: self.x * other,
                y: self.y * other,
                z: self.z * other,
            }
        }
    }

    impl ops::Mul<Vec3> for f32 {
        type Output = Vec3;

        fn mul(self, other: Vec3) -> Vec3 {
            Vec3 {
                x: other.x() * self,
                y: other.y() * self,
                z: other.z() * self,
            }
        }
    }

    impl ops::Div<f32> for Vec3 {
        type Output = Vec3;

        fn div(self, other: f32) -> Vec3 {
            Vec3 {
                x: self.x / other,
                y: self.y / other,
                z: self.z / other,
            }
        }
    }

    impl ops::Div<Vec3> for f32 {
        type Output = Vec3;

        fn div(self, other: Vec3) -> Vec3 {
            Vec3 {
                x: other.x() / self,
                y: other.y() / self,
                z: other.z() / self,
            }
        }
    }
}

#[cfg(test)]
mod vec3_tests {
    use crate::cpu_path_tracer::primitives::vec3::*;
    // use super::*;

    #[test]
    fn test_length() {
        let a: Vec3 = Vec3::new(2.0, -3.0, -1.2);
        assert_eq!(a.squared_length(), 14.440001);
        assert_eq!(a.length(), 3.8);
    }

    #[test]
    fn test_dot() {
        let a: Vec3 = Vec3::new(1.0, 1.0, 1.0);
        let b: Vec3 = Vec3::new(2.0, -3.0, -0.2);
        assert_eq!(Vec3::dot(a, a), 3.0);
        assert_eq!(Vec3::dot(b, b), 13.04);
        assert_eq!(Vec3::dot(a, b), -1.2);
        assert_eq!(Vec3::dot(b, a), Vec3::dot(b, a));
    }

    #[test]
    fn test_cross() {
        let a: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let b: Vec3 = Vec3::new(-3.0, -2.0, 1.0);
        assert_eq!(Vec3::cross(a, a).x(), 0.0);
        assert_eq!(Vec3::cross(b, b).y(), 0.0);
        assert_eq!(Vec3::cross(a, b).x(), 8.0);
        assert_eq!(Vec3::cross(b, a).z(), -4.0);
    }

    #[test]
    fn test_operations() {
        let a: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let b: Vec3 = Vec3::new(-3.0, -2.0, 1.0);
        assert_eq!((a + b).x(), -2.0);
        assert_eq!((a - b).y(), 4.0);
        assert_eq!((a * 3.0).x(), 3.0);
        assert_eq!((a / 2.0).y(), 1.0);
    }
}
