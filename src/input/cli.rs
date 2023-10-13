use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Number of direct samples per pixel
    #[arg(short, long, default_value_t = 50)]
    pub(crate) samples: i32,

    // Number of indirect bounces per direct ray
    #[arg(short, long, default_value_t = 20)]
    pub(crate) diffuse_bounces: i32,

    // Image height in pixels
    #[arg(short = 'e', long, default_value_t = 1080)]
    pub(crate) height: usize,

    // Image width in pixels
    #[arg(short, long, default_value_t = 1920)]
    pub(crate) width: usize,
}
