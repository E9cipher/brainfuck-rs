mod ops;

use std::num::Wrapping;

use crate::ops::{Cell, Pointer};

fn main() {
    // I know 30,000 cells is the standard, but testing with fewer seems better
    // Using 50 cells for now
    let mut cells: [Cell; 50] = std::array::from_fn(|i| Cell::new(Wrapping(0), i as u64));

    // Init the pointer to 0
    let mut ptr_pos = Pointer::new(0); // first cell
    let ptr_pos_lock = ptr_pos.cell_pos;

    // Hardcoding the program for now
    // This is a program that prints "Hello World"
    let program = String::from(
        "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.",
    );

    let curr_cell = &mut cells[ptr_pos.cell_pos as usize];
    for c in program.chars() {
        let a = match c {
            '+' => &curr_cell.increase(),
            '-' => &curr_cell.decrease(),
            '.' => {
                println!("{}", &curr_cell.print());
                continue;
            }
            ',' => &curr_cell.read(),
            '<' => {
                Pointer::move_ptr(&mut ptr_pos, (ptr_pos_lock - 1) as usize);
                continue;
            }
            '>' => {
                Pointer::move_ptr(&mut ptr_pos, (ptr_pos_lock + 1) as usize);
                continue;
            }
            '[' => {
                todo!("Not impled yet");
            }
            ']' => {
                todo!("Not impled yet");
            }
            _ => continue,
        };
    }
}
