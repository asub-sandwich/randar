mod args;
mod lidar;
use anyhow::Result;

fn main() -> Result<()> {
    let lidar = lidar::Lidar::new()?;
    lidar.generate()?;
    Ok(())
}
