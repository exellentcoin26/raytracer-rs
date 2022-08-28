use super::{Point3, Ray, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
}

impl Camera {
    /// Returns a new instance of the `Camera` struct, with position and direction set to `(0, 0, 0)` looking at the negative z axes.
    ///
    /// Arguments:
    ///
    /// * `vfov`: Vertical angle (in degrees) from the camera origin to the top and bottom of the view port.
    /// * `aspect_ratio`: Ratio of width devided by height of the view port.
    pub fn new(vfov: f64, aspect_ratio: f64) -> Self {
        let theta = f64::to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius: 0.0,
        }
    }

    /// Returns a new instance of the `Camera` struct, with aperture set to `0` and thus everything
    /// is within focus.
    ///
    /// Arguments:
    ///
    /// * `origin`: Origin of the camera.
    /// * `lookat`: Point the camera is looking at.
    /// * `vup`: Vector indication the up acces of the camera (used to specify roll).
    /// * `vfov`: Vertical angle (in degrees) from the camera origin to the top and bottom of the view port.
    /// * `aspect_ratio`: Ratio of width devided by height of the view port.
    pub fn new_positional(
        origin: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = f64::to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (origin - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius: 0.0,
        }
    }

    /// Returns a new instance of the `Camera` struct.
    ///
    /// Arguments:
    ///
    /// * `origin`: Origin of the camera.
    /// * `lookat`: Point the camera is looking at.
    /// * `vup`: Vector indication the up acces of the camera (used to specify roll).
    /// * `vfov`: Vertical angle (in degrees) from the camera origin to the top and bottom of the view port.
    /// * `aspect_ratio`: Ratio of width devided by height of the view port.
    /// * `aperture`: Size of the lens. This is used as a possible offset for the start of a `Ray`,
    /// thus a _larger_ aperture results in more focus blur.
    pub fn new_focusable(
        origin: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = f64::to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (origin - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rc = self.lens_radius * Vec3::random_in_unit_circle();
        let offset = self.horizontal.unit_vector() * rc.x() + self.vertical.unit_vector() * rc.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16_f64 / 9f64;
        let viewport_height = 2_f64;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1_f64;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius: 0.0,
        }
    }
}
