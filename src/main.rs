use anyhow::{bail, Result};
use clap::Parser;
use inquire::Confirm;
use las::{point::Classification, raw::point::Waveform, Builder, Color, Point, Writer};
use rand::{prelude::*, Rng};
use std::fs::File;
use std::path::PathBuf;
use tqdm::tqdm;

#[derive(Clone, Debug, Parser)]
struct Args {
    /// Number of points in cloud
    #[arg(short, long, default_value_t = 10000)]
    num_points: usize,
    /// LAS Point Format (0 - 10)
    #[arg(short, long, default_value_t = 0)]
    format: u8,
    /// W-E extent (meters)
    #[arg(short, long, num_args = 2)]
    x: Vec<f64>,
    /// S-N extent (meters)
    #[arg(short, long, num_args = 2)]
    y: Vec<f64>,
    /// Z extent (meters)
    #[arg(short, long, num_args = 2)]
    z: Vec<f64>,
    /// Output filename. Format is determined by extension.
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let output_string = args.output.clone();
    let output = PathBuf::from(args.output);
    if output.exists() {
        let ans = Confirm::new("Overwrite existing file?")
            .with_default(false)
            .prompt();
        match ans {
            Ok(false) => bail!("Could not overwrite {}. Exiting!", output_string),
            Ok(true) => println!("Overwriting..."),
            Err(_) => bail!("Prompt error... try again later"),
        }
    }

    let n = if args.num_points != 0 {
        args.num_points
    } else {
        println!("Cannot have 0 points! Defaulting to 10000");
        10000
    };

    let (xmin, xmax) = if args.x.len() == 2 {
        let (x1, x2) = (args.x[0], args.x[1]);
        (x1.min(x2), x1.max(x2))
    } else {
        (0.0, 100.0)
    };

    let (ymin, ymax) = if args.y.len() == 2 {
        let (y1, y2) = (args.y[0], args.y[1]);
        (y1.min(y2), y1.max(y2))
    } else {
        (0.0, 100.0)
    };

    let (zmin, zmax) = if args.z.len() == 2 {
        let (z1, z2) = (args.z[0], args.z[1]);
        (z1.min(z2), z1.max(z2))
    } else {
        (0.0, 100.0)
    };

    let format = match args.format {
        0..=10 => args.format,
        _ => {
            println!("Not a valid format: {}. Defaulting to 1", args.format);
            1
        }
    };

    let mut builder = Builder::from((1, 4));
    builder.point_format = las::point::Format::new(format)?;
    let has_gps = builder.point_format.has_gps_time;
    let has_color = builder.point_format.has_color;
    let has_nir = builder.point_format.has_nir;
    let has_waveform = builder.point_format.has_waveform;
    let num_extra_bytes = builder.point_format.extra_bytes;
    let header = builder.into_header()?;

    let f = File::create(output)?;
    let mut writer = Writer::new(f, header)?;

    let mut rng = rand::rng();

    let desc = Some("Generating random points");

    for i in tqdm(0..n).desc(desc) {
        let x = rng.random_range(xmin..=xmax);
        let y = rng.random_range(ymin..=ymax);
        let z = rng.random_range(zmin..=zmax);
        let intensity = rng.random();
        let return_number = match format {
            0..=5 => rng.random_range(0..8),
            6..=10 => rng.random_range(0..16),
            _ => unreachable!(),
        };
        let number_of_returns = match format {
            0..=5 => rng.random_range(return_number..8),
            6..=10 => rng.random_range(return_number..16),
            _ => unreachable!(),
        };
        let scan_direction = match rng.random_bool(0.5) {
            true => las::point::ScanDirection::RightToLeft,
            false => las::point::ScanDirection::LeftToRight,
        };
        let is_edge_of_flight_line = rng.random_bool(0.001);
        let classification = match i % 10 != 0 {
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
