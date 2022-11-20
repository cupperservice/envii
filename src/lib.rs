use std::path::Path;
use std::fs;
use std::io::{BufReader, Read};

use serde_yaml;

pub struct Config {

}

fn from_file(filename: &Path) {
    from_reader(fs::File::open(filename))
}

fn from_reader(reader: &mut Read) {
    serde_yaml::from_reader(reader).unwrap()
}
