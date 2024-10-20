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

fn get_opcode_number_from_str(opcode: &str) -> Result<&str, &'static str> {
    match opcode {
        "nop" => Ok("0000"),
        "lda" => Ok("0001"),
        "add" => Ok("0010"),
        "sub" => Ok("0011"),
        "sta" => Ok("0100"),
        "ldi" => Ok("0101"),
        "jmp" => Ok("0110"),
        "jc" => Ok("0111"),
        "jz" => Ok("1000"),
        "out" => Ok("1110"),
        "hlt" => Ok("1111"),
        _ => Err("Wrong opcode: {_}!"),
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
        println!("!!!!");
        let count = cnt;
        let line_contents: Vec<&str> = line.split_whitespace().collect();
        let opcode = get_opcode_number_from_str(line_contents[0])?;
        println!("!{opcode}");
    }
    Ok(())
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Compiling the assembly form {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    println!("Contents of the file:\n{contents}");

    let _ = compile(&contents);

    Ok(())
}
