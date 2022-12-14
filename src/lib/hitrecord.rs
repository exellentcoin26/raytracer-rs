use super::{ray::Ray, traits::Material, vec3::Point3, vec3::Vec3};
use std::rc::Rc;

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    mat: Rc<dyn Material>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, r: &Ray, outward_normal: Vec3, mat: Rc<dyn Material>, t: f64) -> Self {
        let front_face = (-r.direction().unit_vector()).dot(outward_normal.unit_vector()) > 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            mat,
            t,
            front_face,
        }
    }

    pub fn get_inpact_point(&self) -> Point3 {
        self.p
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }

    pub fn hit_frontface(&self) -> bool {
        self.front_face
    }

    pub fn mat(&self) -> Rc<dyn Material> {
        self.mat.clone()
    }
}
