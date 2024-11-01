use std::fs;
use std::io::Write;
use rand;
use crate::Canvas;

pub fn write_ppm_to_file(canvas: &Canvas, filename: &str) {
    let ppm = canvas.to_ppm();
    let mut file = fs::File::create(filename).expect("Can't create file");
    file.write_all(ppm.as_bytes()).expect("error writing");
}

pub fn random() -> f32 {
    rand::random::<f32>()
}