// Please visit https://brainfuck.org for a more detailed explanation
// In brainfuck, there are 8 kinds of instructions

// + is used to increment the current cell value by 1. Overflow is allowed
// - is used to decrement the current cell value by 1. Overflow is allowed

// . is used to print the current cell value in ascii to stdout
// , is used to read an ascii character from stdin and store it as a number in the current cell

// > is used to move the pointer one cell to the right. Undefined if already on the last cell
// < is used to move the pointer one cell to the left. Undefined if already on the first cell
// NOTE: The code will panic! here as rust does not have null values

// [ is used to start a loop, explained below
// ] is used to close a loop
// A loop keeps running the code between [ ] until the current cell value is 0

use std::{io::stdin, num::Wrapping};

fn as_ascii(num: Wrapping<u8>) -> char {
    num.0 as char
}

fn as_int(char: char) -> Wrapping<u8> {
    match char.is_ascii() {
        true => Wrapping(char as u8),
        false => panic!("ERROR: the value read is not a valid ascii"),
    }
}
/// A struct that represents a cell. Owns an 8-bit digit indicating its value and a u64 indicating its position in the cell list
#[derive(Clone, Copy)]
pub struct Cell {
    /**
    An 8-bit integer, can be wrapped
    */
    pub value: Wrapping<u8>,
    /**
    This Cell's position, 64 bits
    */
    pub pos: u64,
}

impl Cell {
    /// Create a new Cell
    pub fn new(value: Wrapping<u8>, pos: u64) -> Cell {
        Cell { value, pos }
    }

    /// Increase Cell value by 1, returns the updated value
    /// NO OVERFLOW!!! (for now)
    pub fn increase(&mut self) -> Wrapping<u8> {
        self.value += 1;
        self.value
    }

    /// Decrease Cell value by 1, returns the updated value
    /// NO OVERFLOW!!! (for now)
    pub fn decrease(&mut self) -> Wrapping<u8> {
        self.value -= 1;
        self.value
    }

    /// Print the current value in ascii, returns the parsed char
    pub fn print(&self) -> char {
        as_ascii(self.value)
    }

    /// Get a value from stdin as ascii. Replace the Cell value, returns the parsed number
    pub fn read(&mut self) -> Wrapping<u8> {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let buf = buf.trim().chars().next().unwrap();
        as_int(buf)
    }
}

/// Represents the pointer. Owns a u64 indicating the current cell number.
/// "current" means the cell the pointer is at.
pub struct Pointer {
    pub cell_pos: usize,
}

impl<'a> Pointer {
    pub fn new(cell_pos: usize) -> Pointer {
        Pointer { cell_pos }
    }
    /// Set the value of the current cell
    pub fn move_ptr(&mut self, cell_pos: usize) -> &mut Self {
        self.cell_pos = cell_pos;
        self
    }
}
