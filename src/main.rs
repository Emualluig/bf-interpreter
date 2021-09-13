use std::{collections::VecDeque, env, fs, io::Read, usize};

const ARRAY_SIZE: usize = 5000;

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

// create_instructions(input) returns a vector of instructions
// requires: input is valid brainfuck
// notes: 
// time: O(n) where n is the length of input
fn create_instructions(input: String) -> Vec<Instruction> {
    let mut instruction_vector: Vec<Instruction> = Vec::new();

    let mut start_loop_positions: VecDeque<usize> = VecDeque::new();

    let mut index = 0;

    // read through chars and add instruction to vector
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
                start_loop_positions.push_back(index); // Remember starting point of loop
                instruction_vector.push(Instruction::StartLoop { jump: 0 }); // Placeholder value
            },
            ']' => {
                // Will panic on invalid instructions
                let start_loop = start_loop_positions.pop_back().unwrap();

                // Add EndLoop that jumps to the last open loop 
                instruction_vector.push(Instruction::EndLoop { jump: start_loop });

                // Set the StartLoop to have the correct jump
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

// interpreter(instructions) consumes a vector of instructions and returns
//      the tape after the instructions are terminated
// requires: instructions is valid brainfuck
// notes:
// time: O() is the O() of the given instructions
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
                // If current value on tape is 0, jump to end of loop
                if tape[tape_pointer] == 0 {
                    instruction_index = jump;
                }
            },
            Instruction::EndLoop { jump } => {
                // If current value on tape is not 0, jump to start of loop
                if tape[tape_pointer] != 0 {
                    instruction_index = jump;
                }
            },
            Instruction::Read => {
                let read_bytes = std::io::stdin().bytes().next();

                // handle bytes
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

        // Move to next instruction
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
    assert_eq!(test1[2], 72); // Test only one value, for now

    let test2 = interpreter(create_instructions(succ_string_2));
    assert_eq!(test2[2], 87); // Test only one value, for now
    
    let test3 = interpreter(create_instructions(succ_string_3));
    assert_eq!(test3[4], 50); // Test only one value, for now
}

#[test]
fn read_tests() {
    let reverser = String::from(">,[>,]<[.<]");
    let test_reverser = interpreter(create_instructions(reverser));
    print!("{:?}", test_reverser);

    let echo = String::from(",[.,]");
    let test_echo = interpreter(create_instructions(echo));
    print!("{:?}", test_echo);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = args[1].clone();

    // Runs the file code.bf
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    interpreter(create_instructions(contents));
}
