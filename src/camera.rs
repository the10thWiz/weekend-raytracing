//
// camera.rs
// Copyright (C) 2022 matthew <matthew@WINDOWS-05HIC4F>
// Distributed under terms of the MIT license.
//

use std::io::Write;

use crate::{Vec3, ASPECT_RATIO, Ray, image::Pixel};

pub struct Camera {
    viewport_width: f64,
    viewport_height: f64,
    focal_length: f64,

    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    samples: usize,
    upixel: f64,
    vpixel: f64,
}

impl Camera {
    pub fn new(width: usize, height: usize) -> Self {
        let viewport_width = 2.0;
        let viewport_height = viewport_width / ASPECT_RATIO;
        let focal_length = 1.0;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length);
        Self {
            viewport_width,
            viewport_height,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            samples: 1,
            upixel: 1.0 / (width as f64),
            vpixel: 1.0 / (height as f64),
        }
    }

    pub fn pixel_rays(&self, pixel: &Pixel<impl Write>) -> RayIter<'_> {
        RayIter {
            camera: self,
            u: pixel.u(), v: pixel.v(),
            cur: 0,
        }
    }

    pub fn samples(&self) -> f64 {
        (self.samples * self.samples) as f64
    }
}

pub struct RayIter<'s> {
    camera: &'s Camera,
    u: f64,
    v: f64,
    cur: usize,
}

impl<'s> Iterator for RayIter<'s> {
    type Item = Ray;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.camera.samples * self.camera.samples {
            let u = self.u + self.camera.upixel * (self.cur / self.camera.samples) as f64;
            let v = self.v + self.camera.vpixel * (self.cur % self.camera.samples) as f64;
            let ray = Ray {
                origin: self.camera.origin,
                dir: self.camera.lower_left_corner
                    + self.camera.horizontal*u
                    + self.camera.vertical*v
                    - self.camera.origin,
            };
            self.cur += 1;
            Some(ray)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.camera.samples * self.camera.samples - self.cur;
        (size, Some(size))
    }
}
