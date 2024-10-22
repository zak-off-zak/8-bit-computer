use core::fmt;
use std::error::Error;
use std::{fs, u8};

pub struct Config {
    file_path: String,
}

#[derive(Debug)]
pub struct Line {
    count: u8,
    opcode: String,
    argument: String,
}

impl Line {
    fn new(count: u8, opcode: &str, argument: &str) -> Line {
        Line {
            count,
            opcode: opcode.to_owned(),
            argument: argument.to_owned(),
        }
    }
}

impl fmt::Display for Line {
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
    let mut instructions: Vec<Line> = Vec::new();
    for line in contents.lines() {
        let count = cnt;
        let line_contents: Vec<&str> = line.split_whitespace().collect();
        let opcode = get_opcode_number_from_str(&line_contents[0].to_lowercase())?;
        let argument = line_contents[1];
        instructions.push(Line::new(count, &opcode, argument));
        cnt += 1;
    }
    for instruction in instructions {
        println!("{instruction}");
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
