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

/// List of objects that implement the hittable trait
#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(object: Box<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // track the clossest hit point
        let mut closest = t_max;
        let mut rec = None;

        for object in &self.objects {
            if let Some(hitrecord) = object.hit(r, t_min, closest) {
                closest = hitrecord.get_t();
                rec = Some(hitrecord);
            }
        }

        rec
    }
}
