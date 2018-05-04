use structopt::{StructOpt,clap::AppSettings};
#[macro_use] extern crate structopt;
extern crate dc_utils;
use dc_utils::*;

#[derive(StructOpt, Debug)]
// https://docs.rs/clap/2/clap/enum.AppSettings.html#variant.InferSubcommands
#[structopt(raw(setting = "AppSettings::InferSubcommands"))]
pub enum FuncOpt {
    // https://docs.rs/clap/2/clap/struct.App.html#method.alias
    /// roots of a polynomial
    #[structopt(name = "roots", alias = "r")]
    Roots {
        #[structopt(name = "ROOTS")]
        roots: Vec<MyC64>,
    },
    // https://docs.rs/clap/2/clap/struct.App.html#method.aliases
    /// terms of a polynomial
    #[structopt(name = "terms", raw(aliases = r#"&["t", "term"]"#))]
    Terms {
        terms: Vec<MyC64>,
    },
    /// roots of unity
    #[structopt(name = "unity", alias = "u")]
    Unity {
        /// use the nth roots of unity
        n: u32
    },
}

