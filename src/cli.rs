use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about = "CLI tool to generate QR codes")]
/// CLI tool to generate QR codes
///
/// Supported output format: Unicode, PNG, SVG
pub struct QR {
    /// Set the output type of QR code
    #[arg(short = 't', long, value_enum, default_value_t = OutputType::Unicode, ignore_case = true)]
    pub output_type: OutputType,

    /// The text you want to encode in the QR
    pub text: Option<String>,

    /// File to write QR code to, if using PNG or SVG format
    #[arg(short, long, value_name = "FILE",
          required_if_eq_any([("output_type", "png"), ("output_type", "svg")]))]
    pub output: Option<String>,
}

#[derive(ValueEnum, Clone)]
pub enum OutputType {
    /// Output QR code to the terminal
    Unicode,
    /// Output QR code to a PNG image
    PNG,
    /// Output QR code to an SVG image
    SVG,
}
