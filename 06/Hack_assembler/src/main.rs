mod assemble;
mod instruction;
mod parse;
mod symbols;

use crate::assemble::assemble_instruction;
use crate::parse::parse_line;
use crate::symbols::SymbolTable;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::fs;
use std::io::LineWriter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_name = &args[1];

    let file_text = match fs::read_to_string(input_file_name) {
        Ok(text) => text,
        Err(_) => {
            println!("Cannot find file {}!", input_file_name);
            return;
        }
    };
    let lines = file_text.trim().split("\n");

    let output_file_name =
        String::from(input_file_name.split(".").next().unwrap()).push_str(".hack");

        
    let symbol_table = SymbolTable::new();

    let write_file = File::create("test.hack").unwrap();
    let mut write_file = LineWriter::new(write_file);

    for (index,line) in lines.enumerate() {
        match parse_line(line, &symbol_table) {
            Some(instruction) => write_file.write_all(&format!("{:016b}\n", assemble_instruction(&instruction)).as_bytes()).expect("Unable to write to file"),
            None => {
                println!("Unable to parse line {}! Exiting...", index);
                return;
            }
        };
    }
}
