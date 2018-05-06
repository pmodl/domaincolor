use structopt::{StructOpt,clap::AppSettings};
#[macro_use] extern crate structopt;
extern crate dc_utils;
extern crate domain_coloring;
use domain_coloring::*;
use dc_utils::*;

// macro uses `///` comments to derive help text
#[derive(StructOpt, Debug)]
// https://docs.rs/clap/2/clap/enum.AppSettings.html#variant.InferSubcommands
#[structopt(raw(setting = "AppSettings::InferSubcommands"))]
pub enum FuncOpt {
    // https://docs.rs/clap/2/clap/struct.App.html#method.alias
    /// roots of a polynomial
    #[structopt(name = "roots", alias = "r")]
    Roots { roots: Vec<MyC64> },
    // https://docs.rs/clap/2/clap/struct.App.html#method.aliases
    /// terms of a polynomial
    #[structopt(name = "terms", raw(aliases = r#"&["t", "term"]"#))]
    Terms { terms: Vec<MyC64> },
    /// use the nth roots of unity
    #[structopt(name = "unity", alias = "u")]
    Unity { n: u32 },
}

#[derive(StructOpt, Debug)]
pub struct ImgOpt {
    /// pixel dimensions of the output image
    #[structopt(short = "d", long = "dim", default_value = "\"1000\"" )]
    pub dim : XYU32,
    /// resolution (pixels per unit)
    #[structopt(short = "r", long = "res")]
    pub res: Option<XYF64>,
    /// delta (units per pixel)
    #[structopt(short = "δ", long = "delta")]
    pub delta: Option<XYF64>,
    /// value dimensions of the output image
    #[structopt(short = "w", long = "width")]
    pub width: Option<XYF64>,
}

#[derive(StructOpt, Debug)]
pub struct Opts {
    /// increase verbosity
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: u8,
    #[structopt(flatten)]
    pub func: FuncOpt,
    #[structopt(flatten)]
    pub img: ImgOpt,
    /// image file to create
    #[structopt(short = "o", long = "output", default_value = "a.png")]
    pub imgfile: String,
}

use std::str::FromStr;
// TODO: parameterize over <T: FromStr<T>>
#[derive(Debug)]
pub struct XYU32(pub u32, pub Option<u32>);
impl FromStr for XYU32 {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                                 .split(",")
                                 .collect();
        let x = coords[0].parse::<u32>()?;
        if coords.len() > 1 {
            let y = coords[1].parse::<u32>()?;
            Ok(XYU32(x,Some(y)))
        } else {
            Ok(XYU32(x, None))
        }
    }
}
#[derive(Debug)]
pub struct XYF64(pub f64, pub Option<f64>);
impl FromStr for XYF64 {
    type Err = std::num::ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                                 .split(",")
                                 .collect();
        let x = coords[0].parse::<f64>()?;
        if coords.len() > 1 {
            let y = coords[1].parse::<f64>()?;
            Ok(XYF64(x,Some(y)))
        } else {
            Ok(XYF64(x, None))
        }
    }
}

pub fn img_desc_builder(opts: ImgOpt) -> ImageDesc {
    let mut img = ImageDesc { width: 0, height: 0, xres: 0.0, yres: 0.0 };

    match opts.dim {
        XYU32(x, Some(y)) => { img.width = x; img.height = y; }
        XYU32(x, None)    => { img.width = x; img.height = x; }
    };
    match opts.res {
        Some(XYF64(x, Some(y))) => { img.xres = x.recip(); img.yres = y.recip(); }
        Some(XYF64(x, None))    => { img.xres = x.recip(); img.yres = x.recip(); }
        _                       => {;}
    }
    match opts.delta {
        Some(XYF64(x, Some(y))) => { img.xres = x; img.yres = y; }
        Some(XYF64(x, None)) => { img.xres = x; img.yres = x; }
        _ => {;}
    }
    match opts.width {
        Some(XYF64(x, Some(y))) => {
            img.xres = x / img.width as f64;
            img.yres = y / img.height as f64;
        }
        Some(XYF64(x, None)) => {
            img.xres = x / img.width as f64;
            img.yres = x / img.width as f64;
        }
        _ => {;}
    }
    if img.xres == 0.0 {
        img.xres = 0.01;
        img.yres = 0.01;
    }
    img
}

