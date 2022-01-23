//
// image.rs
// Copyright (C) 2022 matthew <matthew@WINDOWS-05HIC4F>
// Distributed under terms of the MIT license.
//

use ouroboros::self_referencing;
use png::{StreamWriter, Writer};

use crate::Vec3;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::{Duration, Instant};

pub struct Image<W: Write + 'static> {
    width: usize,
    height: usize,
    row: usize,
    col: usize,
    start: Instant,
    last: Instant,
    inner: ImageInner<W>
}

#[self_referencing]
pub struct ImageInner<W: Write + 'static> {
    w: Writer<W>,
    #[borrows(mut w)]
    #[covariant]
    writer: StreamWriter<'this, W>,
}

impl Image<BufWriter<File>> {
    pub fn default_file(width: usize, height: usize) -> Self {
        Self::file(width, height, "result.png")
    }

    pub fn file(width: usize, height: usize, file: impl AsRef<Path>) -> Self {
        let file = File::create(file).unwrap();
        let w = BufWriter::new(file);
        Self::writer(width, height, w)
    }
}

impl<W: Write + 'static> Image<W> {
    pub fn writer(width: usize, height: usize, w: W) -> Self {
        let mut encoder = png::Encoder::new(w, width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_trns(vec![0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8]);
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(
            // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let w = encoder.write_header().unwrap();
        //let writer = w.stream_writer().unwrap();
        println!("Wrote Header");
        Image {
            width,
            height,
            row: 0,
            col: 0,
            start: Instant::now(),
            last: Instant::now(),
            inner: ImageInnerBuilder {
                w,
                writer_builder: |w| w.stream_writer().unwrap(),
            }.build()
        }
    }

    pub fn next<'i: 'o, 'o>(&'i mut self) -> Option<Pixel<'o, W>> {
        if self.last.elapsed() > Duration::from_secs(1) {
            let pixel_id = self.row * self.width + self.col;
            let max = self.width * self.height;
            println!("{:.3}%", (pixel_id as f64 / max as f64) * 100.0);
            self.last = Instant::now();
        }
        let u = (self.col as f64) / ((self.width - 1) as f64);
        let v = (self.row as f64) / ((self.height - 1) as f64);
        self.col += 1;
        if self.col >= self.width {
            self.col = 0;
            self.row += 1;
        }
        if self.row >= self.height {
            None
        } else {
            Some(Pixel {
                u,
                v,
                samples: 0,
                color: Vec3::new(0.0, 0.0, 0.0),
                image: &mut self.inner,
            })
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

pub struct Pixel<'i, W: Write + 'static> {
    u: f64,
    v: f64,
    samples: usize,
    color: Vec3,
    image: &'i mut ImageInner<W>,
}

impl<'i, W: Write> Pixel<'i, W> {
    pub fn add_sample(&mut self, color: Vec3) {
        self.samples += 1;
        self.color += color;
    }

    pub fn u(&self) -> f64 {
        self.u
    }
    pub fn v(&self) -> f64 {
        self.v
    }
}

impl<'i, W: Write> Drop for Pixel<'i, W> {
    fn drop(&mut self) {
        self.color /= self.samples as f64;
        self.image.with_writer_mut(|image|
            image.write_all(&[
                (self.color.x * 256.0) as u8,
                (self.color.y * 256.0) as u8,
                (self.color.z * 256.0) as u8,
                255u8,
            ])
            .unwrap());
    }
}
