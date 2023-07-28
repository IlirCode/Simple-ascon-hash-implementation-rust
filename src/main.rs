#![allow(unused)] // compiler doesn't complain about unused variables and mut anymore -> remove for release

use std::io;

use ascon_hash_implementation::*;

/*
Rust implementation done in small steps

DURING INITIAL DEVELOPEMENT WE ASSUME THAT THE MESSAGE M HAS BEEN PADDED

The permutation function consists of submodules
Initial vector depends on the operation mode -> here IV = 0x00400c0400000100 (Ascon-Hash-a)
In future iterations that let the user choose the Hashing function use a struct that lets the user choose


implementation with 64 bit systems in mind
possible future implementation: bit-interleaved for 32, 16 or 8-bit pocessors
-> not sure if the compiler takes care of that for me

main function responsible for managing the phases (Absorption)


THE ASCON STEPS
Initilization:
    - IV appended by 0's for the initial state -> S = IV || 0^256
    - IV input to the permutation function S = p^a (IV || 0^256)
Absorbing Message:
    - Message M is divided into blocks and padded
    - for each round except the last the Message block is XORed into the state S and used as input for the next function
        -> for the last block the p^b is not performed -> p^a will be performed in the squeezing phase
        -> HERE: p^a = p^b
Squeezing:
    - start with p^a(S)
    - extract the message
    - for the following we use p^b
    - the last result has to be truncated
        -> Hash-a r = 64, c = 256, hash output = 256 -> needs 4 output blocks
*/

// Initial Vectors ->   later:  depending on a global variable the user should be able to choose which version of hash to use
//                              this should include the round size

fn main() {
    // vector used for saving the string -> entries should be size r = 64 bits
    let mut message: Vec<u64> = Vec::new();

    // assumed that the message is a string in UTF-8 format
    // preparing the message blocks

    println!(
        "Please enter a valid UTF-8 string and confirm.\n Note that the line break is included in the string"
    );
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let output_blocks = ascon_hash(&input);

    print!("hash is ");
    for i in 0..4 {
        print!("{:x}", output_blocks[i]);
    }
    print!("\n");
    // maybe want the output_blocks as a
}
