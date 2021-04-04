

use js_sys::Math::{sin, cos, sqrt};

use crate::types::DEG_TO_RAD;
use crate::vector::*;


pub const MAT3_BLANK : Mat3 = Mat3 { m: [VEC3_BLANK, VEC3_BLANK, VEC3_BLANK] };

pub const MAT4_BLANK : Mat4 = Mat4 { m: [VEC4_BLANK, VEC4_BLANK, 
                                         VEC4_BLANK, VEC4_BLANK] };
pub const MAT3_IDENTITY : Mat3 = 
                Mat3 { m: [Vec3 { x: 1.0, y: 0.0, z: 0.0 },
                           Vec3 { x: 0.0, y: 1.0, z: 0.0 },
                           Vec3 { x: 0.0, y: 0.0, z: 1.0}] };
                       
pub const MAT4_IDENTITY : Mat4 = 
                Mat4 { m: [Vec4 { x: 1.0, y: 0.0, z: 0.0, w: 0.0 },
                           Vec4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 },
                           Vec4 { x: 0.0, y: 0.0, z: 1.0, w: 0.0 },
                           Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }] };

#[derive(Clone, Copy, Debug)]
pub struct Mat3 {
    pub (crate) m: [Vec3; 3],
}

impl Mat3 {
    pub fn new() -> Self {
        MAT3_BLANK
    }
}

impl Default for Mat3 {
    fn default() -> Self {
        MAT3_BLANK
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    pub (crate) m: [Vec4; 4],
}

impl Mat4 {
    pub fn new() -> Self {
        MAT4_BLANK
    }
    pub fn new_identity() -> Self {
        MAT4_IDENTITY
    }
    pub fn identity(&mut self) {
        *self = MAT4_IDENTITY;
    }
    pub fn copy_to(&self, dest: &mut Mat4) {
        dest.m = self.m;
    }
    pub fn multiply(&self, m: &Mat4) -> Self {
        let mut mat = Mat4::new();
        
        let [s0, s1, s2, s3] = self.m;
        let [m0, m1, m2, m3] = m.m;

        mat.m[0].x = s0.x * m0.x + s1.x * m0.y + 
                     s2.x * m0.z + s3.x * m0.w;
        mat.m[0].y = s0.y * m0.x + s1.y * m0.y + 
                     s2.y * m0.z + s3.y * m0.w;
        mat.m[0].z = s0.z * m0.x + s1.z * m0.y + 
                     s2.z * m0.z + s3.z * m0.w;
        mat.m[0].w = s0.w * m0.x + s1.w * m0.y + 
                     s2.w * m0.z + s3.w * m0.w;

        mat.m[1].x = s0.x * m1.x + s1.x * m1.y + 
                     s2.x * m1.z + s3.x * m1.w;
        mat.m[1].y = s0.y * m1.x + s1.y * m1.y + 
                     s2.y * m1.z + s3.y * m1.w;
        mat.m[1].z = s0.z * m1.x + s1.z * m1.y + 
                     s2.z * m1.z + s3.z * m1.w;
        mat.m[1].w = s0.w * m1.x + s1.w * m1.y + 
                     s2.w * m1.z + s3.w * m1.w;

        mat.m[2].x = s0.x * m2.x + s1.x * m2.y + 
                     s2.x * m2.z + s3.x * m2.w;
        mat.m[2].y = s0.y * m2.x + s1.y * m2.y + 
                     s2.y * m2.z + s3.y * m2.w;
        mat.m[2].z = s0.z * m2.x + s1.z * m2.y + 
                     s2.z * m2.z + s3.z * m2.w;
        mat.m[2].w = s0.w * m2.x + s1.w * m2.y + 
                     s2.w * m2.z + s3.w * m2.w;

        mat.m[3].x = s0.x * m3.x + s1.x * m3.y + 
                     s2.x * m3.z + s3.x * m3.w;
        mat.m[3].y = s0.y * m3.x + s1.y * m3.y + 
                     s2.y * m3.z + s3.y * m3.w;
        mat.m[3].z = s0.z * m3.x + s1.z * m3.y + 
                     s2.z * m3.z + s3.z * m3.w;
        mat.m[3].w = s0.w * m3.x + s1.w * m3.y + 
                     s2.w * m3.z + s3.w * m3.w;

        mat
    }
    pub fn translate(&mut self, v: &Vec3) {
        // TODO - Experiment with this copy based solution vs. reference 
        //        oriented solutions. See what code the compiler produces
        //        and time the alternatives with the timeit crate.
        let [m0, m1, m2, m3] = self.m;
        let mm3 = &mut self.m[3];
        
        mm3.x = m0.x * v.x + m1.x * v.y + m2.x * v.z + m3.x;
        mm3.y = m0.y * v.x + m1.y * v.y + m2.y * v.z + m3.y;
        mm3.z = m0.z * v.x + m1.z * v.y + m2.z * v.z + m3.z;
        mm3.w = m0.w * v.x + m1.w * v.y + m2.w * v.z + m3.w;
    }
    
    pub fn rotate(&mut self, v: &Vec4) {
        if v.w == 0.0 || 
          (v.x == 0.0 && v.y == 0.0 && v.z == 0.0 && v.z == 0.0) 
        { 
            return; 
        }
        // TODO - Check if sin() and cos() are fast, or if there's a faster
        //        lib/crate to use.
        let s = sin((v.w * DEG_TO_RAD) as f64) as f32;
        let c = cos((v.w * DEG_TO_RAD) as f64) as f32;
      
        let xx = v.x * v.x;
        let yy = v.y * v.y;
        let zz = v.z * v.z;
        let xy = v.x * v.y;
        let yz = v.y * v.z;
        let zx = v.z * v.x;
        let xs = v.x * s;
        let ys = v.y * s;
        let zs = v.z * s;
        let c1 = 1.0 - c;

        let mut mat = Mat4::new_identity();

        mat.m[0].x = (c1 * xx) + c;
        mat.m[1].x = (c1 * xy) - zs;
        mat.m[2].x = (c1 * zx) + ys;

        mat.m[0].y = (c1 * xy) + zs;
        mat.m[1].y = (c1 * yy) + c;
        mat.m[2].y = (c1 * yz) - xs;

        mat.m[0].z = (c1 * zx) - ys;
        mat.m[1].z = (c1 * yz) + xs;
        mat.m[2].z = (c1 * zz) + c;
        
        *self = self.multiply(&mat);
    }
    
    pub fn scale(&mut self, v: &Vec3) {
        let [m0, m1, m2, m3] = &mut self.m;

        m0.x *= v.x;
        m0.y *= v.x;
        m0.z *= v.x;
        m0.w *= v.x;

        m1.x *= v.y;
        m1.y *= v.y;
        m1.z *= v.y;
        m1.w *= v.y;

        m2.x *= v.z;
        m2.y *= v.z;
        m2.z *= v.z;
        m2.w *= v.z;
    }
    
    pub fn invert_full(&mut self) -> bool {
        let mut inv = Mat4::new();
        
        let [ m0,  m1,  m2,  m3] = self.m;
        let [im0, im1, im2, im3] = &mut inv.m;

        im0.x =  m1.y * m2.z * m3.w - 
                 m1.y * m2.w * m3.z - 
                 m2.y * m1.z * m3.w +
                 m2.y * m1.w * m3.z + 
                 m3.y * m1.z * m2.w - 
                 m3.y * m1.w * m2.z;
             
        im1.x = -m1.x * m2.z * m3.w +
                 m1.x * m2.w * m3.z +
                 m2.x * m1.z * m3.w -
                 m2.x * m1.w * m3.z -
                 m3.x * m1.z * m2.w +
                 m3.x * m1.w * m2.z;
             
        im2.x =  m1.x * m2.y * m3.w -
                 m1.x * m2.w * m3.y -
                 m2.x * m1.y * m3.w +
                 m2.x * m1.w * m3.y +
                 m3.x * m1.y * m2.w -
                 m3.x * m1.w * m2.y;
             
        im3.x = -m1.x * m2.y * m3.z +
                 m1.x * m2.z * m3.y +
                 m2.x * m1.y * m3.z -
                 m2.x * m1.z * m3.y -
                 m3.x * m1.y * m2.z +
                 m3.x * m1.z * m2.y;
             
        im0.y = -m0.y * m2.z * m3.w +
                 m0.y * m2.w * m3.z +
                 m2.y * m0.z * m3.w -
                 m2.y * m0.w * m3.z -
                 m3.y * m0.z * m2.w +
                 m3.y * m0.w * m2.z;
             
        im1.y =  m0.x * m2.z * m3.w -
                 m0.x * m2.w * m3.z -
                 m2.x * m0.z * m3.w +
                 m2.x * m0.w * m3.z +
                 m3.x * m0.z * m2.w -
                 m3.x * m0.w * m2.z;
             
        im2.y = -m0.x * m2.y * m3.w +
                 m0.x * m2.w * m3.y +
                 m2.x * m0.y * m3.w -
                 m2.x * m0.w * m3.y -
                 m3.x * m0.y * m2.w +
                 m3.x * m0.w * m2.y;
             
        im3.y =  m0.x * m2.y * m3.z - 
                 m0.x * m2.z * m3.y -
                 m2.x * m0.y * m3.z +
                 m2.x * m0.z * m3.y +
                 m3.x * m0.y * m2.z -
                 m3.x * m0.z * m2.y;
             
        im0.z =  m0.y * m1.z * m3.w -
                 m0.y * m1.w * m3.z -
                 m1.y * m0.z * m3.w +
                 m1.y * m0.w * m3.z +
                 m3.y * m0.z * m1.w -
                 m3.y * m0.w * m1.z;
             
        im1.z = -m0.x * m1.z * m3.w +
                 m0.x * m1.w * m3.z +
                 m1.x * m0.z * m3.w -
                 m1.x * m0.w * m3.z -
                 m3.x * m0.z * m1.w +
                 m3.x * m0.w * m1.z;
             
        im2.z =  m0.x * m1.y * m3.w -
                 m0.x * m1.w * m3.y -
                 m1.x * m0.y * m3.w +
                 m1.x * m0.w * m3.y +
                 m3.x * m0.y * m1.w -
                 m3.x * m0.w * m1.y;
             
        im3.z = -m0.x * m1.y * m3.z +
                 m0.x * m1.z * m3.y +
                 m1.x * m0.y * m3.z -
                 m1.x * m0.z * m3.y -
                 m3.x * m0.y * m1.z +
                 m3.x * m0.z * m1.y;
             
        im0.w = -m0.y * m1.z * m2.w +
                 m0.y * m1.w * m2.z +
                 m1.y * m0.z * m2.w -
                 m1.y * m0.w * m2.z -
                 m2.y * m0.z * m1.w +
                 m2.y * m0.w * m1.z;
             
        im1.w =  m0.x * m1.z * m2.w -
                 m0.x * m1.w * m2.z -
                 m1.x * m0.z * m2.w +
                 m1.x * m0.w * m2.z +
                 m2.x * m0.z * m1.w -
                 m2.x * m0.w * m1.z;
             
        im2.w = -m0.x * m1.y * m2.w +
                 m0.x * m1.w * m2.y +
                 m1.x * m0.y * m2.w -
                 m1.x * m0.w * m2.y -
                 m2.x * m0.y * m1.w +
                 m2.x * m0.w * m1.y;
             
        im3.w =  m0.x * m1.y * m2.z -
                 m0.x * m1.z * m2.y -
                 m1.x * m0.y * m2.z +
                 m1.x * m0.z * m2.y +
                 m2.x * m0.y * m1.z -
                 m2.x * m0.z * m1.y;

        let mut d = m0.x * im0.x + 
                    m0.y * im1.x +
                    m0.z * im2.x +
                    m0.w * im3.x;
        
        if d != 0.0 {
            d = 1.0 / d;

            im0.x *= d;
            im0.y *= d;
            im0.z *= d;
            im0.w *= d;

            im1.x *= d;
            im1.y *= d;
            im1.z *= d;
            im1.w *= d;

            im2.x *= d;
            im2.y *= d;
            im2.z *= d;
            im2.w *= d;

            im3.x *= d;
            im3.y *= d;
            im3.z *= d;
            im3.w *= d;
            
            inv.copy_to(self);
        
            true
            
        } else {
            false
        }
    }

    pub fn transpose(&mut self) {
        let [m0, m1, m2, m3] = &mut self.m;
        let mut temp;
        
        temp = m0.y;
        m0.y = m1.x; 
        m1.x = temp;
        
        temp = m0.z; 
        m0.z = m2.x; 
        m2.x = temp;
        
        temp = m0.w; 
        m0.w = m3.x; 
        m3.x = temp;

        temp = m1.z; 
        m1.z = m2.y; 
        m2.y = temp;
        
        temp = m1.w; 
        m1.w = m3.y; 
        m3.y = temp;

        temp = m2.w; 
        m2.w = m3.z; 
        m3.z = temp;    
    }

    pub fn copy_to_mat3(&self, dst: &mut Mat3)
    {   
        let [s0, s1, s2, _] = &self.m;
        let [d0, d1, d2   ] = &mut dst.m;
        
        d0.x = s0.x;
        d0.y = s0.y;
        d0.z = s0.z;

        d1.x = s1.x;
        d1.y = s1.y;
        d1.z = s1.z;
 
        d2.x = s2.x;
        d2.y = s2.y;
        d2.z = s2.z;
    }


    pub fn ortho(&mut self,
                 left       : f32,
                 right      : f32,
                 bottom     : f32,
                 top        : f32,
                 clip_start : f32,
                 clip_end   : f32) 
    {
        // TODO - Should this be an in-place operation like multiply()?
        //        Should other operations be in-place that aren't?

	    let mut mat = Mat4::new();
	    let [m0, m1, m2, m3] = &mut mat.m;

	    m0.x = 2.0 / (right - left );
	    m1.x = 0.0;
	    m2.x = 0.0;
	    m3.x = -(right + left) / (right - left);

	    m0.y = 0.0;
	    m1.y = 2.0 / (top - bottom);
	    m2.y = 0.0;
	    m3.y = -(top + bottom) / (top - bottom);

	    m0.z = 0.0;
	    m1.z = 0.0;
	    m2.z = -2.0 / (clip_end - clip_start);
	    m3.z = -(clip_end + clip_start) / (clip_end - clip_start);

	    m0.w = 0.0;
	    m1.w = 0.0;
	    m2.w = 0.0;
	    m3.w = 1.0;		

        *self = self.multiply(&mat);
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        MAT4_BLANK
    }
}



