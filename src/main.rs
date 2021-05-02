use std::{collections::VecDeque, io::Read, usize};

const ARRAY_SIZE: usize = 32;

enum Instruction {
    Increment,
    Decrement,
    Next,
    Previous,
    StartLoop { jump: usize },
    EndLoop { jump: usize },
    Read,
    Print,
}

fn create_instructions(input: String) -> Vec<Instruction> {
    let mut instruction_vector: Vec<Instruction> = Vec::new();

    let mut start_loop_positions: VecDeque<usize> = VecDeque::new();

    let mut index = 0;

    for charc in input.chars() {
        match charc {
            '+' => {
                instruction_vector.push(Instruction::Increment);
            },
            '-' => {
                instruction_vector.push(Instruction::Decrement);
            },
            '>' => {
                instruction_vector.push(Instruction::Next);
            },
            '<' => {
                instruction_vector.push(Instruction::Previous);
            },
            '[' => {
                start_loop_positions.push_back(index);
                instruction_vector.push(Instruction::StartLoop { jump: 0 });
            },
            ']' => {
                let start_loop = start_loop_positions.pop_back().unwrap();

                instruction_vector.push(Instruction::EndLoop { jump: start_loop });

                instruction_vector[start_loop] = Instruction::StartLoop { jump: index };
            },
            ',' => {
                instruction_vector.push(Instruction::Read);
            },
            '.' => {
                instruction_vector.push(Instruction::Print);
            },
            _ => { // Ignore comments
                continue;
            },
        }
        index += 1;
    }

    return instruction_vector;
}

fn interpreter(instructions: Vec<Instruction>) -> [u8; ARRAY_SIZE] {
    let mut tape = [0_u8; ARRAY_SIZE];
    let mut tape_pointer = 0;

    let mut instruction_index = 0;

    while instruction_index < instructions.len() {
        match instructions[instruction_index] {
            Instruction::Increment => {
                tape[tape_pointer] = tape[tape_pointer].wrapping_add(1);
            },
            Instruction::Decrement => {
                tape[tape_pointer] = tape[tape_pointer].wrapping_sub(1);
            },
            Instruction::Next => {
                tape_pointer += 1;
            },
            Instruction::Previous => {
                tape_pointer -= 1;
            },
            Instruction::StartLoop { jump } => {
                if tape[tape_pointer] == 0 {
                    instruction_index = jump;
                }
            },
            Instruction::EndLoop { jump } => {
                if tape[tape_pointer] != 0 {
                    instruction_index = jump;
                }
            },
            Instruction::Read => {
                let read_bytes = std::io::stdin().bytes().next();

                match read_bytes {
                    Some(result_bytes) => {
                        // Handle character or error
                        match result_bytes {
                            Ok(13) => { // Skip ASCII 13
                                continue;
                            }
                            Ok(10) => { // Consider ASCII 10 to be EOF
                                tape[tape_pointer] = 0;
                            }
                            Ok(result_bytes) => { // charcters
                                tape[tape_pointer] = result_bytes;
                            },
                            Err(e) => { // Some kind of error reading bytes
                                panic!("Error reading bytes {:?}", e);
                            },
                        }
                    },
                    None => {
                        tape[tape_pointer] = 0;
                    },
                }
            },
            Instruction::Print => {
                print!("{}",tape[tape_pointer] as char);
            },
        }
        instruction_index += 1;
    }

    return tape;
}

#[test]
fn success_tests() {
    if ARRAY_SIZE != 32 {
        panic!("NOT VALID TEST");
    }
    let succ_string_1 = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    let succ_string_2 = String::from(">++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<+++>]<<]>-----.>->+++..+++.>-.<<+[>[+>+]>>]<--------------.>>.+++.------.--------.>+.>+.");
    let succ_string_3 = String::from("+[>>>->-[>->----<<<]>>]>.---.>+..+++.>>.<.>>---.<<<.+++.------.<-.>>+.");

    let test1 = interpreter(create_instructions(succ_string_1));
    let answer1: [u8; ARRAY_SIZE] = [0, 0, 72, 100, 87, 33, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(test1, answer1);

    let test2 = interpreter(create_instructions(succ_string_2));
    let answer2: [u8; ARRAY_SIZE] = [72, 0, 87, 0, 100, 33, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(test2, answer2);
    
    let test3 = interpreter(create_instructions(succ_string_3));
    let answer3: [u8; ARRAY_SIZE] = [1, 0, 0, 255, 50, 255, 0, 55, 3, 0, 205, 164, 0, 52, 60, 0, 41, 0, 100, 108, 33, 44, 119, 200, 0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(test3, answer3);
}

fn main() {
    let reverser = String::from(">,[>,]<[.<]");
    let test_reverser = interpreter(create_instructions(reverser));
    print!("{:?}", test_reverser);

    let echo = String::from(",[.,]");
    let test_echo = interpreter(create_instructions(echo));
    print!("{:?}", test_echo);
}