mod ops;

use core::panic;
use std::num::Wrapping;

use crate::ops::{Cell, Pointer};

fn main() {
    // I know 30,000 cells is the standard, but testing with fewer seems better
    // Using 50 cells for now
    let mut cells: [Cell; 50] = [Cell::new(Wrapping(0u8)); 50];

    // Init the pointer to 0
    let mut ptr_pos = Pointer::new(0); // first cell

    // Hardcoding the program for now
    // This is a program that prints "Hello World"
    let program = String::from(
        "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.",
    );

    let mut curr_cell = &mut cells[ptr_pos.cell_pos as usize];
    for (i, c) in program.chars().enumerate() {
        match c {
            '[' => {
                let remaining = &program[(i + 1)..match &program.find(']') {
                    Some(n) => *n,
                    None => panic!("Closing ] not found!"),
                }]; // i+1 to not get the [ (i = '[')
                let mut remaining: Vec<&str> = remaining.split("").collect();
                remaining.reverse();

                while curr_cell.value != Wrapping(0u8) {
                    parse_bf(c, &mut curr_cell, &mut ptr_pos);
                    remaining.pop();
                }
            }
            ']' => {
                continue;
            }
            _ => {
                parse_bf(c, &mut curr_cell, &mut ptr_pos);
                continue;
            }
        };
    }
}

fn parse_bf(symb: char, curr_cell: &mut Cell, mut ptr_pos: &mut Pointer) {
    let ptr_pos_lock = ptr_pos.cell_pos;
    let a = match symb {
        '+' => {
            curr_cell.increase();
        }
        '-' => {
            curr_cell.decrease();
        }
        '.' => {
            println!("{}", &curr_cell.print());
        }
        ',' => {
            let _ = &curr_cell.read();
        }
        '<' => {
            Pointer::move_ptr(&mut ptr_pos, (ptr_pos_lock - 1) as usize);
        }
        '>' => {
            Pointer::move_ptr(&mut ptr_pos, (ptr_pos_lock + 1) as usize);
        }
        _ => (),
    };
    a
}
