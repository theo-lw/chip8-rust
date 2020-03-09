pub mod chip8;
pub mod instructions;
pub mod variables;

fn main() {
    println!("Hello world!")
}

/*

CHIP-8 emulator architecture

The overall idea is simple, read instructions and execute them. 
There are many types of instructions and the effect of an instruction depends on the state of the emulator.
I.e, the effects can depend on the contents of the registers.

This calls for an Instruction trait with a method `evaluate` that takes in the state of the emulator

Instructions are called on variables. You can read variables or you can write variables. 

This calls for two traits, Read<T> and Write<T>. 
Read<T> should take in State and return T. 
Write<T> should take in T and mutate State.

*/
