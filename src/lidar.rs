use crate::args::parse;
use anyhow::Result;
use las::{point::Classification, raw::point::Waveform, Builder, Color, Point, Writer};
use rand::{prelude::*, Rng};
use std::{fs::File, path::PathBuf};
use tqdm::tqdm;

#[derive(Clone, Debug, Default)]
pub struct Lidar {
    pub output: PathBuf,
    pub file_type: FileType,
    pub num_points: usize,
    pub las_version: u8,
    pub las_format: u8,
    pub ground: f64,
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
        let mut builder = Builder::from((1, self.las_version));
        builder.point_format = las::point::Format::new(self.las_format)?;
        let has_gps = builder.point_format.has_gps_time;
        let has_color = builder.point_format.has_color;
        let has_nir = builder.point_format.has_nir;
        let has_waveform = builder.point_format.has_waveform;
        let num_extra_bytes = builder.point_format.extra_bytes;
        let header = builder.into_header()?;

        let f = File::create(self.output.clone())?;
        let mut writer = Writer::new(f, header)?;

        let mut rng = rand::rng();

        let desc = Some("Generating random points");

        for _ in tqdm(0..self.num_points).desc(desc) {
            let x = rng.random_range(self.xmin..=self.xmax);
            let y = rng.random_range(self.ymin..=self.ymax);
            let z = rng.random_range(self.zmin..=self.zmax);
            let intensity = rng.random();
            let return_number = match self.las_format {
                0..=5 => rng.random_range(0..8),
                6..=10 => rng.random_range(0..16),
                _ => unreachable!(),
            };
            let number_of_returns = match self.las_format {
                0..=5 => rng.random_range(return_number..8),
                6..=10 => rng.random_range(return_number..16),
                _ => unreachable!(),
            };
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
        }
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

#[derive(Clone, Debug, Default)]
pub enum FileType {
    #[default]
    Las,
    Laz,
    Txt,
    Gz,
}
