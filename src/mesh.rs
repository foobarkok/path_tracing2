use glam::*;
use obvhs::{Boundable, aabb::Aabb, ray::Ray, triangle::Triangle};

#[derive(Clone, Copy, Debug)]
pub enum Mesh {
    Triangle(Triangle),
    Sphere { center: Vec3A, r: f32 },
}

impl Mesh {
    pub fn intersect_and_normal(&self, ray: &Ray, normal: &mut Vec3A) -> f32 {
        match self {
            Mesh::Triangle(tri) => {
                let dis = tri.intersect(ray);
                if dis > 0.0 {
                    *normal = tri.compute_normal();
                }
                dis
            }
            Mesh::Sphere { center, r } => {
                let oc = center - ray.origin;
                let a = ray.direction.length_squared();
                let h = ray.direction.dot(oc);
                let c = oc.length_squared() - r * r;
                let discriminant = h * h - a * c;
                if discriminant < 0.0 {
                    return -1.0;
                }
                let sqrtd = discriminant.sqrt();
                let mut root = (h - sqrtd) / a;
                if root <= ray.tmin || ray.tmax <= root {
                    root = (h + sqrtd) / a;
                    if root <= ray.tmin || ray.tmax <= root {
                        return -1.0;
                    }
                }

                *normal = (root - center) / r;
                root
            }
        }
    }
}

impl Boundable for Mesh {
    fn aabb(&self) -> obvhs::aabb::Aabb {
        match self {
            Mesh::Triangle(tri) => tri.aabb(),
            Mesh::Sphere { center, r } => {
                let r = r.abs();
                Aabb {
                    min: center - vec3a(r, r, r),
                    max: center + vec3a(r, r, r),
                }
            }
        }
    }
}
