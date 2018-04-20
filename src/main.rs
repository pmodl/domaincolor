extern crate mylib;
extern crate num_complex;
extern crate domain_coloring;

use num_complex::Complex64;
use domain_coloring::{*,LightnessAlg::*};
use std::env::args;
use mylib::*;

//TODO: return Box<Iterator> or use struct w/ impl Iterator?
fn parse_as_c64(slice: &[String]) -> Vec<Complex64> {
    let mut v: Vec<Complex64> = vec![];
    for s in slice {
        if let Ok(MyC64(num)) = s.parse::<MyC64>() {
            f.push(num);
        } else {
            eprintln!("Warning: could not parse {} as a Complex number, ignoring", s);
        }
    }
    v
}

fn parse_as_roots(slice &[String]) -> &ComplexFunction {
    C64Roots(parse_as_c64(slice))
}

fn parse_as_terms(slice &[String]) -> &ComplexFunction {
    C64Terms(parse_as_c64(slice))
}

fn main() {
    let mut argv: Vec<String> = vec![];
    for arg in args() {
        argv.push(arg);
    }
    for num in parse_as_c64(&argv[1..]) {
        println!("Parsed: {}", num)
    };
}
