//
// image.rs
// Copyright (C) 2022 matthew <matthew@WINDOWS-05HIC4F>
// Distributed under terms of the MIT license.
//

use png::{StreamWriter, Writer};

use crate::Vec3;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct Image<W: Write> {
    w: Writer<W>,
    width: usize,
    height: usize,
    writer: bool,
}

impl Image<BufWriter<File>> {
    pub fn new(width: usize, height: usize) -> Self {
        let file = File::create("result.png").unwrap();
        let w = BufWriter::new(file);

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
        Self {
            w,
            width,
            height,
            writer: false,
        }
    }
}

impl<W: Write> Image<W> {
    pub fn iter<'a>(&'a mut self) -> PixelIter<'a, W> {
        assert!(!self.writer, "Only one writer per image may be created");
        self.writer = true;
        PixelIter {
            width: self.width,
            height: self.height,
            row: 0,
            col: 0,
            start: Instant::now(),
            last: Instant::now(),
            writer: self.writer(),
        }
    }

    fn writer<'a>(&'a mut self) -> StreamWriter<'a, W> {
        self.w.stream_writer().unwrap()
    }
}

pub struct PixelIter<'a, W: Write> {
    width: usize,
    height: usize,
    row: usize,
    col: usize,
    start: Instant,
    last: Instant,
    writer: StreamWriter<'a, W>,
}

impl<'a, W: Write> PixelIter<'a, W> {
    pub fn next<'i>(&'i mut self) -> Option<Pixel<'i, 'a, W>> {
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
                image: &mut self.writer,
            })
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

pub struct Pixel<'i, 'a, W: Write> {
    u: f64,
    v: f64,
    samples: usize,
    color: Vec3,
    image: &'i mut StreamWriter<'a, W>,
}

impl<'i, 'a, W: Write> Pixel<'i, 'a, W> {
    pub fn add_sample(&mut self, color: Vec3) {
        self.samples += 1;
        self.color += color;
    }

    pub fn samples(&self) -> usize {
        self.samples
    }

    pub fn u(&self) -> f64 {
        self.u
    }
    pub fn v(&self) -> f64 {
        self.v
    }
}

impl<'i, 'a, W: Write> Drop for Pixel<'i, 'a, W> {
    fn drop(&mut self) {
        //println!("Writing pixel");
        self.image
            .write_all(&[
                (self.color.x * 256.0) as u8,
                (self.color.y * 256.0) as u8,
                (self.color.z * 256.0) as u8,
                255u8,
            ])
            .unwrap();
    }
}
