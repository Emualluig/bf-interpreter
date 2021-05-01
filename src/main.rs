mod interpreter;

// notes: Brainfuck is referred to as bf for brevity
fn main() {
    let input: String = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."); 
    //let input: String = String::from("-[>[->]++[-<+]-]"); 
    //let input: String = String::from(">,[>,]<[.<]"); 

    interpreter::interpret(input, true);
}
