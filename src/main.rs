mod ops;

use std::num::Wrapping;

use crate::ops::Cell;

fn main() {
    // I know 30,000 cells is the standard, but testing with fewer seems better
    // Using 50 cells for now
    let mut cells: [Cell; 50] = [Cell::new(Wrapping(0), 0); 50];

    // Hardcoding the program for now
    // This is a program that prints "Hello World"
    let program = String::from(
        "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.",
    );
}
