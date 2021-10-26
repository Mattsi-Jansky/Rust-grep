use std::fs;
use anyhow::*;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug)]
pub struct Flags;

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Flags {}
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let result = fs::read_to_string(files[0]);
    match result {
        Err(_) => Err(anyhow!("something went bang")),
        Ok(contents) => {
            Ok(do_the_thing(contents, pattern))
        }
    }
}

fn do_the_thing(file_contents: String, pattern: &str) -> Vec<String> {
    if file_contents.contains(pattern) {
        vec![String::from(pattern)]
    } else {
        vec![]
    }
}
