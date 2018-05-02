extern crate mylib;
extern crate num_complex;
extern crate domain_coloring;
extern crate structopt;
use structopt::StructOpt;

use num_complex::Complex64;
use domain_coloring::{*,LightnessAlg::*};
use std::env::args;
use mylib::*;

//TODO: return Box<Iterator> or use struct w/ impl Iterator?
fn parse_as_c64(slice: &[String]) -> Vec<Complex64> {
    let mut v: Vec<Complex64> = vec![];
    for s in slice {
        if let Ok(MyC64(num)) = s.parse::<MyC64>() {
            v.push(num);
        } else {
            eprintln!("Warning: could not parse {} as a Complex number, ignoring", s);
        }
    }
    v
}

fn parse_as_roots(slice: &[String]) -> C64Roots {
    C64Roots(parse_as_c64(slice))
}

fn parse_as_terms(slice: &[String]) -> C64Terms {
    C64Terms(parse_as_c64(slice))
}

fn main() {
    let opt = FuncOpt::from_args();
    println!("{:?}", opt);
}
