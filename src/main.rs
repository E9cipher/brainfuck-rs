mod ops;

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
    let program_str = String::from(
        "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.",
    );
    let program = program_str.as_bytes();

    // loop index
    let mut i = 0usize;
    loop {
        let mut curr_cell = &mut cells[ptr_pos.cell_pos];
        let curr_char = program[i] as char;
        match curr_char {
            '[' => {
                let mut open_brackets_count = 1u64;
                let mut close_bracket_pos = i;
                let remaining_program = &program_str[i+1..]; // till the end
                for a in remaining_program.chars() {
                    if a == '[' {
                        open_brackets_count += 1;
                    } else if a == ']' {
                        open_brackets_count -= 1;
                    }
                    if open_brackets_count == 0 {
                        return;
                    }
                    close_bracket_pos += 1;
                }

                let loopsnip = &program[(i + 1)..close_bracket_pos];
                while curr_cell.value != Wrapping(0u8) {
                    parse_bf(program[i + 1] as char, curr_cell, &mut ptr_pos);
                }
            }
            _ => parse_bf(program[i] as char, &mut curr_cell, &mut ptr_pos),
        }
        i += 1;
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

#[cfg(test)]
mod tests {
    use std::num::Wrapping;

    use crate::ops;
    #[test]
    fn test_cell_increase() {
        let mut a = ops::Cell::new(std::num::Wrapping(0u8));
        a.increase();
        assert_eq!(a.value, Wrapping(1u8));
    }

    #[test]
    fn test_cell_decrease() {
        let mut a = ops::Cell::new(std::num::Wrapping(100u8));
        a.decrease();
        assert_eq!(a.value, Wrapping(99u8));
    }

    #[test]
    fn test_cell_wrap_increase() {
        let mut a = ops::Cell::new(std::num::Wrapping(0u8));
        let mut b = ops::Cell::new(std::num::Wrapping(255u8));
        a.decrease();
        b.increase();
        assert_eq!(a.value, Wrapping(255u8));
        assert_eq!(b.value, Wrapping(0u8));
    }
}
