//! # AetherVM Memory Management Module
//!
//! This module implements the memory management system for the AetherVM. It provides an abstraction layer for memory access
//! and manipulation via the [AVMBus] and [AVMMemory] structures. The memory is organized with a starting address and a fixed
//! size, and it supports both byte-level and word-level read/write operations. The module ensures safety by validating memory
//! bounds and size arguments before performing operations.
//!
use crate::hardware::exceptions::Exception;

/// The starting address of the AetherVM memory space.
pub const MEMORY_START_ADDRESS: u64 = 0x10000000;
/// The size of the AetherVM memory space.
pub const MEMORY_SIZE: u64 = 0x40000000;

/// Trait for AetherVM devices to standardize memory read/write operations.
pub trait AVMDevice {
    /// Writes data to a specific address in memory.
    ///
    /// # Parameters
    /// - `addr`: The memory address to write to.
    /// - `data`: The data to write.
    /// - `size`: The size of the data in bits (8, 16, 32, or 64).
    ///
    /// # Returns
    /// - `Ok(u64)`: The resulting memory address after the write operation.
    /// - `Err(Exception)`: An exception if the operation fails.
    fn write(&mut self, addr: u64, data: u64, size: usize) -> Result<u64, Exception>;
    /// Reads data from a specific address in memory.
    ///
    /// # Parameters
    /// - `addr`: The memory address to read from.
    /// - `size`: The size of the data in bits (8, 16, 32, or 64).
    ///
    /// # Returns
    /// - `Ok(u64)`: The data read from memory.
    /// - `Err(Exception)`: An exception if the operation fails.
    fn read(&mut self, addr: u64, size: usize) -> Result<u64, Exception>;
}

/// Represents the memory bus, which interfaces between the CPU and memory devices.
pub struct AVMBus {
    memory: AVMMemory
}
impl AVMBus {
    /// Creates a new instance of the memory bus.
    ///
    /// # Returns
    /// A new [AVMBus] instance with initialized memory.
    pub fn new() -> AVMBus {
        Self {
            memory: AVMMemory::new()
        }
    }
}
impl AVMDevice for AVMBus {
    fn write(&mut self, addr: u64, data: u64, size: usize) -> Result<u64, Exception> {
        if addr < MEMORY_START_ADDRESS || addr > (MEMORY_START_ADDRESS + MEMORY_SIZE) {
            return Err(Exception::AddressNotInMemoryBounds(addr))
        }
        self.memory.write(addr, data, size)
    }
    fn read(&mut self, addr: u64, size: usize) -> Result<u64, Exception> {
        if addr < MEMORY_START_ADDRESS || addr > (MEMORY_START_ADDRESS + MEMORY_SIZE) {
            return Err(Exception::AddressNotInMemoryBounds(addr))
        }
        self.memory.read(addr, size)
    }
}

/// Represents the physical memory of the AetherVM.
pub struct AVMMemory {
    data: Vec<u8>
}
impl AVMMemory {
    /// Creates a new instance of memory with a predefined capacity.
    ///
    /// # Returns
    /// A new [AVMMemory] instance.
    pub fn new() -> AVMMemory {
        Self {
            data: Vec::with_capacity(1073741824)
        }
    }
    /// Reads a single byte from memory.
    fn load_byte(&self, addr: u64) -> u64 {
        let index = (addr - MEMORY_START_ADDRESS) as usize;
        self.data[index] as u64
    }
    /// Reads a 16-bit word from memory.
    fn load_short(&self, addr: u64) -> u64 {
        let index = (addr - MEMORY_START_ADDRESS) as usize;
        (self.data[index] as u64) | ((self.data[index + 1] as u64) << 8)
    }
    /// Reads a 32-bit word from memory.
    fn load_int(&self, addr: u64) -> u64 {
        let index = (addr - MEMORY_START_ADDRESS) as usize;
        self.int_load_util(index)
    }
    /// Reads a 64-bit word from memory.
    fn load_long(&self, addr: u64) -> u64 {
        let index = (addr - MEMORY_START_ADDRESS) as usize;
        self.int_load_util(index)
            | ((self.data[index + 4] as u64) << 32)
            | ((self.data[index + 5] as u64) << 40)
            | ((self.data[index + 6] as u64) << 48)
            | ((self.data[index + 7] as u64) << 56)
    }
    fn int_load_util(&self, index: usize) -> u64 {
        (self.data[index] as u64)
            | ((self.data[index + 1] as u64) << 8)
            | ((self.data[index + 2] as u64) << 16)
            | ((self.data[index + 3] as u64) << 24)
    }

    /// Writes a single byte to memory.
    fn write_byte(&mut self, addr: u64, val: u64) -> u64 {
        let index = (addr - MEMORY_START_ADDRESS) as usize;
        self.data[index] = val as u8;
        index as u64
    }
    /// Writes a 16-bit word to memory.
    fn write_short(&mut self, addr: u64, val: u64) -> u64 {
        let index = (addr - MEMORY_START_ADDRESS) as usize;
        self.data[index] = (val & 0xFF) as u8;
        self.data[index+1] = ((val >> 8) & 0xFF) as u8;
        index as u64
    }
    /// Writes a 32-bit word to memory.
    fn write_int(&mut self, addr: u64, val: u64) -> u64 {
        let index = (addr - MEMORY_START_ADDRESS) as usize;
        self.data[index] = (val & 0xFF) as u8;
        self.data[index + 1] = ((val >> 8) & 0xFF) as u8;
        self.data[index + 2] = ((val >> 16) & 0xFF) as u8;
        self.data[index + 3] = ((val >> 24) & 0xFF) as u8;
        index as u64
    }
    /// Writes a 64-bit word to memory.
    fn write_long(&mut self, addr: u64, val: u64) -> u64 {
        let index = (addr - MEMORY_START_ADDRESS) as usize;
        self.data[index] = (val & 0xFF) as u8;
        self.data[index + 1] = ((val >> 8) & 0xFF) as u8;
        self.data[index + 2] = ((val >> 16) & 0xFF) as u8;
        self.data[index + 3] = ((val >> 24) & 0xFF) as u8;
        self.data[index + 4] = ((val >> 32) & 0xFF) as u8;
        self.data[index + 5] = ((val >> 40) & 0xFF) as u8;
        self.data[index + 6] = ((val >> 48) & 0xFF) as u8;
        self.data[index + 7] = ((val >> 56) & 0xFF) as u8;
        index as u64
    }
}
impl AVMDevice for AVMMemory {
    fn write(&mut self, addr: u64, data: u64, size: usize) -> Result<u64, Exception> {
        match size {
            8 => {
                Ok(self.write_byte(addr, data))
            }
            16 => {
                Ok(self.write_short(addr, data))
            }
            32 => {
                Ok(self.write_int(addr, data))
            }
            64 => {
                Ok(self.write_long(addr, data))
            }
            _ => {
                Err(Exception::IllegalSizeArgument(size))
            }
        }
    }
    fn read(&mut self, addr: u64, size: usize) -> Result<u64, Exception> {
        match size {
            8 => {
                Ok(self.load_byte(addr))
            }
            16 => {
                Ok(self.load_short(addr))
            }
            32 => {
                Ok(self.load_int(addr))
            }
            64 => {
                Ok(self.load_long(addr))
            }
            _ => {
                Err(Exception::IllegalSizeArgument(size))
            }
        }
    }
}