//! # AetherVM Exception Handling Module
//!
//! This module defines custom exceptions for the AetherVM environment, providing detailed error reporting
//! for issues encountered during execution. Exceptions are represented as variants of the [Exception] enum,
//! each tailored to a specific type of error, such as invalid memory access, illegal arguments, or unexpected
//! opcodes. These exceptions implement the **`Debug`** and **`Display`** traits to enable clear error descriptions
//! and easy debugging.
//!
use std::fmt::{Debug, Display, Formatter};

use crate::hardware::memory::{MEMORY_START_ADDRESS, MEMORY_SIZE};

/// Enum representing various exceptions that can occur in the AetherVM.
///
/// The [Exception] enum provides specific error cases for common problems encountered during execution, such as:
/// - Memory access violations.
/// - Illegal size arguments.
/// - Unexpected conditions or opcodes.
#[derive(Debug)]
pub enum Exception {
    /// Raised when an address is accessed that falls outside the defined memory bounds.
    AddressNotInMemoryBounds(u64),
    /// Raised when an invalid size argument is provided for memory access or other operations.
    IllegalSizeArgument(usize),
    /// Raised when attempting to access an instruction at an invalid or out-of-bounds address.
    InstructionAccessFaultOnAddress(u64),
    /// Raised when an unexpected condition is encountered during conditional execution.
    UnexpectedCondition(u64),
    /// Raised when an unrecognized opcode is encountered during instruction decoding.
    UnexpectedOpcode(u64)
}
impl Display for Exception {
    /// Provides a human-readable description of each exception.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Exception::AddressNotInMemoryBounds(addr) => write!(f, "Address {} is not in memory bounds ({}..{})", addr, MEMORY_START_ADDRESS, (MEMORY_START_ADDRESS + MEMORY_SIZE)),
            Exception::IllegalSizeArgument(size) => write!(f, "Size {} is not allowed. Size can be one of that: 8, 16, 32, 64", size),
            Exception::InstructionAccessFaultOnAddress(addr) => write!(f, "No instructions on address {} or address out of bounds", addr),
            Exception::UnexpectedCondition(cond) => write!(f, "Unexpected condition {:X}", cond),
            Exception::UnexpectedOpcode(opcode) => write!(f, "Unexpected opcode {:X}", opcode)
        }
    }
}


