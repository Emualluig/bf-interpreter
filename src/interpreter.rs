use core::panic;
use std::{collections::VecDeque, io::Read};

// Customize max size of byte_array
const ARRAY_SIZE: usize = 32;

// validate(input) returns true if input is valid bf
// requires: input is String
// effects: dynamic sizing deque
// note: currently only checks if brackets are balanced
pub fn validate(input: String) -> bool {
    let instruction_vector: Vec<char> = input.chars().collect();

    let mut instruction_scope: VecDeque<char> = VecDeque::new();
    let mut valid: bool = true;

    for instruction in instruction_vector {
        match instruction {
            '+' => {},
            '-' => {},
            '>' => {},
            '<' => {},
            '[' => {
                instruction_scope.push_back('[');
            },
            ']' => {
                if instruction_scope.is_empty() {
                    valid = false;
                    break;
                } else {
                    instruction_scope.pop_back();
                }
            },
            ',' => {},
            '.' => {},
            _ => {
                valid = false;
                break;
            },
        }
    }

    if !instruction_scope.is_empty() {
        valid = false; 
    }

    return valid;
}

// interpret(input, slient_mode) interprets input as bf code, returns byte_array on termination
// requires: input is String, slient_mode is bool
// effects: can produce output, dynamic sizing deque
// notes: size of array is fixed, values in cells are not negative,
//        strict bf code, only +-><[]., are allowed
pub fn interpret(input: String, slient_mode: bool) -> [u8; ARRAY_SIZE] {
    assert!(validate(input.clone())); // Validate bf code

    let mut byte_array: [u8; ARRAY_SIZE] = [0_u8; ARRAY_SIZE];

    let instruction_vector: Vec<char> = input.chars().collect();
    let instruction_length = instruction_vector.len();

    let mut instruction_index: usize = 0;
    let mut tape_index: usize = 0;

    let mut instruction_frame: VecDeque<usize> = VecDeque::new();

    while instruction_index < instruction_length {

        match instruction_vector[instruction_index] {
            '+' => {
                byte_array[tape_index] = byte_array[tape_index].wrapping_add(1);
            },
            '-' => {
                byte_array[tape_index] = byte_array[tape_index].wrapping_sub(1);
            },
            '>' => {
                if tape_index >= ARRAY_SIZE + 1 {
                    panic!("TAPE INDEX TOO LARGE");
                }

                tape_index += 1;
            },
            '<' => {
                if tape_index == 0 {
                    panic!("TAPE INDEX TOO SMALL");
                }

                tape_index -= 1;
            },
            '[' => {
                if byte_array[tape_index] == 0 {
                    while instruction_vector[instruction_index] != ']' {
                        instruction_index += 1;
                    }
                } else {
                    instruction_frame.push_back(instruction_index);
                }
            },
            ']' => {
                if byte_array[tape_index] != 0 {
                    instruction_index = *instruction_frame.back().unwrap(); // No need to handle unbalanced brackets
                } else {
                    instruction_frame.pop_back();
                }
            },
            ',' => {
                let read_bytes = std::io::stdin().bytes().next();

                match read_bytes {
                    Some(result_bytes) => {
                        // Handle character or error
                        match result_bytes {
                            Ok(13) => { // Skip ASCII 13
                                continue;
                            }
                            Ok(10) => { // Consider ASCII 10 to be EOF
                                byte_array[tape_index] = 0;
                            }
                            Ok(result_bytes) => { // charcters
                                byte_array[tape_index] = result_bytes;
                            },
                            Err(e) => { // Some kind of error reading bytes
                                panic!("Error reading bytes {:?}", e);
                            },
                        }
                    },
                    None => {
                        byte_array[tape_index] = 0;
                    },
                }
            },
            '.' => {
                print!("{}", byte_array[tape_index] as char);
            },
            _ => panic!("UNKNOWN INSTRUCTION"),
        }

        // Move to next instruction
        instruction_index += 1;

        // If on slient_mode, do not print
        if !slient_mode {
            print_byte_array(byte_array);
        }
    }

    return byte_array;
}

// print_byte_array(byte_array) prints byte_array
// requires: byte_array is an array
// effects: produces output
fn print_byte_array(byte_array: [u8; ARRAY_SIZE]) {
    for &i in byte_array.iter() {
        print!("|{i:>width$}|", i=i, width=3);
    }
    print!("\n");
}