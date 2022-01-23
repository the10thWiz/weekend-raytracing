//
// camera.rs
// Copyright (C) 2022 matthew <matthew@WINDOWS-05HIC4F>
// Distributed under terms of the MIT license.
//

use std::io::Write;

use crate::{
    image::{Image, Pixel},
    Ray, Vec3,
};

use crate::builder;

builder!(pub CameraBuilder => pub Camera {
    builder {
        /// the direction the camera is facing
        forward: Vec3 = Vec3::new(0.0, 0.0, 1.0),
        /// width of the viewport in scene units
        viewport_width: f64 = 2.0,
        /// focal length in scene units
        focal_length: f64 = 1.0,
    }
    shared {
        /// Camera Origin
        origin: Vec3 = Vec3::new(0.0, 0.0, 0.0),
        /// Up direction
        vertical: Vec3 = Vec3::new(0.0, 1.0, 0.0),
        /// Number of samples per pixel
        samples: usize = 4,
    }
    constructor(image: &Image<impl Write>) {
        let viewport_height = viewport_width / (image.width() as f64 / image.height() as f64);
        let horizontal = forward.cross(&vertical) * viewport_width;
        let vertical = vertical * viewport_height;
        let forward = forward * focal_length;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 + forward;
    }
    computed {
        horizontal: Vec3,
        lower_left_corner: Vec3,
        upixel: f64 = 1.0 / (image.width() as f64),
        vpixel: f64 = 1.0 / (image.height() as f64),
    }
});

impl Camera {
    pub fn pixel_rays(&self, pixel: &Pixel<impl Write>) -> RayIter<'_> {
        RayIter {
            camera: self,
            u: pixel.u(),
            v: pixel.v(),
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
                    + self.camera.horizontal * u
                    + self.camera.vertical * v
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
