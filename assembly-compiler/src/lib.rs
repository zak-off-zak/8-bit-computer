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
    #[clap(long, short, default_value_t = false)]
    logisim_ram: bool,
}

#[derive(Debug)]
pub struct Instruction {
    count: u8,
    opcode: u8,
    argument: String,
}

impl Instruction {
    fn new(count: u8, opcode: u8, argument: &str) -> Instruction {
        Instruction {
            count,
            opcode,
            argument: argument.to_owned(),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04b}: {:04b} {}",
            self.count, self.opcode, self.argument
        )
    }
}

fn get_opcode_number_from_str(opcode: &str) -> Result<u8, &'static str> {
    match opcode {
        "nop" => Ok(0),
        "lda" => Ok(1),
        "add" => Ok(2),
        "sub" => Ok(3),
        "sta" => Ok(4),
        "ldi" => Ok(5),
        "jmp" => Ok(6),
        "jc" => Ok(7),
        "jz" => Ok(8),
        "out" => Ok(14),
        "hlt" => Ok(15),
        _ => Err("Wrong opcode: {_}!"),
    }
}

pub fn compile(
    contents: &str,
    output_file_path: &str,
    logisim_ram: &bool,
) -> Result<(), Box<dyn Error>> {
    let mut cnt: u8 = 0;
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in contents.lines() {
        let count = cnt;
        let line_contents: Vec<&str> = line.split_whitespace().collect();
        let opcode = get_opcode_number_from_str(&line_contents[0].to_lowercase())?;
        let argument = line_contents[1];
        instructions.push(Instruction::new(count, opcode, argument));
        cnt += 1;
    }
    for i in cnt..16 {
        instructions.push(Instruction::new(i, 0, "0"));
    }
    println!("Resulting instructions: ");
    let mut output = File::create(output_file_path)?;
    if !logisim_ram {
        for instruction in instructions {
            println!("{instruction}");
            writeln!(output, "{instruction}")?;
        }
    } else {
        print!("v3.0 hex words addressed\n0:");
        write!(output, "v3.0 hex words addressed\n0:");
        for instruction in instructions {
            print!(" {:1x}{} ", instruction.opcode, instruction.argument);
            write!(
                output,
                " {:1x}{} ",
                instruction.opcode, instruction.argument
            )?;
        }
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

    let _ = compile(&contents, &config.output_file_path, &config.logisim_ram);

    Ok(())
}
