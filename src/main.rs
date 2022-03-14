//use std::process::Command;

//use clap::App;
//use clap::Arg;

//use std::collections::hash_map::HashMap;

use clap::{Parser,ValueHint};
use std::io::{self, BufRead};

use std::fs::File;
use std::path::PathBuf;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    #[clap(name = "JSON", parse(from_os_str), value_hint = ValueHint::AnyPath)]
    input: PathBuf,
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    #[clap(long)]
    n: bool,
    #[clap(long)]
    min: bool,
    #[clap(long)]
    max: bool,
    #[clap(long)]
    mean: bool,
    #[clap(long)]
    stdev: bool,
    #[clap(long)]
    stderr: bool,
    #[clap(long)]
    sum: bool,
    #[clap(long)]
    variance: bool,

    #[clap(long)]
    q1: bool,
    #[clap(long)]
    q2: bool,
    #[clap(long)]
    q3: bool,
    #[clap(long)]
    percentile: u8,
    #[clap(long)]
    quartile: u8,
    #[clap(long)]
    summary: bool,
    #[clap(long)]
    complete: bool,
    #[clap(long)]
    format: String,

    #[clap(long)]
    delimiter: String,
    #[clap(long)]
    no_header: bool,
    #[clap(long)]
    transpose: bool,

    #[clap(long)]
    strict: bool,
    #[clap(long)]
    quiet: bool,
    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    // let help_str = include_str!("help.txt");

    let args = Args::parse();

//    for _ in 0..args.count {
//        println!("Hello {}!", args.name)
 //   }

    /*
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                    .args(["/C", "echo hello"])
                    .output()
                    .expect("failed to execute process")
        } else {
            Command::new("sh")
                    .arg("-c")
                    .arg("cat help.txt:w")
                    .output()
                    .expect("failed to execute process")
        };

        let hello = output.stdout;

    */

    // println!("{}",my_str);

    // read buffer full of bytes
    // throw them in summary stats
}
