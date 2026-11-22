mod ops;

use std::{num::Wrapping, time::Instant};

use crate::ops::{Cell, Pointer};

fn main() {
    let start = Instant::now();
    let mut cells: [Cell; 30_000] = [Cell::new(Wrapping(0u8)); 30_000];

    // Init the pointer to 0
    let mut ptr_pos = Pointer::new(0); // first cell

    let program_str = include_str!("../test/bitwidth.b");
    let program = program_str.as_bytes();

    // loop index
    let mut i = 0usize;
    while i < program.len() {
        let curr_char = program[i] as char;
        let curr_cell = &mut cells[ptr_pos.cell_pos];
        match curr_char {
            '[' => {
                let mut open_brackets_count = 1u64;
                let mut close_bracket_pos = i;
                let remaining_program = &program_str[i + 1..]; // till the end
                for a in remaining_program.chars() {
                    if a == '[' {
                        open_brackets_count += 1;
                    } else if a == ']' {
                        open_brackets_count -= 1;
                    }
                    if open_brackets_count == 0 {
                        close_bracket_pos += 1;
                        break;
                    }
                    close_bracket_pos += 1;
                }

                let loopsnip = &program[(i + 1)..close_bracket_pos];
                while cells[ptr_pos.cell_pos].value != Wrapping(0u8) {
                    exec_body(loopsnip, &mut cells, &mut ptr_pos);
                }
                i = close_bracket_pos;
            }
            _ => parse_bf(program[i] as char, curr_cell, &mut ptr_pos),
        }
        i += 1;
    }
    let duration = Instant::now().duration_since(start);
    let formatted_duration = match duration.as_micros() {
        0..=1000 => format!("{}µs", duration.as_micros()),
        1001.. => format!("{}ms", duration.as_millis()),
    };
    println!("Ran in: {}", formatted_duration);
}

fn exec_body(body: &[u8], cells: &mut [Cell], ptr_pos: &mut Pointer) {
    let mut position = 0;
    while position < body.len() {
        if body[position] == b'[' {
            let mut nested_depth = 1;
            let mut close = position + 1;
            while nested_depth > 0 {
                match body[close] {
                    b'[' => nested_depth += 1,
                    b']' => nested_depth -= 1,
                    _ => (),
                }
                close += 1;
            }
            while cells[ptr_pos.cell_pos].value != Wrapping(0u8) {
                exec_body(&body[position + 1..close - 1], cells, ptr_pos);
            }
            position = close;
        } else {
            parse_bf(
                body[position] as char,
                &mut cells[ptr_pos.cell_pos],
                ptr_pos,
            );
            position += 1;
        }
    }
}

fn parse_bf(symb: char, curr_cell: &mut Cell, ptr_pos: &mut Pointer) {
    let ptr_pos_lock = ptr_pos.cell_pos;
    match symb {
        '+' => {
            curr_cell.increase();
        }
        '-' => {
            curr_cell.decrease();
        }
        '.' => {
            print!("{}", curr_cell.print());
        }
        ',' => {
            let _ = &curr_cell.read();
        }
        '<' => {
            let previous_cell = ptr_pos_lock
                .checked_sub(1)
                .expect("Data pointer moved to invalid left cell");
            Pointer::move_ptr(ptr_pos, previous_cell);
        }
        '>' => {
            Pointer::move_ptr(ptr_pos, ptr_pos_lock + 1);
        }
        _ => (),
    };
}

#[cfg(test)]
mod tests {
    use std::num::Wrapping;

    use crate::{
        exec_body, main,
        ops::{self, Pointer},
    };
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

    #[test]
    fn test_loop_executes_entire_body() {
        let mut cells = [ops::Cell::new(Wrapping(0u8)); 2];
        let mut ptr_pos = Pointer::new(0);

        exec_body(b"+++[>++<-]", &mut cells, &mut ptr_pos);

        assert_eq!(cells[0].value, Wrapping(0u8));
        assert_eq!(cells[1].value, Wrapping(6u8));
    }

    #[test]
    fn test_nested_loops() {
        let mut cells = [ops::Cell::new(Wrapping(0u8)); 3];
        let mut ptr_pos = Pointer::new(0);

        exec_body(b"++[>++[>+<-]<-]", &mut cells, &mut ptr_pos);

        assert_eq!(cells[0].value, Wrapping(0u8));
        assert_eq!(cells[1].value, Wrapping(0u8));
        assert_eq!(cells[2].value, Wrapping(4u8));
    }

    #[test]
    fn test_main() {
        main();
    }
}
