use std::fs::File;
use std::io::{BufWriter, Write};
use std::mem::MaybeUninit;
use std::time::{Instant, Duration};

use crate::camera::Camera;

mod camera;
mod image;

pub type Vec3 = nalgebra::base::Vector3<f64>;

fn normal_color(v: Vec3) -> Vec3 {
    (v + Vec3::new(1.0, 1.0, 1.0)) * 0.5
}

const INF: f64 = f64::MAX;
const PI: f64 = 3.1415926535897932385;

pub enum Geometry {
    Sphere(Vec3, f64),
}

impl Geometry {
    fn hit<'s>(&'s self, ray: &Ray) -> Option<Hit<'s>> {
        match self {
            Self::Sphere(center, radius) => {
                let oc = ray.origin - center;
                let a = ray.dir.magnitude_squared();
                let half_b = oc.dot(&ray.dir);
                let c = oc.magnitude_squared() - radius*radius;
                let discriminant = half_b*half_b - a*c;
                if discriminant > 0.0 {
                    let point = ray.at((-half_b - discriminant.sqrt()) / a);
                    let normal = point - center;
                    let front = ray.dir.dot(&normal) < 0.0;
                    let normal = if front {
                        normal
                    } else {
                        -normal
                    };
                    Some(Hit {
                        shape: self,
                        front,
                        normal,
                        point,
                    })
                } else {
                    None
                }
            }
        }
    }
}

pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        self.origin + self.dir*t
    }

    fn dist(&self, hit: &Hit) -> usize {
        ((self.origin - hit.point).magnitude() * 1000.0) as usize
    }
}

struct Hit<'s> {
    shape: &'s Geometry,
    point: Vec3,
    normal: Vec3,
    front: bool,
}

fn ray_color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.dir.normalize();
    let t = 0.5*(unit_direction.y + 1.0);
    (1.0-t)*(Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0))
}

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const WIDTH: usize = 400;
pub const HEIGHT: usize = ((WIDTH as f64) / ASPECT_RATIO) as usize;

pub struct Material {

}

fn main() {
    let scene = vec![
        Geometry::Sphere(Vec3::new(0.0, 0.0, -2.0), 0.5),
        Geometry::Sphere(Vec3::new(1.0, 0.0, -2.0), 0.5),
        Geometry::Sphere(Vec3::new(-1.0, 0.0, -2.0), 0.5),
    ];

    let camera = Camera::new(WIDTH, HEIGHT);

    println!("init");
    let mut image = image::Image::new(WIDTH, HEIGHT);
    let mut image = image.iter();
    println!("created image");

    while let Some(mut pixel) = image.next() {
        for ray in camera.pixel_rays(&pixel) {
            if let Some(Hit { normal, .. }) = scene.iter()
                .filter_map(|s| s.hit(&ray))
                .min_by_key(|h| ray.dist(h))
            {
                pixel.add_sample(normal_color(normal));
            } else {
                pixel.add_sample(ray_color(&ray));
            }
        }
    }

    println!("Time: {} ms", image.elapsed().as_millis());
}
