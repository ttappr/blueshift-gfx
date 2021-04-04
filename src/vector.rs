
use js_sys::Math::{sin, cos, sqrt};

use crate::matrix::Mat4;

pub const VEC3_BLANK: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
pub const VEC4_BLANK: Vec4 = Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub (crate) x: f32,
    pub (crate) y: f32,
    pub (crate) z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
    pub fn new_zeroed() -> Self {
        VEC3_BLANK
    }
    pub fn diff(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z
        }
    }
    pub fn add(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }
    pub fn normalize(&mut self) -> f32 {
        let len = self.length();
        if len != 0.0 {
            let m = 1.0 / len;
            self.x = self.x * m;
            self.y = self.y * m;
            self.z = self.z * m;
        }
        len
    }
    pub fn dot(&self) -> f32 {
        self.x * self.x +
        self.y * self.y +
        self.z * self.z
    }
    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - v.y * self.z,
            y: self.z * v.x - v.z * self.x,
            z: self.x * v.y - v.x * self.y
        }
    }
    pub fn dot_vec3(&self, v: &Vec3) -> f32 {
        self.x * v.x + 
        self.y * v.y +
        self.z * v.z
    }
    pub fn length(&self) -> f32 {
        // TODO - Find out if there are 32 bit options in the JS webasm
        //        math API. Check to see how these casts impacts performance.
        sqrt(self.dot() as f64) as f32
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        VEC3_BLANK
    }
}

impl From<Vec4> for Vec3 {
    fn from(m: Vec4) -> Self {
        Vec3 { x: m.x, y: m.y, z: m.z }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub (crate) x: f32,
    pub (crate) y: f32,
    pub (crate) z: f32,
    pub (crate) w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 { x, y, z, w }
    }
    pub fn new_zeroed() -> Vec4 {
        VEC4_BLANK
    }
    pub fn multiply_mat4(&self, m: &Mat4) -> Vec4 {
        // TODO - Which is better, `m.m` or `&m.m`?
        let [m0, m1, m2, m3] = &m.m;
        Vec4 {
            x: (self.x * m0.x + self.y * m1.x + self.z * m2.x + self.w * m3.x),
            y: (self.x * m0.y + self.y * m1.y + self.z * m2.y + self.w * m3.y),
            z: (self.x * m0.z + self.y * m1.z + self.z * m2.z + self.w * m3.z),
            w: (self.x * m0.w + self.y * m1.w + self.z * m2.w + self.w * m3.w)
        }
    }
}

impl Default for Vec4 {
    fn default() -> Self {
        VEC4_BLANK
    }
}



