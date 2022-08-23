use super::super::{color::Color, hitrecord::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, r_in: &Ray, hitrecord: &HitRecord) -> Option<(Color, Ray)>;
}
