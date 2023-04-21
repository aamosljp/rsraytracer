use crate::ray::Ray;
use crate::utils::random_float;
use crate::vec3::{dot, Vec3};
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Box<dyn Material>,
}

impl HitRecord {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * dot(v, n) * 2.0
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * -((1.0 - r_out_perp.len_squared()).abs().sqrt());
    r_out_perp + r_out_parallel
}

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
    fn clone(&self) -> Box<dyn Material>;
}

#[derive(Clone)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction = ray.direction().unit_vector();
        let cos_theta = (-dot(unit_direction, rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_float() {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };
        let scattered = Ray::new(rec.p, direction);
        Some((attenuation, scattered))
    }
    fn clone(&self) -> Box<dyn Material> {
        Box::new(Dielectric { ref_idx: self.ref_idx })
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(ray.direction().unit_vector(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        if dot(scattered.direction(), rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
    fn clone(&self) -> Box<dyn Material> {
        Box::new(Metal { albedo: self.albedo, fuzz: self.fuzz})
    }
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            return None;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
    fn clone(&self) -> Box<dyn Material> {
        Box::new(Lambertian::new(self.albedo))
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().len_squared();
        let half_b = dot(oc, ray.direction());
        let c = oc.len_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let t = root;
        let mut rec = HitRecord {
            t,
            p: ray.at(t),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            material: self.material.clone(),
        };
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        Some(rec)
    }
}

pub struct ObjectList {
    objects: Vec<Box<dyn Object>>,
}

impl ObjectList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn push(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }
    pub fn get(&self, index: usize) -> Option<&Box<dyn Object>> {
        self.objects.get(index)
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Object for ObjectList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }
        hit_anything
    }
}
