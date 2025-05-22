# 8-bit-computer

A turing complete Von-Neumann 8-bit computer build entirely in Logisim Evolution.

## High overview of the architecture
On the diagramm below the spatial positon of different modules inside the computer can be seen. 
Thick lines represent the data flow and thin lines show the clock propagation paths.

![structure](/diagrams/Structure_Diagram.svg)

## Usage
In order to be able to experiment and test the 8-bit-compuer the [Logisim Evoltuion](https://github.com/logisim-evolution/logisim-evolution) is required.
It is simple: 
  1. Install [Logisim Evoltuion](https://github.com/logisim-evolution/logisim-evolution) on your system.
  2. Open the [8_bit_computer.circ](8_bit_computer.circ).

The [standard Logisim](http://www.cburch.com/logisim/) might work, though this has never been tested.

## Assembly instructions overview
This computer has a total of 16 byte of RAM and has the following set of assembly instructions:
  - `NOP` -> `0000` -> No operation
  - `LDA X` -> `0001` -> Load a value from RAM at the address `X` into the `A rigister`
  - `ADD X` -> `0010` -> Add a value from RAM at the address `X` to the`A rigister` and store it in the `A rigister`
  - `SUB X` -> `0011` -> Subtruct a value from RAM at the address `X` from `A rigister` and store it in the `A rigister`
  - `STA X` -> `0100` -> Load a value from the `A rigister` into RAM at the address `X`
  - `LDI X` -> `0101` -> Load `X` into the `A rigister`
  - `JMP X` -> `0110` -> Jump to the instruction `X`
  - `JC X` -> `0111` -> Jump to the instruction `X` if the carry flag is set
  - `JZ X` -> `1000` -> Jump to the instruction `X` if the zero flag is set
  - `OUT` -> `1110` -> Output the contents of the `A rigister`
  - `HLT` -> `1111` -> Halt (Not implemented yet)

## Programming the 8-bit-computer

### Programming the computer using `assembly-compiler`:
  1. Create a file containing your valid assembly instruction ( please think of the the limitaions of the 16-byte RAM)
  2. Compile it using the `assembly-compiler`:
     - Make sure you have `RUST` installed on your machine
     - Navigate to the `assembly-compiler` folder
     - Compile using the following command `cargo run -- -i <your_assembly_file> -o <output_file_path> -l`
  3. Copy the contents of the output file into the RAM module:
     - Right click on the `PRAM`
     - Select `Edit Contents...`.
     - Select the fist address.
     - Paste the contens of your compiled code  using `v.3.0 hex` , `words` and `addressed` as settings.
     - Click `Ok`.
  4. Set `PRAM` pin to `1/High` in order to switch to programmable memory.
  5. Start the clock (cmd + k)

### Programming each instruction manually:
  1. Set the `MANUAL` pin to `1/High`
  2. Set the `RI` pin `1/High`
  3. Set the address using the pins `A0` - `A3`
  4. Set the assembly opcode (and the `X` if needed) using `D0` - `D7`
  5. Puls the clock `High` and `Low` (cmd + t)
  6. Repeat for all the other instruction
After writing all the instructions and data to the RAM:
  1. Set the `MANUAL` pin back to `0/Low`
  2. Reset the computer using the `RESET` button
  3. Start the clock (cmd + k)

## Programming the Control Unit ROM
For Control Unit to work correctly it has to be programmed. The [code](microcode_gen.c) for generatin of the ROM microinstruction is written in C and produces a file called [control_unit](control_unit) which has the microcode in it.
In most cases one does not have to reprogramm the ROM. In case if it has to be done:
  1. Modify the [microcode_gen.c](microcode_gen.c) if needed.
  2. Compile and run the program.
  3. Open [8_bit_computer.circ](8_bit_computer.circ) in Logisim Evolution.
  4. Right click on the Control Unit ROM.
  5. Select `Edit Contents...`.
  6. Select the fist address.
  7. Paste the contens of generated [control_unit](control_unit) using `v.3.0 hex` , `words` and `addressed` as settings.
  8. Click `Ok`.

## Picture of the Direct Implementation
![Screenshot of the Direct Implementation from Logisim Evolution](https://github.com/user-attachments/assets/a3ccc37f-1156-417c-a840-9979b6c1b0f3)


## Acknowledgements
This project was inspited by the works of [Ben Eater](https://github.com/beneater) and his tutorial series on [building an 8-bit breadboard computer!](https://www.youtube.com/watch?v=HyznrdDSSGM&list=PLowKtXNTBypGqImE405J2565dvjafglHU). Some of the designs were take directly from his videos and some were modified. Also some parts of [microcode_gen.c](microcode_gen.c) were taken from the [eeprom-programmer](https://github.com/beneater/eeprom-programmer?tab=readme-ov-file) of Ben Eater.
     
