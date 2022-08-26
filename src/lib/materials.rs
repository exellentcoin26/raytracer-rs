use super::{traits::Material, Color, HitRecord, Ray, Vec3};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hitrecord: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = hitrecord.normal() + Vec3::random_unit_vector();

        // check that the `random_unit_vector` is not _almost_ the opposite of the hit normal
        let scatter_direction = match scatter_direction.near_zero() {
            true => hitrecord.normal(),
            false => scatter_direction,
        };

        Some((
            self.albedo,
            Ray::new(hitrecord.get_inpact_point(), scatter_direction),
        ))
    }
}

pub struct Metal {
    /// Color of the material
    albedo: Color,
    /// Radius of the circle used to randomize ray bounce
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hitrecord: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().unit_vector().reflect(&hitrecord.normal());
        let scattered = Ray::new(
            hitrecord.get_inpact_point(),
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );

        if scattered.direction().dot(hitrecord.normal()) > 0.0 {
            return Some((self.albedo, scattered));
        }

        None
    }
}
