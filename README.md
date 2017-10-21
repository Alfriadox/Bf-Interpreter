# BF-Interpreter
An interpreter for the esoteric language, "Brain Fuck," (their name, not mine) written in Rust.

Brain Fuck is an esoteric language based around using very few characters.

It works around a single expandable array (vector) of integers and uses 8 commands.

  \`>` Move to the next cell in the array
  
  \`<` Move to the previous cell in the array.
  
  \`+` Increments value in current cell by 1.
  
  \`-` Decrements value in current cell by 1.
  
  \`.` prints the UTF-8 character associated with the value in current cell.
  
  \`,` gets the UTF-8 number of a single character from the standard input.
  
  \`[` if the current cell contains 0, move to the character after the associated ].
  
  \`]` if the current cell does not contain 0, move back to the associated [.

All other characters are seen as comments.


### install
 depends on:
 * Rust
 
 clone the project and move into its directory, then
 ` cargo build --release`
 
### use:
  ` ./target/release/brain_fuck_interpreter <file.bf> `
