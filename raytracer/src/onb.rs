use crate::vec3::{cross, Vec3};
use std::ops::{Index, IndexMut};

pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn new(w: &Vec3) -> Self {
        let unit_w = w.unit();
        let a = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = cross(&unit_w, &a).unit();
        let u = cross(&unit_w, &v);
        let mut axis = [Vec3::new(1.0, 1.0, 1.0); 3];
        axis[0] = u;
        axis[1] = v;
        axis[2] = unit_w;
        Self { axis }
    }
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }
    pub fn local(&self, a: &Vec3) -> Vec3 {
        self.u() * a.x + self.v() * a.y + self.w() * a.z
    }
}

impl Index<usize> for Onb {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Vec3 {
        match index {
            0 => &self.axis[0],
            1 => &self.axis[1],
            2 => &self.axis[2],
            _ => panic!("Index out of range"),
        }
    }
}

impl IndexMut<usize> for Onb {
    fn index_mut(&mut self, index: usize) -> &mut Vec3 {
        match index {
            0 => &mut self.axis[0],
            1 => &mut self.axis[1],
            2 => &mut self.axis[2],
            _ => panic!("Index out of range"),
        }
    }
}
