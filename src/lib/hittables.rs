use super::{traits::Hittable, HitRecord, Point3, Ray, Vec3};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = r.direction().dot(oc);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        // find the nearest root inside the acceptable range
        let mut root = (-half_b - discriminant.sqrt()) / a;
        if root < t_min || root > t_max {
            // try both roots if one fails
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        // get point of inpact
        let p = r.at(root);
        Some(HitRecord::new(p, r, (p - self.center) / self.radius, root))
    }
}
