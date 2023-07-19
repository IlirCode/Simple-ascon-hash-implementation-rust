mod test_padding_from_utf8_string;
mod test_permutation;
//mod test_permutations;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

// constants
// for hash-a the round constant goes from 0 to 11 (p^a -> r = i ; p^b -> r = i+a-b)
pub const ROUND_CONSTANTS: [u64; 12] = [
    0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69, 0x5a, 0x4b,
];

// 320-bit State S
// operations are done on 5 64-bt words
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct State {
    pub x: [u64; 5], // first entry is r for us
}

// method on State variable
impl State {
    pub fn new(x_0: u64, x_1: u64, x_2: u64, x_3: u64, x_4: u64) -> Self {
        State {
            x: [x_0, x_1, x_2, x_3, x_4],
        }
    }

    pub fn single_permutation(mut self, round_constant: u64) -> Self {
        // the i is used for determining, which round constant is going to be used

        // add round constant
        //x_2 ^= round_constant;
        let mut x_0 = self.x[0];
        let mut x_1 = self.x[1];
        let mut x_2 = self.x[2];
        let mut x_3 = self.x[3];
        let mut x_4 = self.x[4];
        // S-box
        // not sure if I'm writting in the array or constantly changing the arrays length and location
        x_0 ^= x_4;
        x_2 ^= x_1 ^ round_constant;
        x_4 ^= x_3;

        // intermediate variables with x
        let x_0_0: u64 = x_0; // should be a copy not a borrow
        let x_1_1: u64 = x_1;

        x_0 ^= (!x_1) & x_2;
        x_1 ^= (!x_2) & x_3;
        x_2 ^= (!x_3) & x_4;
        x_3 ^= (!x_4) & x_0_0;
        x_4 ^= (!x_0_0) & x_1_1;

        x_1 ^= x_0;
        x_3 ^= x_2;
        x_0 ^= x_4;

        x_2 = !x_2; // can be combined in the following linear layer by inverting the output

        // linear layer
        // maybe better to seperate
        x_0 ^= x_0.rotate_right(19) ^ x_0.rotate_right(28);
        x_1 ^= x_1.rotate_right(61) ^ x_1.rotate_right(39);
        x_2 ^= x_2.rotate_right(1) ^ x_2.rotate_right(6);
        x_3 ^= x_3.rotate_right(10) ^ x_3.rotate_right(17);
        x_4 ^= x_4.rotate_right(7) ^ x_4.rotate_right(41);

        // return self
        State::new(x_0, x_1, x_2, x_3, x_4)
    }

    pub fn permutation_12_for(mut self) -> Self {
        for i in 0..12 {
            self = self.single_permutation(ROUND_CONSTANTS[i]);
        }

        // return
        self
    }

    /// Permutation function used 12 times
    pub fn permutation_12(mut self) -> Self {
        // see above just with the single permutation function with outside variables
        self = self
            .single_permutation(0xf0)
            .single_permutation(0xe1)
            .single_permutation(0xd2)
            .single_permutation(0xc3)
            .single_permutation(0xb4)
            .single_permutation(0xa5)
            .single_permutation(0x96)
            .single_permutation(0x87)
            .single_permutation(0x78)
            .single_permutation(0x69)
            .single_permutation(0x5a)
            .single_permutation(0x4b);

        // return
        self
    }
}

// for the padding function

// to simulate getting a string of UTF-8

pub fn read_string_from() -> Result<()> {
    let file = File::open("SomeText.txt")?; // reading from SomeText.txt in the same folder structute
                                            // question mark returns an error if not possible -> don't have to handle both cases
    let reader = BufReader::new(file); // from the internet -> Buffreader is very efficient for such tasks
    if let Some(Ok(line)) = reader.lines().next() {
        // iterator
        println!("Line: {}", line);
    }

    Ok(())
}

///////////////////
//// Assuming we get a string and have to convert
//// NOTE: these implementations are NOT online. They require the entire text block to be availabl at once
///////////////////

// UTF-8 uses 1 to 4 bytes
// should work for other encodings as well
pub fn message_blocks_padding(messages: &String) -> Vec<u64> {
    let mut vec_message_blocks: Vec<u64> = Vec::new();

    let bytes = messages.as_bytes();
    let block_number = (bytes.len() + 7) / 8; // the number of resulting blocks with padding
    let mut padding = 0;

    for i in 0..block_number {
        let start = i * 8;
        // compares the length of bytes and determins if enough bytes are left for the blocks
        let end = std::cmp::min(start + 8, bytes.len());
        let chunk = &bytes[start..end];

        let mut buffer: [u8; 8] = [0; 8];
        buffer[..chunk.len()].copy_from_slice(chunk);

        if chunk.len() < 8 {
            // Add padding
            padding = 8 - chunk.len();
            buffer[chunk.len()] = 0b1000_0000;
        }

        let value = u64::from_be_bytes(buffer);
        vec_message_blocks.push(value);
    }

    // Apply additional padding if needed
    if padding > 0 {
        let last_value = vec_message_blocks.last_mut().unwrap();
        *last_value <<= 8 * padding;
    }

    vec_message_blocks
}

// assuming the input data is a String of type UTF-8
// UTF-8 characters can be 1 to 4 bytes
pub fn convert_pad_into_blocks(messages: &String) -> Vec<u64> {
    // storing the message as a std::str::Bytes<'_>
    let mut input_blocks: Vec<u8> = messages.as_bytes().to_vec(); // as_bytes without taking ownership

    // 0b1000_0000 is always added
    input_blocks.push(0b1000_0000);

    // extending so that the message block lengths are dividable by 64 = 8*8
    let padding_need = 8 - (input_blocks.len() % 8);
    for _ in 1..=padding_need {
        input_blocks.push(0);
    }

    // vec <u8> to vec <u64> WITHOUT adding 0's before the bytes
    let mut result: Vec<u64> = Vec::new();

    for chunk in input_blocks.chunks(8) {
        let mut buffer: [u8; 8] = [0; 8]; // initialized as 0's

        for (i, &value) in chunk.iter().enumerate() {
            buffer[i] = value;
        }

        let value = u64::from_be_bytes(buffer);
        result.push(value);
    }

    // for chunks of 8
    // for i in 0..8 {result |= (input_blocks[i] as u64) << ((7 - i) * 8);

    result
}

// trying to divide and conquer
// takes in a string (UTF-8) and returns a vector u8
pub fn to_byte_vector(input: &str) -> Vec<u8> {
    input.bytes().collect()
}

// same but as a method
pub trait StringToU8 {
    fn string_to_u8(&self) -> Vec<u8>;
}

impl StringToU8 for &str {
    fn string_to_u8(&self) -> Vec<u8> {
        self.bytes().collect()
    }
}

impl StringToU8 for String {
    fn string_to_u8(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

// the current implementations is not online. That has to be fixed in future iterations
pub fn vec_u8_to_u64_and_pad(mut input_blocks: Vec<u8>) -> Vec<u64> {
    // needs to append a single 1 to the input blocks but the input blocks
    // as they come only blockwiese we have to add a byte onto it
    input_blocks.push(0b1000_0000);
    let padding_need = (8 - (input_blocks.len() % 8)) % 8; // last %8 is for case length %8 = 0
                                                           // alternatively use an if to overwrite
    for _ in 1..=padding_need {
        input_blocks.push(0);
    }
    // vec <u8> to vec <u64> WITHOUT adding 0's before the bytes
    let mut result: Vec<u64> = Vec::new();

    for chunk in input_blocks.chunks(8) {
        let mut buffer: [u8; 8] = [0; 8]; // initialized as 0's

        for (i, &value) in chunk.iter().enumerate() {
            buffer[i] = value;
        }

        let value = u64::from_be_bytes(buffer); // be = big endien; alternatively use le
        result.push(value);
    }

    // for chunks of 8
    // for i in 0..8 {result |= (input_blocks[i] as u64) << ((7 - i) * 8);

    result
}

pub fn ascon_hash(input_string: &String) -> Vec<u64> {
    // maybe just an array
    let message_blocks = convert_pad_into_blocks(&input_string);

    // Initialization
    //let mut s : State = (State::new(0x00400c0000000100, 0, 0, 0, 0)).permutation_12();

    // Initalized state precomputed
    let mut s: State = State::new(
        0xee9398aadb67f03d,
        0x8bb21831c60f1002,
        0xb48a92db98d5da62,
        0x43189921b8f8e3e8,
        0x348fa5c9d525e140,
    );

    // Absorption
    for i in &message_blocks {
        s.x[0] ^= *i;
        s = s.permutation_12_for(); // very last implementation is part of the Squeezing Phase
    }

    // Squeezing
    let mut output_blocks: Vec<u64> = Vec::new();

    // output of Hash-a is 256 bits = 64*4
    for _ in 0..3 {
        output_blocks.push(s.x[0]);
        s = s.permutation_12_for();
    }
    output_blocks.push(s.x[0]);
    output_blocks
}
