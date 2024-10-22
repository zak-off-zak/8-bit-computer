use clap::Parser;
use core::fmt;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::{fs, u8};

#[derive(Parser)]
pub struct Config {
    #[arg(short, long)]
    input_file_path: String,
    #[arg(short, long, default_value_t = String::from("a.out"))]
    output_file_path: String,
}

#[derive(Debug)]
pub struct Instruction {
    count: u8,
    opcode: String,
    argument: String,
}

impl Instruction {
    fn new(count: u8, opcode: &str, argument: &str) -> Instruction {
        Instruction {
            count,
            opcode: opcode.to_owned(),
            argument: argument.to_owned(),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04b}: {} {}", self.count, self.opcode, self.argument)
    }
}

fn get_opcode_number_from_str(opcode: &str) -> Result<String, &'static str> {
    match opcode {
        "nop" => Ok(String::from("0000")),
        "lda" => Ok(String::from("0001")),
        "add" => Ok(String::from("0010")),
        "sub" => Ok(String::from("0011")),
        "sta" => Ok(String::from("0100")),
        "ldi" => Ok(String::from("0101")),
        "jmp" => Ok(String::from("0110")),
        "jc" => Ok(String::from("0111")),
        "jz" => Ok(String::from("1000")),
        "out" => Ok(String::from("1110")),
        "hlt" => Ok(String::from("1111")),
        _ => Err("Wrong opcode: {_}!"),
    }
}

pub fn compile(contents: &str, output_file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut cnt: u8 = 0;
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in contents.lines() {
        let count = cnt;
        let line_contents: Vec<&str> = line.split_whitespace().collect();
        let opcode = get_opcode_number_from_str(&line_contents[0].to_lowercase())?;
        let argument = line_contents[1];
        instructions.push(Instruction::new(count, &opcode, argument));
        cnt += 1;
    }
    for i in cnt..16 {
        instructions.push(Instruction::new(i, "0000", "00000"));
    }
    println!("Resulting instructions: ");
    let mut output = File::create(output_file_path)?;
    for instruction in instructions {
        println!("{instruction}");
        writeln!(output, "{instruction}")?;
    }
    Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "Compiling the assembly form {} into {}",
        config.input_file_path, config.output_file_path
    );

    let contents = fs::read_to_string(config.input_file_path)?;

    println!("Contents of the input file:\n{contents}");

    let _ = compile(&contents, &config.output_file_path);

    Ok(())
}
