//! # AetherVM CPU Emulator
//! ***
//! This module implements a virtual CPU for the AetherVM, a virtual machine designed to execute custom bytecode efficiently.
//! It provides mechanisms for instruction loading, decoding, and execution, along with support for register manipulation
//! and memory interactions. The CPU operates on a set of registers and utilizes a memory bus to interact with external memory
//! and devices. This documentation outlines the key structures, methods, and functionalities provided by the [AVMCpu] implementation.

use std::collections::VecDeque;
use std::process::exit;
use crate::hardware::exceptions::Exception;
use crate::hardware::memory::{AVMBus, AVMDevice, MEMORY_SIZE, MEMORY_START_ADDRESS};

/// Represents the virtual CPU for AetherVM.
///
/// The [AVMCpu] structure encapsulates the core functionality of the AetherVM CPU, including:
/// - Registers: A fixed array of 32 64-bit general-purpose registers.
/// - Instruction Pointer (IP): A 64-bit value indicating the current execution address.
/// - Memory Bus: Facilitates memory access and device communication.
pub struct AVMCpu {
    bus: AVMBus,
    registers: [u64; 32],
    ip: u64,
}
impl AVMCpu {
    /// Creates a new instance of the AetherVM CPU.
    ///
    /// Initializes all registers to 0 and sets up the memory bus.
    /// Registers 12 and 13 are stack base pointer and stack pointer.
    ///
    /// # Returns
    /// A new [AVMCpu] instance.
    pub fn new() -> AVMCpu {
        let mut registers = [0; 32];
        registers[12] = MEMORY_START_ADDRESS + MEMORY_SIZE;
        registers[13] = MEMORY_START_ADDRESS + MEMORY_SIZE;
        Self {
            bus: AVMBus::new(),
            registers,
            ip: MEMORY_START_ADDRESS
        }
    }
    /// Loads the next instruction from memory.
    ///
    /// The instruction is read as a 64-bit value from the memory address pointed to by the instruction pointer (IP).
    ///
    /// # Returns
    /// - `Ok(u64)` if the instruction is successfully loaded.
    /// - `Err(Exception)` if there is an error during memory access.
    pub fn load_instr(&mut self) -> Result<u64, Exception> {
        match self.bus.read(self.ip, 64) {
            Ok(instr) => Ok(instr),
            Err(_) => Err(Exception::InstructionAccessFaultOnAddress(self.ip))
        }
    }
    /// Executes a given instruction.
    ///
    /// Decodes the instruction, performs the corresponding operation, and updates the instruction pointer.
    /// Supports various operations including register manipulation, arithmetic, jumps, and memory access.
    ///
    /// # Parameters
    /// - `instr`: A 64-bit value representing the instruction to execute.
    ///
    /// # Returns
    /// - `Ok(())` if the instruction executes successfully.
    /// - `Err(Exception)` if an error occurs (e.g., unexpected opcode or invalid arguments).
    pub fn execute_instr(&mut self, instr: u64) -> Result<(), Exception> {
        let operation = instr >> 48;
        let cda1 = ((instr >> 32) & 0x0000FF00) >> 8;
        let cda2 = (instr >> 32) & 0x000000FF;
        let cda3 = (instr >> 24) & 0x00000000FF;
        let eda1 = (instr >> 32) & 0x0000FFFF;
        let eda2 = (instr >> 16) & 0x00000000FFFF;
        let eda3 =  instr & 0x000000000000FFFF;
        let ota = instr & 0x00000000FFFFFFFF;
        match operation {
            0xCD00 => { //[regFrom] $move [regTo]
                let reg_from = cda1 as usize;
                let reg_to = cda2 as usize;
                self.registers[reg_to] = self.registers[reg_from];
                self.ip += 1;
                Ok(())
            }
            0xCD01 => { //[reg] $set [value]
                let reg = cda1 as usize;
                let value = eda2 | eda3 | (eda1 & 0xFF);
                self.registers[reg] = value;
                self.ip += 1;
                Ok(())
            }
            0xCF00 => { //$jmp [address]
                let address = eda1 | eda2 | eda3;
                self.ip = address;
                Ok(())
            }
            0xCF01 => { //[reg1] $jmc [cond] [reg2]
                let address = (eda1 | eda3 | eda2) & 0x000FFFFFFFFF;
                let condition = cda1 >> 4;
                let reg1 = (cda1 & 0x0F) as usize;
                let reg2 = (cda2 >> 4) as usize;

                match condition {
                    0xA => {
                        if self.registers[reg1] > self.registers[reg2] {
                            self.ip = address;
                            return Ok(())
                        }
                        Ok(())
                    }
                    0xB => {
                        if self.registers[reg1] < self.registers[reg2] {
                            self.ip = address;
                            return Ok(())
                        }
                        Ok(())
                    }
                    0xC => {
                        if self.registers[reg1] == self.registers[reg2] {
                            self.ip = address;
                            return Ok(())
                        }
                        Ok(())
                    }
                    0xD => {
                        if self.registers[reg1] >= self.registers[reg2] {
                            self.ip = address;
                            return Ok(())
                        }
                        Ok(())
                    }
                    0xE => {
                        if self.registers[reg1] <= self.registers[reg2] {
                            self.ip = address;
                            return Ok(())
                        }
                        Ok(())
                    }
                    _ => {
                        Err(Exception::UnexpectedCondition(condition))
                    }
                }
            }
            0xCD02 => { //[reg1] $add [reg2] [resReg]
                let reg1 = cda1 as usize;
                let reg2 = cda2 as usize;
                let res_reg = cda3 as usize;
                self.registers[res_reg] = self.registers[reg1].wrapping_add(self.registers[reg2]);
                self.ip += 1;
                Ok(())

            }
            0xCD03 => { //[reg1] $sub [reg2] [resReg]
                let reg1 = cda1 as usize;
                let reg2 = cda2 as usize;
                let res_reg = cda3 as usize;
                self.registers[res_reg] = self.registers[reg1].wrapping_sub(self.registers[reg2]);
                self.ip += 1;
                Ok(())

            }
            0xCD04 => { //[reg1] $mul [reg2] [resReg]
                let reg1 = cda1 as usize;
                let reg2 = cda2 as usize;
                let res_reg = cda3 as usize;
                self.registers[res_reg] = self.registers[reg1].wrapping_mul(self.registers[reg2]);
                self.ip += 1;
                Ok(())

            }
            0xCD05 => { //[reg1] $div [reg2] [resReg]
                let reg1 = cda1 as usize;
                let reg2 = cda2 as usize;
                let res_reg = cda3 as usize;
                self.registers[res_reg] = self.registers[reg1].wrapping_div(self.registers[reg2]);
                self.ip += 1;
                Ok(())

            }
            0xCD06 => { //[address] $move [reg]
                let address = eda2 | eda3;
                let reg = cda1 as usize;
                let size = cda2 as usize;
                if let Ok(val) = self.bus.read(address, size) {
                    self.registers[reg] = val;
                    self.ip += 1;
                    Ok(())
                } else {
                    Err(Exception::IllegalSizeArgument(size))
                }
            }
            0xCD07 => { //[reg] $move [address]
                let address = eda2 | eda3;
                let reg = cda1 as usize;
                let size = cda2 as usize;
                if let Ok(addr) = self.bus.write(address, self.registers[reg], size) {
                    self.ip += 1;
                    Ok(())
                } else {
                    Err(Exception::IllegalSizeArgument(size))
                }
            }
            0xFFFF => { //$halt
                exit(0)
            }
            0xFFFA => { //$exit [code] / $quit [code]
                let code = cda1 as i32;
                exit(code)
            }
            _ => {
                Err(Exception::UnexpectedOpcode(operation))
            }

        }
    }
}
