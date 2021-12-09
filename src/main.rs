use std::fs::{File};
use std::io::Read;
use std::process::exit;

use crate::parser::{convert_source_to_op_chain, sanitize_source};

mod parser;

fn main() {
    let mut src_code = read_source_code("test.brainfuck");
    sanitize_source(&mut src_code);
    convert_source_to_op_chain(&src_code);
    println!("{}", src_code);
}

fn read_source_code(filename: &str) -> String {
    let file = File::open(filename);
    let mut file = file.unwrap_or_else(|_| {
        println!("Could not open given source code file!");
        exit(1);
    });

    let mut content = String::new();
    let read_res = file.read_to_string(&mut content);
    match read_res {
        Ok(_) => {}
        Err(_) => {
            println!("Error reading file!");
            exit(1);
        }
    }

    return content;
}
