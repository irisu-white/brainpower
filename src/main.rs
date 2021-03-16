use std::iter;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct BrainFuck {
    // instruction
    instruction: Vec<char>,
    // dynamic brainfuck array
    // default length is 0x100
    array: Vec<i8>,
    // register
    pc: usize,
    sp: usize,
}

impl BrainFuck {
    fn new(inst: &str) -> BrainFuck {
        let s: Vec<char> = inst.chars().filter(|c| match c {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
            _ => false,
        }).collect();

        if cfg!(debug_assertions) {
            println!("init data:");
            for c in &s {
                print!("{}", c);
            }
            println!("\ndata len {}", s.len());
        }

        BrainFuck {
            instruction: s,
            array: Vec::new(),
            pc: 0,
            sp: 0,
        }
    }

    fn run(&mut self) -> &Vec<i8> {
        self.pc = 0;
        self.sp = 0;
        self.array = vec![0; 100];

        let mut input = io::stdin();
        let mut output = io::stdout();
        let stop = self.instruction.len();

        loop {
            let curr = self.instruction[self.pc];

            if cfg!(debug_assertions) {
                println!("PC: {}, Inst: {}", self.pc, curr);
            }

            match curr {
                '>' => {
                    // max size of array is 0x7500 (29952)
                    if (self.sp + 1) >= 0x7500 {
                        panic!("Fuck: stack out of range (max 0x7500)");
                    }
                    // extend array
                    if (self.sp + 1) >= self.array.len() {
                        self.array.extend(iter::repeat(0).take(0x100));
                    }
                    self.sp += 1;
                },
                '<' => {
                    // bound check
                    if self.sp == 0 {
                        panic!("Fuck: stack less then zero");
                    }
                    self.sp -= 1;
                },
                '+' => {
                    self.array[self.sp] += 1;
                },
                '-' => {
                    self.array[self.sp] -= 1;
                },
                '.' => {
                    let s: u8 = self.array[self.sp] as u8;
                    output.write(&[s]).expect("Fuck: output error");
                },
                ',' => {
                    let mut s: [u8; 1] = [0];
                    input.read(&mut s).expect("Fuck: input error");
                    self.array[self.sp] = s[0] as i8;
                },
                '[' => {
                    if self.array[self.sp] == 0 {
                        if self.pc + 1 >= stop {
                            panic!("Fuck: invalid '[' at {}", self.pc);
                        }
                        // find forward ']'
                        let mut count: i32 = 1;
                        for i in (self.pc + 1)..stop {
                            let c = self.instruction[i];
                            if c == '[' {
                                count += 1
                            }
                            else if c == ']' {
                                count -= 1;
                                if count == 0 {
                                    if cfg!(debug_assertions) {
                                        println!("PC forward to {}", i);
                                    }
                                    self.pc = i;
                                    break;
                                }
                            }
                        }
                        // not found
                        if count != 0 {
                            panic!("Fuck: invalid '[' at {}", self.pc);
                        }
                    }
                },
                ']' => {
                    if self.array[self.sp] != 0 {
                        if self.pc == 0 {
                            panic!("Fuck: invalid ']' at 0");
                        }
                        // find back '['
                        let mut count: i32 = 1;
                        for i in (0..(self.pc - 1)).rev() {
                            let c = self.instruction[i];
                            if c == '[' {
                                count -= 1;
                                if count == 0 {
                                    if cfg!(debug_assertions) {
                                        println!("PC back to {}", i);
                                    }
                                    self.pc = i;
                                    break;
                                }
                            }
                            else if c == ']' {
                                count += 1;
                            }
                        }
                        if count != 0 {
                            panic!("Fuck: invalid ']' at {}", self.pc);
                        }
                    }
                },
                s => {
                    panic!("Fuck: Unknown char {} at {}", s, self.pc);
                }
            }

            // next instruction
            self.pc += 1;
            if self.pc >= stop {
                break;
            }
        }

        // return restul stack
        &self.array
    }
}

fn main() {
    let s = r#"
[ This program prints "Hello World!" and a newline to the screen, its
  length is 106 active command characters. [It is not the shortest.]

  This loop is an "initial comment loop", a simple way of adding a comment
  to a BF program such that you don't have to worry about any command
  characters. Any ".", ",", "+", "-", "<" and ">" characters are simply
  ignored, the "[" and "]" characters just have to be balanced. This
  loop and the commands it contains are ignored because the current cell
  defaults to a value of 0; the 0 value causes this loop to be skipped.
]
++++++++               Set Cell #0 to 8
[
    >++++               Add 4 to Cell #1; this will always set Cell #1 to 4
    [                   as the cell will be cleared by the loop
        >++             Add 2 to Cell #2
        >+++            Add 3 to Cell #3
        >+++            Add 3 to Cell #4
        >+              Add 1 to Cell #5
        <<<<-           Decrement the loop counter in Cell #1
    ]                   Loop till Cell #1 is zero; number of iterations is 4
    >+                  Add 1 to Cell #2
    >+                  Add 1 to Cell #3
    >-                  Subtract 1 from Cell #4
    >>+                 Add 1 to Cell #6
    [<]                 Move back to the first zero cell you find; this will
                        be Cell #1 which was cleared by the previous loop
    <-                  Decrement the loop Counter in Cell #0
]                       Loop till Cell #0 is zero; number of iterations is 8

The result of this is:
Cell No :   0   1   2   3   4   5   6
Contents:   0   0  72 104  88  32   8
Pointer :   ^

>>.                     Cell #2 has value 72 which is 'H'
>---.                   Subtract 3 from Cell #3 to get 101 which is 'e'
+++++++..+++.           Likewise for 'llo' from Cell #3
>>.                     Cell #5 is 32 for the space
<-.                     Subtract 1 from Cell #4 for 87 to give a 'W'
<.                      Cell #3 was set to 'o' from the end of 'Hello'
+++.------.--------.    Cell #3 for 'rl' and 'd'
>>+.                    Add 1 to Cell #5 gives us an exclamation point
>++.                    And finally a newline from Cell #6
"#;

    let mut bf = BrainFuck::new(s);

    bf.run();

    println!("It works!");
}
