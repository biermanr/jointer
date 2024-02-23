use std::fs::read_to_string;
use clap::Parser;
use std::collections::HashMap;

/// jointer command line arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Join-field for file 1
    #[arg(short='1', default_value_t = 0)]
    field_f1: usize,

    /// Join-field for file 2
    #[arg(short='2', default_value_t = 0)]
    field_f2: usize,

    /// File-path for file 1
    #[arg()]
    f1: std::path::PathBuf,

    /// File-path for file 2
    #[arg()]
    f2: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let f1 = read_to_string(args.f1).unwrap();
    let f2 = read_to_string(args.f2).unwrap();

    // NOTE THIS IS CURRENTLY HARD-CODING NON-KEY COLUMNS!
    // works only for:
    // ./target/debug/jointer data/readme_examples/country_code_examples/country_populations.tsv data/readme_examples/country_code_examples/country_codes.tsv


    // Read all of file 1 into a hashmap (TODO, buffered read)
    // indexed on the field_f1 column, valued on remaining fields
    let mut f1_hm = HashMap::new();
    for line in f1.lines() {
        let fields: Vec<&str> = line.split("\t").collect();
        f1_hm.insert(fields[args.field_f1], fields[1]);
    }

    // Iterate through file 2, checking values against hashmap
    for line in f2.lines() {
        let fields: Vec<&str> = line.split("\t").collect();
        let k = fields[args.field_f2];
        if let Some(v) = f1_hm.get(k) {
            println!("{}\t{}\t{}",k,v,fields[1]);
        }
    }

}
