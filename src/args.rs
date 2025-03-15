use crate::lidar::{FileType, Lidar};
use anyhow::{bail, Result};
use clap::Parser;
use inquire::Confirm;
use std::{path::PathBuf, str::FromStr};

#[derive(Clone, Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Number of points in cloud
    #[arg(short, long, default_value_t = 10000)]
    num_points: usize,
    /// LAS Minor Version
    #[arg(short, long, default_value_t = 4)]
    minor: u8,
    /// LAS Point Format (0 - 10)
    #[arg(short, long, default_value_t = 1)]
    format: u8,
    /// Percent (0 - 100) of points classified as Ground
    #[arg(short, long, default_value_t = 90.)]
    ground: f64,
    /// Make Z-values surface-like
    #[arg(short, long, default_value_t = false)]
    surface: bool,
    /// Number of hills, otherwise random. Used with surface
    #[arg(short, long)]
    hills: Option<u16>,
    /// W-E extent (meters)
    #[arg(short, long, num_args = 2, default_values_t = [0.,1000.])]
    x: Vec<f64>,
    /// S-N extent (meters)
    #[arg(short, long, num_args = 2, default_values_t = [0.,1000.])]
    y: Vec<f64>,
    /// Z extent (meters)
    #[arg(short, long, num_args = 2, default_values_t = [0., 100.])]
    z: Vec<f64>,
    /// Output filename. Format is determined by extension.
    output: String,
}

pub fn parse() -> Result<Lidar> {
    let args = Args::parse();

    let output_string = args.output;
    let output = PathBuf::from(output_string.clone());

    if output.exists() {
        let ans = Confirm::new("Overwrite existing file?")
            .with_default(false)
            .prompt();
        match ans {
            Ok(false) => bail!("Could not overwrite {}. Exiting!", output_string.clone()),
            Ok(true) => println!("Overwriting..."),
            Err(_) => bail!("Prompt error... try again later"),
        }
    }

    let file_type = if let Some(ext) = output.extension() {
        FileType::from_str(ext.to_str().unwrap())?
    } else {
        FileType::default()
    };
    let num_points = args.num_points;
    let las_version = match args.minor {
        0..=4 => args.minor,
        _ => {
            println!(
                "Warning! `{}` is not a valid minor las version. Defaulting to minor version 4!",
                args.minor
            );
            4
        }
    };
    let las_format = match args.format {
        0..=10 => args.format,
        _ => {
            println!(
                "Warning! `{}` is not a valid las point format. Defaulting to point format 1!",
                args.format
            );
            1
        }
    };
    let g = args.ground / 100f64;
    let ground = if (0f64..=1f64).contains(&g) {
        g
    } else {
        println!("Warning! Invalid ground percentage. Defaulting to 90%!");
        0.9
    };
    let surface = args.surface;
    let hills = args.hills;
    let (xmin, xmax) = {
        let (x1, x2) = (args.x[0], args.x[1]);
        (x1.min(x2), x1.max(x2))
    };
    let (ymin, ymax) = {
        let (y1, y2) = (args.y[0], args.y[1]);
        (y1.min(y2), y1.max(y2))
    };
    let (zmin, zmax) = {
        let (z1, z2) = (args.z[0], args.z[1]);
        (z1.min(z2), z1.max(z2))
    };

    Ok(Lidar {
        output,
        file_type,
        num_points,
        las_version,
        las_format,
        ground,
        surface,
        hills,
        xmin,
        xmax,
        ymin,
        ymax,
        zmin,
        zmax,
    })
}

impl FromStr for FileType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<FileType> {
        let s = s.to_ascii_lowercase();
        let s = s.as_str();
        match s {
            "las" => Ok(FileType::Las),
            "laz" => Ok(FileType::Laz),
            "txt" | "xyz" => Ok(FileType::Txt),
            "gz" => Ok(FileType::Gz),
            _ => {
                println!("Warning! Could not determine file type from extension: `{}`. Defaulting to LAS!", s);
                Ok(FileType::Las)
            }
        }
    }
}
