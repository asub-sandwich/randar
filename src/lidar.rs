use crate::args::parse;
use anyhow::Result;
use indicatif::ProgressBar;
use las::{point::Classification, raw::point::Waveform, Builder, Color, Point, Writer};
use noise::{NoiseFn, Perlin};
use rand::{prelude::*, Rng};
use std::{fs::File, path::PathBuf};

#[derive(Clone, Debug, Default)]
pub struct Lidar {
    pub output: PathBuf,
    pub file_type: FileType,
    pub num_points: usize,
    pub las_version: u8,
    pub las_format: u8,
    pub ground: f64,
    pub surface: bool,
    pub hills: Option<u16>,
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
}

impl Lidar {
    pub fn new() -> Result<Self> {
        parse()
    }

    pub fn generate(&self) -> Result<()> {
        match self.file_type {
            FileType::Las | FileType::Laz => self.generate_las(),
            FileType::Txt | FileType::Gz => self.generate_txt(),
        }
    }

    fn generate_las(&self) -> Result<()> {
        let pb = ProgressBar::new(self.num_points as u64);
        let mut builder = Builder::from((1, self.las_version));
        builder.point_format = las::point::Format::new(self.las_format)?;
        builder.point_format.is_compressed = match self.file_type {
            FileType::Las => false,
            FileType::Laz => true,
            _ => unreachable!(),
        };
        let has_gps = builder.point_format.has_gps_time;
        let has_color = builder.point_format.has_color;
        let has_nir = builder.point_format.has_nir;
        let has_waveform = builder.point_format.has_waveform;
        let num_extra_bytes = builder.point_format.extra_bytes;
        let header = builder.into_header()?;

        let f = File::create(self.output.clone())?;
        let mut writer = Writer::new(f, header)?;

        let perlin = Perlin::new(42069);

        let x_range = self.xmax - self.xmin;
        let y_range = self.ymax - self.ymin;
        let min_range = x_range.min(y_range);
        let z_base = (self.zmin + self.zmax) / 2.0;
        let z_var = self.zmax - z_base;
        let mut rng = rand::rng();

        // Hill generation
        let num_hills = if let Some(n) = self.hills {
            n
        } else {
            let max_hills = min_range / 50.0;
            rng.random_range(2..max_hills as u16)
        };

        let hills: Vec<(f64, f64, f64, f64)> = (0..num_hills)
            .map(|_| {
                let hill_x = rng.random_range(self.xmin..self.xmax);
                let hill_y = rng.random_range(self.ymin..self.ymax);
                let height = rng.random_range(5.0..z_var);
                let spread = rng.random_range(30.0..(min_range / 2.0));
                (hill_x, hill_y, height, spread)
            })
            .collect();

        for _ in 0..self.num_points {
            let x = rng.random_range(self.xmin..=self.xmax);
            let y = rng.random_range(self.ymin..=self.ymax);
            let z = if self.surface {
                let noise = perlin.get([x * 0.1 * z_var, y * 0.1 * z_var]);
                let mut zh = z_base + noise + z_var;
                for (hill_x, hill_y, height, spread) in &hills {
                    zh += gaussian_hill(x, y, *hill_x, *hill_y, *height, *spread);
                }
                zh
            } else {
                rng.random_range(self.zmin..=self.zmax)
            };
            let intensity = rng.random();
            let return_number = rng.random_range(0..6);
            let number_of_returns = rng.random_range(return_number..6);
            let scan_direction = match rng.random_bool(0.5) {
                true => las::point::ScanDirection::RightToLeft,
                false => las::point::ScanDirection::LeftToRight,
            };
            let is_edge_of_flight_line = rng.random_bool(0.001);
            let classification = match rng.random_bool(self.ground) {
                true => Classification::Ground,
                false => {
                    if let Some(cls) = class_vec().choose(&mut rng) {
                        *cls
                    } else {
                        Classification::Ground
                    }
                }
            };
            let is_synthetic = rng.random_bool(0.001);
            let is_key_point = rng.random_bool(0.001);
            let is_withheld = rng.random_bool(0.001);
            let is_overlap = rng.random_bool(0.1);
            let scanner_channel = 0;
            let scan_angle = rng.random_range(-90.0..=90.0);
            let user_data = rng.random();
            let point_source_id = 42069;
            let gps_time = match has_gps {
                true => Some(rng.random()),
                false => None,
            };
            let color = match has_color {
                true => Some(Color {
                    red: rng.random(),
                    green: rng.random(),
                    blue: rng.random(),
                }),
                false => None,
            };
            let waveform = match has_waveform {
                true => Some(Waveform {
                    wave_packet_descriptor_index: rng.random(),
                    byte_offset_to_waveform_data: rng.random(),
                    waveform_packet_size_in_bytes: rng.random(),
                    return_point_waveform_location: rng.random(),
                    x_t: rng.random(),
                    y_t: rng.random(),
                    z_t: rng.random(),
                }),
                false => None,
            };
            let nir = match has_nir {
                true => Some(rng.random()),
                false => None,
            };
            let extra_bytes = if num_extra_bytes == 0 {
                vec![]
            } else {
                (0..num_extra_bytes).map(|_| rng.random()).collect()
            };

            let point = Point {
                x,
                y,
                z,
                intensity,
                return_number,
                number_of_returns,
                scan_direction,
                is_edge_of_flight_line,
                classification,
                is_synthetic,
                is_key_point,
                is_withheld,
                is_overlap,
                scanner_channel,
                scan_angle,
                user_data,
                point_source_id,
                gps_time,
                color,
                waveform,
                nir,
                extra_bytes,
            };
            writer.write_point(point)?;
            pb.inc(1);
        }
        pb.finish_and_clear();
        Ok(())
    }

    fn generate_txt(&self) -> Result<()> {
        println!("I have not made this yet");
        Ok(())
    }
}

fn class_vec() -> Vec<Classification> {
    vec![
        Classification::CreatedNeverClassified,
        Classification::Unclassified,
        Classification::Ground,
        Classification::LowVegetation,
        Classification::MediumVegetation,
        Classification::HighVegetation,
        Classification::Building,
        Classification::LowPoint,
        Classification::ModelKeyPoint,
        Classification::Water,
    ]
}

fn gaussian_hill(x: f64, y: f64, hill_x: f64, hill_y: f64, height: f64, spread: f64) -> f64 {
    let dx = x - hill_x;
    let dy = y - hill_y;
    let distance = (dx * dx + dy * dy).sqrt();
    height * (-distance.powi(2) / (2.0 * spread.powi(2))).exp()
}

#[derive(Clone, Debug, Default)]
pub enum FileType {
    #[default]
    Las,
    Laz,
    Txt,
    Gz,
}
