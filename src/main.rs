
// local lib
extern crate mylib;
extern crate dc_utils;

// other lib
extern crate num_complex;
extern crate domain_coloring;
extern crate structopt;

use structopt::StructOpt;
use num_complex::Complex64;
use domain_coloring::{*,LightnessAlg::*};
use mylib::*;
use dc_utils::*;

fn main() {
    let opt = Opts::from_args();
    let v = opt.verbose;
    if v > 2 { println!("{:?}", opt) }
    let i: ImageDesc = img_desc_builder(opt.img);
    let f = match opt.func {
        FuncOpt::Roots{roots: v} => Box::new(C64Roots(v.iter().map(|x| x.0).collect())) as Box<ComplexFunction>,
        FuncOpt::Terms{terms: v} => Box::new(C64Terms(v.iter().map(|x| x.0).collect())) as Box<ComplexFunction>,
        FuncOpt::Unity{n: x} => Box::new(C64Unity(x as usize)) as Box<ComplexFunction>,
        _ => Box::new(C64Roots(vec![])) as Box<ComplexFunction>,
    };
    if v > 1 { println!("Image details: {:?}", i) }
    if v > 3 { println!("{:?}", f.eval_at(Complex64::new(1.0, 0.0))) }
    if v > 0 { println!("Writing to {:?}.", opt.imgfile) }
    domain_color_simple(&i, &(|z| f.eval_at(z)), &opt.imgfile);
}

