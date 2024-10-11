#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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
#define FI  0b0000000000000001  // Flags in

#define FLAGS_Z0C0 0
#define FLAGS_Z0C1 1
#define FLAGS_Z1C0 2
#define FLAGS_Z1C1 3

#define JC 0b0111
#define JZ 0b1000

// Microcode instructions for the CPU
uint16_t TEMPLATE[16][8] = {
    {MI|CO,  RO|II|CE,  0,      0,      0,            0, 0, 0},   // 0000 - NOP
    {MI|CO,  RO|II|CE,  IO|MI,  RO|AI,  0,            0, 0, 0},   // 0001 - LDA
    {MI|CO,  RO|II|CE,  IO|MI,  RO|BI,  EO|AI|FI,     0, 0, 0},   // 0010 - ADD
    {MI|CO,  RO|II|CE,  IO|MI,  RO|BI,  EO|AI|SU|FI,  0, 0, 0},   // 0011 - SUB
    {MI|CO,  RO|II|CE,  IO|MI,  AO|RI,  0,            0, 0, 0},   // 0100 - STA
    {MI|CO,  RO|II|CE,  IO|AI,  0,      0,            0, 0, 0},   // 0101 - LDI
    {MI|CO,  RO|II|CE,  IO|J,   0,      0,            0, 0, 0},   // 0110 - JMP
    {MI|CO,  RO|II|CE,  0,      0,      0,            0, 0, 0},   // 0111 - JC
    {MI|CO,  RO|II|CE,  0,      0,      0,            0, 0, 0},   // 1000 - JZ
    {MI|CO,  RO|II|CE,  0,      0,      0,            0, 0, 0},   // 1001
    {MI|CO,  RO|II|CE,  0,      0,      0,            0, 0, 0},   // 1010
    {MI|CO,  RO|II|CE,  0,      0,      0,            0, 0, 0},   // 1011
    {MI|CO,  RO|II|CE,  0,      0,      0,            0, 0, 0},   // 1100
    {MI|CO,  RO|II|CE,  0,      0,      0,            0, 0, 0},   // 1101
    {MI|CO,  RO|II|CE,  AO|OI,  0,      0,            0, 0, 0},   // 1110 - OUT
    {MI|CO,  RO|II|CE,  HLT,    0,      0,            0, 0, 0}   // 1111 - HLT
};

uint16_t ucode[4][16][8];

void initUcode(){
    // Z = 0, C = 0
    memcpy(ucode[FLAGS_Z0C0], TEMPLATE, sizeof(TEMPLATE));
    // Z = 0, C = 1
    memcpy(ucode[FLAGS_Z0C1], TEMPLATE, sizeof(TEMPLATE));
    ucode[FLAGS_Z0C1][JC][2] = IO|J;
    // Z = 1, C = 0
    memcpy(ucode[FLAGS_Z1C0], TEMPLATE, sizeof(TEMPLATE));
    ucode[FLAGS_Z1C0][JZ][2] = IO|J;
    // Z = 1, C = 1
    memcpy(ucode[FLAGS_Z1C1], TEMPLATE, sizeof(TEMPLATE));
    ucode[FLAGS_Z1C1][JC][2] = IO|J;
    ucode[FLAGS_Z1C1][JZ][2] = IO|J;
}


int main(){
    initUcode();

    FILE *control_unit_rom;

    control_unit_rom = fopen("control_unit", "w");

    // Writing data to the ROM
    if(control_unit_rom == NULL) {
        printf("The file for the first ROM Control Unit was not found! Exiting... \n");
        exit(0);
    } else{
        fprintf(control_unit_rom, "v3.0 hex words addressed\n");
        int mem_cnt = 0;
        int mem_step_flag = 0;
        fprintf(control_unit_rom, "%02x: ", 0);
        for(int address = 0; address < 1024; address += 1){
            int flags        = (address & 0b1100000000) >> 8;
            int byte_sel     = (address & 0b0010000000) >> 7;
            int instructions = (address & 0b0001111000) >> 3;
            int step         = (address & 0b0000000111);

            // if(step == 0b000){
            //     fprintf(control_unit_rom, "%02x: ", mem_cnt);
            // }
            
            fprintf(control_unit_rom, "%04x ", ucode[flags][instructions][step]);
            if(step == 0b111){
                // fprintf(control_unit_rom, "\n");
                mem_step_flag++;
            }

            if(mem_step_flag == 2){
                fprintf(control_unit_rom, "\n");
                mem_step_flag = 0;
                mem_cnt += 16;
                fprintf(control_unit_rom, "%02x: ", mem_cnt);
            }

        }

        // for(int address = 0; address < sizeof(data) / 2; address += 16)
        // {
        //     char buf[128];
        //     sprintf(buf, "%02x: %04x %04x %04x %04x %04x %04x %04x %04x %04x %04x %04x %04x %04x %04x %04x %04x", address, data[(address) + 0], data[(address) + 1], data[(address) + 2], data[(address) + 3], data[(address) + 4], data[(address) + 5], data[(address) + 6], data[(address) + 7], data[(address) + 8], data[(address) + 9], data[(address) + 10], data[(address) + 11], data[(address) + 12], data[(address) + 13], data[(address) + 14], data[(address) + 15]);
        //     fprintf(control_unit_rom, "%s\n", buf);
        // }
        fclose(control_unit_rom);
    }

}
