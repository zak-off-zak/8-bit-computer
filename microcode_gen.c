#include <stdio.h>
#include <stdlib.h>

// This project was inspired by https://github.com/beneater and his https://github.com/beneater/eeprom-programmer/blob/master/microcode-eeprom-programmer/microcode-eeprom-programmer.ino

#define MI  0b1000000000000000  // Memory address register in
#define RI  0b0100000000000000  // RAM data in
#define RO  0b0010000000000000  // RAM data out
#define IO  0b0001000000000000  // Instruction register out
#define II  0b0000100000000000  // Instruction register in
#define AI  0b0000010000000000  // A register in
#define AO  0b0000001000000000  // A register out
#define EO  0b0000000100000000  // ALU out
#define SU  0b0000000010000000  // ALU subtract
#define BI  0b0000000001000000  // B register in
#define OI  0b0000000000100000  // Output register in
#define CE  0b0000000000010000  // Program counter enable
#define CO  0b0000000000001000  // Program counter out
#define J   0b0000000000000100  // Jump (program counter in)
#define HLT 0b0000000000000010  // Halt clock

// Microcode instructions for the CPU
uint16_t data[] = {
  MI|CO,  RO|II|CE,  0,      0,      0,         0, 0, 0,   // 0000 - NOP
  MI|CO,  RO|II|CE,  IO|MI,  RO|AI,  0,         0, 0, 0,   // 0001 - LDA
  MI|CO,  RO|II|CE,  IO|MI,  RO|BI,  EO|AI,     0, 0, 0,   // 0010 - ADD
  MI|CO,  RO|II|CE,  IO|MI,  RO|BI,  EO|AI|SU,  0, 0, 0,   // 0011 - SUB
  MI|CO,  RO|II|CE,  IO|MI,  AO|RI,  0,         0, 0, 0,   // 0100 - STA
  MI|CO,  RO|II|CE,  IO|AI,  0,      0,         0, 0, 0,   // 0101 - LDI
  MI|CO,  RO|II|CE,  IO|J,   0,      0,         0, 0, 0,   // 0110 - JMP
  MI|CO,  RO|II|CE,  0,      0,      0,         0, 0, 0,   // 0111
  MI|CO,  RO|II|CE,  0,      0,      0,         0, 0, 0,   // 1000
  MI|CO,  RO|II|CE,  0,      0,      0,         0, 0, 0,   // 1001
  MI|CO,  RO|II|CE,  0,      0,      0,         0, 0, 0,   // 1010
  MI|CO,  RO|II|CE,  0,      0,      0,         0, 0, 0,   // 1011
  MI|CO,  RO|II|CE,  0,      0,      0,         0, 0, 0,   // 1100
  MI|CO,  RO|II|CE,  0,      0,      0,         0, 0, 0,   // 1101
  MI|CO,  RO|II|CE,  AO|OI,  0,      0,         0, 0, 0,   // 1110 - OUT
  MI|CO,  RO|II|CE,  HLT,    0,      0,         0, 0, 0,   // 1111 - HLT
};


int main(){
    FILE *control_unit_rom;

    control_unit_rom = fopen("control_unit", "w");

    // Writing data to the ROM
    if(control_unit_rom == NULL) {
        printf("The file for the first ROM Control Unit was not found! Exiting... \n");
        exit(0);
    } else{
        fprintf(control_unit_rom, "v3.0 hex words addressed\n");
        for(int address = 0; address < sizeof(data) / 2; address += 16)
        {
            char buf[128];
            sprintf(buf, "%02x: %04x %04x %04x %04x %04x %04x %04x %04x %04x %04x %04x %04x %04x %04x %04x %04x", address, data[(address) + 0], data[(address) + 1], data[(address) + 2], data[(address) + 3], data[(address) + 4], data[(address) + 5], data[(address) + 6], data[(address) + 7], data[(address) + 8], data[(address) + 9], data[(address) + 10], data[(address) + 11], data[(address) + 12], data[(address) + 13], data[(address) + 14], data[(address) + 15]);
            fprintf(control_unit_rom, "%s\n", buf);
        }
        fclose(control_unit_rom);
    }

}
