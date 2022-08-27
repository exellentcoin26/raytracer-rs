use super::{traits::Material, utils, Color, HitRecord, Ray, Vec3};

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

pub struct Dielectric {
    /// Refraction index of the dielectric material
    ref_index: f64,
}

impl Dielectric {
    pub fn new(ref_index: f64) -> Self {
        assert!(ref_index >= 1.0, "refraction index cannot be lower than 1");
        Self { ref_index }
    }

    /// Returns the ratio that should be reflected using the `Schlick's approximation` (<https://en.wikipedia.org/wiki/Schlick%27s_approximation>)
    fn reflectance(cos: f64, ref_index: f64) -> f64 {
        let r0 = ((1.0 - ref_index) / (1.0 + ref_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hitrecord: &HitRecord) -> Option<(Color, Ray)> {
        // calculate eta_over_etap according to hitting a frontface
        let ref_ratio = if hitrecord.hit_frontface() {
            1.0 / self.ref_index
        } else {
            self.ref_index
        };

        let unit_dir = r_in.direction().unit_vector();

        let cos_theta = (-unit_dir).dot(hitrecord.normal());
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        // check whether to refract or reflect
        let direction = if ref_ratio * sin_theta > 1.0
            || Self::reflectance(cos_theta, ref_ratio) > utils::random_double()
        {
            // reflect
            unit_dir.reflect(&hitrecord.normal())
        } else {
            // refract
            unit_dir.refract(&hitrecord.normal(), ref_ratio)
        };

        Some((
            Color::new(1.0, 1.0, 1.0),
            Ray::new(hitrecord.get_inpact_point(), direction),
        ))
    }
}
