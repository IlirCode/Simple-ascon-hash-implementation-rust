#![allow(unused)]

use std::io;

mod test_permutation;
use ascon_hash_implementation::State;
use ascon_hash_implementation::ascon_hash;
use ascon_hash_implementation::convert_pad_into_blocks;



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
    let mut message : Vec<u64> = Vec::new();

    // assumed that the message is a string in UTF-8 format

    /*In this code, we first create a mutable input string to store the user's input. Then, we use io::stdin().read_line(&mut input) to read a line from the standard input and store it in the input string.

    Next, we split the line into individual whitespace-separated numbers using split_whitespace(), which returns an iterator over these numbers as strings. We then use map() to parse each number string into a u64 value using parse().

    Finally, we use collect() to collect the parsed numbers into a vector of type Vec<u64>. The resulting vector is assigned to the numbers variable.

    You can modify the code according to your specific requirements and add error handling as needed. */


    // preparing the message blocks

    /*let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let output_blocks = ascon_hash(&input);
    
    print!("hash is ");
    for i in 0 .. 4 {
        print!("{:x}", output_blocks[i]);
    }
    print!("\n");
    // maybe want the output_blocks as a 
*/

    // for testing cause vs code doesn#t allow me to use hex in structs
    
  
        // numbers taken from the test for the official permutation
        let mut s : State = State::new(
            0x00400c0400000100,
            0x0,
            0x0,
            0x0,
            0x0)
            .permutation_12_for();
         
        let s_compare = State::new(
            0xee9398aadb67f03d, 
            0x8bb21831c60f1002, 
            0xb48a92db98d5da62, 
            0x43189921b8f8e3e8, 
            0x348fa5c9d525e140);

        println!("The result is {:x} {:x} {:x} {:x} {:x}", s.x[0], s.x[1], s.x[2], s.x[3], s.x[4]);
        println!("It shoulb be  {:x} {:x} {:x} {:x} {:x}", s_compare.x[0], s_compare.x[1], s_compare.x[2], s_compare.x[3], s_compare.x[4]);


}
