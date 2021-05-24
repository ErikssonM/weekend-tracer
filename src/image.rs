use ndarray::prelude::*;
use std::io::Result;
use std::{fs::File, io::Write};

use crate::color::Color;

#[derive(Clone)]
pub struct Image {
    pub img: Array2<Color>, //pub img: Vec<Vec<Color>>
}

pub fn merge_samples(images: Vec<Image>) -> Image {
    let s = images.len();
    let ratio = 1.0 / (s as f64);
    let w = images[0].width();
    let h = images[0].height();

    let mut new = Image::new(w, h);

    new.img = images
        .iter()
        .map(|img| &img.img)
        .fold(Array::from_elem((w, h), Color::black()), |acc, i| acc + i);

    new.img.iter_mut().for_each(|c| c.mut_const_mul(ratio));

    new
}

impl Image {
    pub fn new(w: usize, h: usize) -> Self {
        Image {
            img: Array::from_elem((w, h), Color::black()),
        }
    }

    pub fn width(&self) -> usize {
        self.img.shape()[0]
    }

    pub fn height(&self) -> usize {
        self.img.shape()[1]
    }

    pub fn to_ppm_list(&self) -> Vec<String> {
        //self.img.iter()
        //    .map(|c| c.ppm()).collect()
        let mut list = Vec::with_capacity(self.height() * self.width());
        for j in (0..self.height()).rev() {
            for i in 0..self.width() {
                list.push(self.img[(i, j)].ppm());
            }
        }
        list
    }

    pub fn write_ppm(&self, file: &mut File) -> Result<()> {
        let mut rows: Vec<String> = Vec::new();
        rows.push("P3".to_string());
        rows.push(format!("{} {}", self.width(), self.height()));
        rows.push("255".to_string());
        rows.extend(self.to_ppm_list());

        let contents = rows.join("\n");
        file.write_all(contents.as_bytes())?;

        Ok(())
    }
}
