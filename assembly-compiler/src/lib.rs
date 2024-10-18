use std::error::Error;
use std::{fs, u8};

pub struct Config {
    file_path: String,
}

pub struct Line {
    count: u8,
    opcode: String,
    argument: String,
}

enum Opcode {
    Nop,
    Lda,
    Add,
    Sub,
    Sta,
    Ldi,
    Jmp,
    Jc,
    Jz,
    Out,
    Hlt,
}

fn get_opcode_number(opcode: &Opcode) -> Result<&str, &'static str> {
    match opcode {
        Opcode::Nop => Ok("0000"),
        Opcode::Lda => Ok("0001"),
        Opcode::Add => Ok("0010"),
        Opcode::Sub => Ok("0011"),
        Opcode::Sta => Ok("0100"),
        Opcode::Ldi => Ok("0101"),
        Opcode::Jmp => Ok("0110"),
        Opcode::Jc => Ok("0111"),
        Opcode::Jz => Ok("1000"),
        Opcode::Out => Ok("1110"),
        Opcode::Hlt => Ok("1111"),
    }
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough argumetns!");
        }
        let file_path = args[1].clone();
        Ok(Config { file_path })
    }
}

pub fn compile(contents: &str) -> Result<(), Box<dyn Error>> {
    let mut cnt: u8 = 0;
    let mut instructiosn: Vec<Line> = Vec::new();
    for line in contents.lines() {
        let count = cnt;
        let line_contents: Vec<&str> = line.split_whitespace().collect();
        let opcode = get_opcode_number(line_contents[0]);
    }
    Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Compiling the assembly form {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    println!("Contents of the file:\n{contents}");

    Ok(())
}
