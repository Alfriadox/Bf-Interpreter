use std::fs::File;
use std::env;
use std::io::{self, Read, Write};

/*
      BrainFuck Interpreter  Syntax:
    >           Move to the next cell
    <           Move to the previous cell
    +           Add 1 to the current cell
    -           Subtract 1 from the current cell
    .           Print the cell's Unicode Character
    ,           Input one unicode character from STDIN
    [           Create a checkpoint
    ]           If the current cells value is not 0, go back to the checkpoint.
*/

fn main() {
  let in_src: Vec<String> = env::args().collect();
  if in_src[1] == "-" {
    println!("Repl not implemented!");
    panic!();
  } else {
    let mut file = match File::open(&in_src[1]) {
      Ok(file) => file,
      Err(e) => {
        panic!(e);
      },
    };
    let mut contents_s = String::new();
    match file.read_to_string(&mut contents_s) {
      Ok(a) => a,
      Err(e) => {
        panic!(e);
      },
    };
    let mut contents = vec![];
    for c in contents_s.as_bytes() {
      contents.push(*c as char);
    }
    let mut read_head: usize = 0;
    let mut checkpoint: Vec<usize> = vec![0];
    let mut stack: Vec<isize> = vec![0];
    let mut loc: usize = 0;
    loop {
      match contents[read_head] {
        '+' => {
          stack[loc] += 1;
        },
        '-' => {
          stack[loc] -= 1;
        },
        '>' => {
          loc += 1;
          if loc >= stack.len() {
            stack.push(0);
          }
        },
        '<' => {
          if loc == 0 {
            panic!("Cannot point to pre-zero stack location");
          } else {
            loc -= 1;
          }
        },
        '.' => {
          let c: u8 = (stack[loc] as i8) as u8;
          print!("{}", c as char);
          io::stdout().flush().unwrap();
        },
        ',' => {
          let mut swap: [u8; 1] = [0];
          io::stdin().read(&mut swap).unwrap();
          stack[loc] = swap[0] as isize;
          swap = [0];
        },
        '[' => {
          checkpoint.push(read_head);
          if stack[loc] == 0 {
            let mut parenlevel = 1;
            while parenlevel > 0 {
              read_head += 1;
              if contents[read_head] == '[' {
                parenlevel += 1;
              } else if contents[read_head] == ']' {
                parenlevel -= 1;
              }
            }
          }
        },
        ']' => {
          if stack[loc] == 0 {
            checkpoint.pop();
          } else {
            read_head = checkpoint[checkpoint.len()-1];
          }
        },
        _ => {},
      }
      read_head += 1;
      if read_head == contents.len() {
        break;
      }
    }
  }
}