use std::io::stdin;

use fast_qr::{
    convert::{image::ImageBuilder, svg::SvgBuilder, Builder, Shape},
    QRBuilder, ECL,
};
use miette::{IntoDiagnostic, Result};

mod cli;
use cli::{OutputType, QR};

use clap::Parser;

fn main() -> Result<()> {
    let qr_args = QR::parse();
    generate_qr(qr_args)?;

    Ok(())
}

fn generate_qr(qr_args: QR) -> Result<()> {
    let mut text = String::new();
    match qr_args.text {
        None => {
            stdin().read_line(&mut text).into_diagnostic()?;
        }
        Some(x) => {
            if x.len() == 1 && x.chars().next().unwrap() == '-' {
                stdin().read_line(&mut text).into_diagnostic()?;
            } else {
                text = x;
            }
        }
    }

    let qr_code = QRBuilder::new(text).ecl(ECL::H).build().into_diagnostic()?;
    match qr_args.output_type {
        OutputType::Unicode => {
            println!("Your QR code:");
            qr_code.print();
        }
        OutputType::PNG => {
            let out = qr_args.output.unwrap_or("qr.png".into());
            ImageBuilder::default()
                .shape(Shape::Square)
                .fit_width(300)
                .to_file(&qr_code, &out)
                .expect("Cannot write out to file!");
            println!("Your QR code is at {out}");
        }
        OutputType::SVG => {
            let out = qr_args.output.unwrap_or("qr.svg".into());
            SvgBuilder::default()
                .shape(Shape::Square)
                .to_file(&qr_code, &out)
                .expect("Cannot write out to file!");
            println!("Your QR code is at {out}");
        }
    }
    Ok(())
}
