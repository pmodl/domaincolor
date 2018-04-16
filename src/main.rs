extern crate mylib;
extern crate num_complex;
extern crate domain_coloring;

use num_complex::Complex64;
use domain_coloring::{*,LightnessAlg::*};
use std::env::args;
use mylib::*;


fn main() {
    //let mut f: Vec<Complex64>;
    for arg in args() {
        println!("{}", arg);
        println!("{:?}", arg.parse::<MyC64>());
    }
}
