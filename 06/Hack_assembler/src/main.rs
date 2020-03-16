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
use std::io::LineWriter;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_file_name = &args[1];

    let input_file_path = Path::new(input_file_name);

    let mut input_file = match File::open(input_file_path) {
        Ok(file) => file,
        Err(error) => {
            println!("Unable to read file!");
            return Err(error)
        }
    };

    let mut file_contents = String::new();

    input_file.read_to_string(&mut file_contents)?;

    let lines = file_contents.trim().split("\n");

    let output_file_name = String::from(input_file_path.file_stem().unwrap().to_str().unwrap()) + ".hack"; //ew

    let mut symbol_table = SymbolTable::new();

    let write_file = File::create(output_file_name)?;
    let mut write_file = LineWriter::new(write_file);

    //Phase 1, register all labels
    //The parse line function will create any new labels that it doesn't know about, so thats why I can just use it twice, the first time I just don't write
    //Any of the generated instructions.

    let mut index = 0;

    for line in lines.clone() {
        println!("{}", line);
    }

    for line in lines.clone() {
        index = match parse_line(line, &mut symbol_table, index, true) {
            Some(_) => index + 1,
            None => index
        }
    }

    symbol_table.print_table();

    index = 0;

    // Phase 2
    for line in lines.clone() {
        match parse_line(line, &mut symbol_table, index, false) {
            Some(instruction) => {
                if !instruction.is_empty() {
                    write_file.write_all(&format!("{:016b}\n", assemble_instruction(&instruction)).as_bytes()).expect("Unable to write to file")
                }
                index += 1;
            },
            None => () //println!("Unable to parse line {}!", index),
        };
    }

    Ok(())
}

