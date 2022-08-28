use std::{fmt::Debug, ops::Deref};

// Coordinate (x, y, z)

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f for Coordinate {
    #[inline(always)]
    fn x(self) -> f32 {
        self.x
    }

    #[inline(always)]
    fn y(self) -> f32 {
        self.y
    }

    #[inline(always)]
    fn z(self) -> f32 {
        self.z
    }
}

// Rotation (yaw, pitch)

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AutoFixingRotation(f32);

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Rotation {
    pub fn fix(&mut self) {
        self.yaw %= 360.0;
        if self.yaw < -180.0 {
            self.yaw += 360.0;
        } else if self.yaw > 180.0 {
            self.yaw -= 360.0;
        }

        self.pitch %= 360.0;
        if self.pitch < -180.0 {
            self.pitch += 360.0;
        } else if self.pitch > 180.0 {
            self.pitch -= 360.0;
        }

        debug_assert!(self.yaw >= -180.0);
        debug_assert!(self.yaw <= 180.0);
        debug_assert!(self.pitch >= -180.0);
        debug_assert!(self.pitch <= 180.0);
    }

    pub fn is_diff_u8(self, other: Rotation) -> bool {
        unsafe {
            self.yaw.to_int_unchecked::<u8>() != other.yaw.to_int_unchecked::<u8>()
                || self.pitch.to_int_unchecked::<u8>() != other.pitch.to_int_unchecked::<u8>()
        }
    }
}

// Position (x, y, z, yaw, pitch)

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Position {
    pub coord: Coordinate,
    pub rot: Rotation,
}

impl Vec3f for Position {
    #[inline(always)]
    fn x(self) -> f32 {
        self.coord.x
    }

    #[inline(always)]
    fn y(self) -> f32 {
        self.coord.y
    }

    #[inline(always)]
    fn z(self) -> f32 {
        self.coord.z
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Position")
            .field("x", &self.coord.x)
            .field("y", &self.coord.y)
            .field("z", &self.coord.z)
            .field("yaw", &self.rot.yaw)
            .field("pitch", &self.rot.pitch)
            .finish()
    }
}

// Trait

pub trait Vec3f
where
    Self: Sized + Copy,
{
    fn x(self) -> f32;
    fn y(self) -> f32;
    fn z(self) -> f32;

    fn distance_sq(self, other: impl Vec3f) -> f32 {
        let x = other.x() - self.x();
        let y = other.y() - self.y();
        let z = other.z() - self.z();
        x * x + y * y + z * z
    }

    fn distance(self, other: impl Vec3f) -> f32 {
        self.distance_sq(other).sqrt()
    }
}
