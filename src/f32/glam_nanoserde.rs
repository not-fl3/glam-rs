use nanoserde::{DeBin, SerBin};

#[derive(DeBin, SerBin)]
pub struct Vec3Portable {
    x: f32,
    y: f32,
    z: f32,
}

impl Into<Vec3Portable> for &crate::Vec3 {
    fn into(self) -> Vec3Portable {
        Vec3Portable {
            x: self.x(),
            y: self.y(),
            z: self.z(),
        }
    }
}

impl Into<crate::Vec3> for &Vec3Portable {
    fn into(self) -> crate::Vec3 {
        crate::Vec3::new(self.x, self.y, self.z)
    }
}

#[derive(DeBin, SerBin)]
pub struct Vec4Portable {
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

impl Into<Vec4Portable> for &crate::Vec4 {
    fn into(self) -> Vec4Portable {
        Vec4Portable {
            x: self.x(),
            y: self.y(),
            z: self.z(),
            w: self.w()
        }
    }
}

impl Into<crate::Vec4> for &Vec4Portable {
    fn into(self) -> crate::Vec4 {
        crate::Vec4::new(self.x, self.y, self.z, self.w)
    }
}
