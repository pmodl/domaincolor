extern crate num_complex;
extern crate regex;
use num_complex::{Complex64};
#[macro_use] extern crate lazy_static;
use regex::Regex;
use std::{
    error::Error,
    fmt
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_parsing() {
        for (z, ss) in vec![
            (Complex64::new(1.0, 4.1), vec![
             "1 + 4.1i", "1+4.1i", " 1--4.1 j"
            ]),
            (Complex64::new(0.0, 1.0), vec![
             "i", "j", "0.000 + i", "0 +j", "0+1.0i"
            ]),
            (Complex64::new(0.0, -1.0), vec![
             "-i", "---j", "0.000 - i", "-0 -j", "0-1.0i"
            ]),
            (Complex64::new(1.0, 0.0), vec![
             "1", "--1", "+", "ii", "i j"
            ]),
        ]{
            let z1 = MyC64(z);
            for s in ss.iter() {
                let z2 = s.parse::<MyC64>();
                assert_eq!(z1, z2.unwrap());
            }
        }
        assert!("text".parse::<MyC64>().is_err());
    }
}

#[derive(Debug,PartialEq)]
pub struct MyC64 (pub Complex64);
impl std::str::FromStr for MyC64 {
    type Err = C64ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref NUM_RE: Regex = Regex::new("^[[:digit:].][[:digit:].]*").unwrap();
        }
        let mut i  = 0;
        let mut j  = 0;
        let mut s1 = s;

        let mut ret = Complex64::new(0.0, 0.0);

        while s1.len() > 0 {

            let mut neg: bool = false;
            let mut is_im = false;
            let mut val: f64 = 1.0;

            // -- get sign --
            for c in s1.chars() {
                match c {
                    ' ' | '+' => (),
                    '-'       => neg ^= true,
                    _         => break,
                };
                i += 1;
            }

            s1 = s.get(i..).unwrap(); // update slice

            // -- match number --

            // -- parse number --
            if let Some(cap) = NUM_RE.captures(s1) {
                if cap.len() > 0 {
                    if let Ok(f) = cap[0].parse::<f64>() {
                        val = f;
                    }
                    i += cap[0].len();
                }
            }

            s1 = s.get(i..).unwrap(); // update slice

            // -- get imaginary --
            for c in s1.chars() {
                match c {
                    ' ' => (),
                    'i' | 'j' | 'I' | 'J'
                        => is_im ^= true,
                    _   => break,
                };
                i += 1;
            }
            if neg { val = -val; }
            if is_im {
                ret.im += val;
            } else {
                ret.re += val;
            }

            // could not parse: Err
            if i == j {
                return Err(C64ParseError(()));
            }
            j = i;
            s1 = s.get(i..).unwrap(); // update slice
        }
        return Ok(MyC64(ret));
    }
}
impl std::fmt::Display for MyC64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} + {}i", self.0.re, self.0.im)
    }
}

// StructOpt requires FromStr::Err to impl Display for error mesages
#[derive(Debug)]
pub struct C64ParseError(());
impl Error for C64ParseError {
    fn description(&self) -> &str {
        "invalid format for complex value"
    }
}
impl fmt::Display for C64ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.description())
    }
}
