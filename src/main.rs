extern crate num_complex;
extern crate domain_coloring;

use num_complex::Complex64;
use domain_coloring::{*,LightnessAlg::*};
use std::env::args;


fn main() {
    //let mut f: Vec<Complex64>;
    for arg in args() {
        println!("{}", arg);
    }
}
