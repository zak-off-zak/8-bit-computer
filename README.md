# 8-bit-computer

A turing complete 8-bit computer build entirely in Logisim Evolution.

## High overview of the architecture
On the diagramm below the spetial positon of different modules inside the computer can be seen can be seen. 
Thick lines represent the data flow and thin lines show the clock propagtions paths.

![structure](/diagrams/Structure_Diagram.svg)

## Usage
In order to be able to experiment and test the 8-bit-compuer the [Logisim Evoltuion](https://github.com/logisim-evolution/logisim-evolution) is required.
It is pretty simple: 
  1. Install [Logisim Evoltuion](https://github.com/logisim-evolution/logisim-evolution) on your system.
  2. Open the [8_bit_computer.circ](8_bit_computer.circ).
  3. You are ready to go!

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
In order to programm the computer:
  1. Set the `MANUAL` pin to `1/High`
  2. Set the `RI` pin `1/High`
  3. Set the address using the pins `A0` - `A3`
  4. Set the assembly opcode + the `X` if needed using `D0` - `D7`
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
  4. Double right click on the Control Unit ROM.
  5. Select `Edit Contents...`.
  6. Select the fist address.
  7. Paste the contens of generated [control_unit](control_unit) using `v.3.0 hex` , `words` and `addressed` as settings.
  8. Click `Ok`.
  9. You are ready to go!

## Acknowledgements
This project was inspited by the works of [Ben Eater](https://github.com/beneater) and his tutorial series on [building an 8-bit breadboard computer!](https://www.youtube.com/watch?v=HyznrdDSSGM&list=PLowKtXNTBypGqImE405J2565dvjafglHU). Some of the designs were take directly from his videos and some were modified. Also some parts of [microcode_gen.c](microcode_gen.c) were taken from the [eeprom-programmer](https://github.com/beneater/eeprom-programmer?tab=readme-ov-file) of Ben Eater.
     
