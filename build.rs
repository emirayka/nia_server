extern crate serde_json;

use std::iter::Peekable;
use std::{fs, env};
use std::io::{Read, BufReader, BufRead, Error, BufWriter, Write};

use serde_json::{Value, Map};
use serde_json::value::Number;
use std::path::Path;
use std::fs::File;

const KBM_INPUT_DIR: &'static str = "keyboard_models";
const KBM_OUTPUT_DIR: &'static str = "dist";
const PHF_INPUT_DIR: &'static str = "dist";

fn read(path: &str) -> Vec<i64> {
    let result = fs::read_to_string(path)
        .expect("Failure reading input file.");

    result.lines()
        .flat_map(|line| line.split_whitespace())
        .map(|part| {
            part.parse()
                .expect("Cannot convert an integer")
        })
        .collect()
}

fn convert_to_json(integers: Vec<i64>) -> Value {
    let mut iter = integers.into_iter().peekable();

    let width = iter.next()
        .expect("Cannot read width from input file")
        .into();

    let height = iter.next()
        .expect("Cannot read height from input file")
        .into();

    let mut map = Map::new();

    map.insert(String::from("width"), width);
    map.insert(String::from("height"), height);

    let mut keys = Vec::new();

    while iter.peek().is_some() {
        let mut key_map = Map::new();

        let x = iter.next()
            .expect("Cannot read key horizontal position.")
            .into();

        let y = iter.next()
            .expect("Cannot read key vertical position.")
            .into();

        let width = iter.next()
            .expect("Cannot read key width.")
            .into();

        let height = iter.next()
            .expect("Cannot read key height.")
            .into();

        let code = iter.next()
            .expect("Cannot read key code.")
            .into();

        key_map.insert(String::from("x"), x);
        key_map.insert(String::from("y"), y);
        key_map.insert(String::from("width"), width);
        key_map.insert(String::from("height"), height);
        key_map.insert(String::from("code"), code);

        keys.push(Value::Object(key_map));
    }

    map.insert(String::from("keys"), Value::Array(keys));

    Value::Object(map)
}

fn write(value: Value, output_file_path: &str) {
    let string = serde_json::to_string(&value)
        .expect("Cannot convert json value to string");

    fs::write(output_file_path, string);
}

fn compile_kbm_file(input_file_path: String, output_file_path: String) {
    let integers = read(&input_file_path);
    let json = convert_to_json(integers);

    write(json, &output_file_path);
}

fn compile_kbm_files() {
    let mut input_dir_path = Path::new(KBM_INPUT_DIR);
    let mut output_dir_path = Path::new(KBM_OUTPUT_DIR);

    for entry in fs::read_dir(input_dir_path).unwrap() {
        let entry = entry.unwrap();

        let input_name = entry.path().file_name().unwrap().to_str().unwrap().to_string();

        let mut output_path = entry.path();
        output_path.set_extension("json");

        let output_name = output_path.file_name().unwrap().to_str().unwrap().to_string();

        let input_file_path = input_dir_path.join(input_name).to_str().unwrap().to_string();
        let output_file_path = output_dir_path.join(output_name).to_str().unwrap().to_string();

        compile_kbm_file(input_file_path, output_file_path);
    }
}


fn main() -> Result<(), Error> {
    compile_kbm_files();

    Ok(())
}
