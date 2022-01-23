use crate::camera::Camera;

mod camera;
mod image;
mod builder;

pub type Vec3 = nalgebra::base::Vector3<f64>;

fn normal_color(v: Vec3) -> Vec3 {
    (v + Vec3::new(1.0, 1.0, 1.0)) * 0.5
}

mod random {
    use crate::Vec3;
    use rand::{thread_rng, Rng};

    pub fn f64() -> f64 {
        thread_rng().gen()
    }

    pub fn vec() -> Vec3 {
        let mut rng = thread_rng();
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn unit_sphere() -> Vec3 {
        loop {
            let r = vec();
            if r.magnitude_squared() <= 1.0 {
                break r;
            }
        }
    }
}

const INF: f64 = f64::MAX;
const PI: f64 = 3.1415926535897932385;

#[derive(Debug)]
pub enum Geometry {
    Sphere(Vec3, f64),
}

impl Geometry {
    fn hit<'s>(&'s self, ray: &Ray, material: &'s Material) -> Option<Hit<'s>> {
        match self {
            Self::Sphere(center, radius) => {
                let oc = ray.origin - center;
                let a = ray.dir.magnitude_squared();
                let half_b = oc.dot(&ray.dir);
                let c = oc.magnitude_squared() - radius * radius;
                let discriminant = half_b * half_b - a * c;
                if discriminant >= 0.0 {
                    let t = (-half_b - discriminant.sqrt()) / a;
                    if t > 0.0 {
                        let point = ray.at(t);
                        let normal = center - point;
                        let front = ray.dir.dot(&normal) < 0.0;
                        let normal = if front { normal } else { -normal };
                        Some(Hit {
                            shape: self,
                            front,
                            normal,
                            point,
                            material,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }

    fn dist(&self, hit: &Hit) -> usize {
        ((self.origin - hit.point).magnitude() * 1000.0) as usize
    }
}

#[derive(Debug)]
struct Hit<'s> {
    shape: &'s Geometry,
    material: &'s Material,
    point: Vec3,
    normal: Vec3,
    front: bool,
}

impl<'s> Hit<'s> {
    fn ray(&self) -> Ray {
        Ray {
            origin: self.point,
            dir: self.normal,
        }
    }

    fn bounce(&self) -> Ray {
        let origin = self.point;
        Ray {
            dir: origin - self.point + self.normal + random::unit_sphere(),
            origin,
        }
    }
}

#[derive(Debug)]
pub enum Material {
    Diffuse(Vec3),
}

impl Material {
    fn color(&self) -> &Vec3 {
        match self {
            Self::Diffuse(color) => color,
        }
    }

    fn bounce(&self) -> f64 {
        match self {
            Self::Diffuse(_) => 0.5,
        }
    }
}

#[derive(Debug)]
pub struct Scene {
    shapes: Vec<(Geometry, Material)>,
}

impl Scene {
    pub fn new() -> Self {
        Self { shapes: vec![] }
    }

    pub fn add_shape(&mut self, shape: Geometry, material: Material) {
        self.shapes.push((shape, material));
    }

    pub fn ray_cast(&self, ray: &Ray, bounces: usize) -> Vec3 {
        if bounces == 0 {
            Vec3::new(0.0, 1.0, 0.0)
        } else if let Some(hit) = self
            .shapes
            .iter()
            .filter_map(|(s, m)| s.hit(&ray, m))
            .min_by_key(|h| ray.dist(h))
        {
            //normal_color(normal)
            //hit.material.color().clone()
            //println!("{:?}", hit);
            hit.material.bounce() * self.ray_cast(&hit.bounce(), bounces - 1)
            //Self::background(&hit.ray())
        } else {
            Self::background(&ray)
        }
    }

    fn background(ray: &Ray) -> Vec3 {
        let unit_direction = ray.dir.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * (Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0))
    }
}

fn main() {
    let scene = {
        let mut scene = Scene::new();
        scene.add_shape(
            Geometry::Sphere(Vec3::new(0.0, 0.0, 2.0), 0.5),
            Material::Diffuse(Vec3::new(1.0, 0.0, 0.0)),
        );
        scene.add_shape(
            Geometry::Sphere(Vec3::new(0.9, 0.0, 2.0), 0.5),
            Material::Diffuse(Vec3::new(1.0, 0.0, 0.0)),
        );
        scene.add_shape(
            Geometry::Sphere(Vec3::new(-0.9, 0.0, 2.0), 0.5),
            Material::Diffuse(Vec3::new(1.0, 0.0, 0.0)),
        );
        scene
    };

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: usize = 400;
    const HEIGHT: usize = ((WIDTH as f64) / ASPECT_RATIO) as usize;

    let mut image = image::Image::default_file(WIDTH, HEIGHT);
    let camera = Camera::builder()
        .samples(20)
        .build(&image);

    const MAX_BOUNCES: usize = 4;

    //let ray = Ray {
    //origin: Vec3::new(0.0, 0.0, 0.0),
    //dir: Vec3::new(0.0, 0.0, 1.0),
    //};
    //scene.ray_cast(&ray, MAX_BOUNCES);
    //todo!();

    while let Some(mut pixel) = image.next() {
        for ray in camera.pixel_rays(&pixel) {
            pixel.add_sample(scene.ray_cast(&ray, MAX_BOUNCES));
        }
    }

    println!("Time: {} ms", image.elapsed().as_millis());
}
