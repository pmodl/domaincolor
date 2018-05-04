
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
    let opt = FuncOpt::from_args();
    println!("{:?}", opt);
}
