#[allow(unused)]
mod parallel_implementations;
mod test_padding_from_utf8_string;
mod test_permutation;
//mod test_permutations;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::thread;

use rayon::prelude::*; // library for data parallization

// constants
pub const ROUND_CONSTANTS: [u64; 12] = [
    0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69, 0x5a, 0x4b,
];

// 320-bit State S
// operations are done on 5 64-bit words
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct State {
    pub x: [u64; 5], // in ascon hash: r = first entry = state_name.x[0]
}

// method on State variable
impl State {
    pub fn new(x_0: u64, x_1: u64, x_2: u64, x_3: u64, x_4: u64) -> Self {
        State {
            x: [x_0, x_1, x_2, x_3, x_4],
        }
    }

    pub fn new_arr(arr: [u64; 5]) -> Self {
        State { x: arr }
    }

    // alternatively not defining single_permutation on state but working on arrays or tuples would
    // be faster compared to working with methods on the struct
    /// Ascon's single round permutation function. Method on State takes the current round constant as input
    pub fn single_permutation(self, round_constant: u64) -> Self {
        // round constant added further below
        // operations on single variable faster compared to arrays or even tuples
        let mut x_0 = self.x[0];
        let mut x_1 = self.x[1];
        let mut x_2 = self.x[2];
        let mut x_3 = self.x[3];
        let mut x_4 = self.x[4];

        // S-box
        x_0 ^= x_4;
        x_2 ^= x_1 ^ round_constant;
        x_4 ^= x_3; // -> no to paralel

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

    pub fn single_permutation_with_tupples(self, round_constant: u64) -> Self {
        let (mut x_0, mut x_1, mut x_2, mut x_3, mut x_4) =
            (self.x[0], self.x[1], self.x[2], self.x[3], self.x[4]);

        // S-box
        x_0 ^= x_4;
        x_2 ^= x_1 ^ round_constant;
        x_4 ^= x_3;

        let x_0_0: u64 = x_0; // should be a copy not a borrow
        let x_1_1: u64 = x_1;

        x_0 ^= (!x_1) & x_2;
        x_1 ^= (!x_2) & x_3;
        x_2 ^= (!x_3) & x_4;
        x_3 ^= (!x_4) & x_0_0;
        x_4 ^= (!x_0) & x_1_1;

        x_1 ^= x_0;
        x_3 ^= x_2;
        x_0 ^= x_4;

        x_2 = !x_2;

        // Linear layer
        let rotate_xor = |x: u64, r1: u32, r2: u32| x.rotate_right(r1) ^ x.rotate_right(r2);
        x_0 ^= rotate_xor(x_0, 19, 28);
        x_1 ^= rotate_xor(x_1, 61, 39);
        x_2 ^= rotate_xor(x_2, 1, 6);
        x_3 ^= rotate_xor(x_3, 10, 17);
        x_4 ^= rotate_xor(x_4, 7, 41);

        State::new(x_0, x_1, x_2, x_3, x_4)
    }

    pub fn single_permutation_concurrent_mutex(self, round_constant: u64) -> Self {
        let mut x_0 = self.x[0];
        let mut x_1 = self.x[1];
        let mut x_2 = self.x[2];
        let mut x_3 = self.x[3];
        let mut x_4 = self.x[4];

        // S-box
        x_0 ^= x_4;
        x_2 ^= x_1 ^ round_constant;
        x_4 ^= x_3;

        let x_0_0 = x_0;
        let x_1_1 = x_1;

        x_0 ^= (!x_1) & x_2;
        x_1 ^= (!x_2) & x_3;
        x_2 ^= (!x_3) & x_4;
        x_3 ^= (!x_4) & x_0_0;
        x_4 ^= (!x_0_0) & x_1_1;

        x_1 ^= x_0;
        x_3 ^= x_2;
        x_0 ^= x_4;

        // Linear layer
        let handle0 = thread::spawn(move || x_0.rotate_right(19) ^ x_0.rotate_right(28));
        let handle1 = thread::spawn(move || x_1.rotate_right(61) ^ x_1.rotate_right(39));
        let handle2 = thread::spawn(move || x_2.rotate_right(1) ^ x_2.rotate_right(6));
        let handle3 = thread::spawn(move || x_3.rotate_right(10) ^ x_3.rotate_right(17));
        let handle4 = thread::spawn(move || x_4.rotate_right(7) ^ x_4.rotate_right(41));

        // Retrieve the updated values from the spawned threads
        let x_0_rotated = handle0.join().unwrap();
        let x_1_rotated = handle1.join().unwrap();
        let x_2_rotated = handle2.join().unwrap();
        let x_3_rotated = handle3.join().unwrap();
        let x_4_rotated = handle4.join().unwrap();

        State::new(
            x_0 ^ x_0_rotated,
            x_1 ^ x_1_rotated,
            !x_2 ^ x_2_rotated,
            x_3 ^ x_3_rotated,
            x_4 ^ x_4_rotated,
        )
    }

    pub fn single_permutation_concurent(mut self, round_constant: u64) -> Self {
        // S-box (parallelize the independent operations)
        self.x.par_iter_mut().for_each(|x| *x ^= round_constant);
        self.x.swap(0, 1);
        self.x.swap(3, 4);
        self.x.par_iter_mut().for_each(|x| *x ^= round_constant);
        self.x.swap(1, 2);
        self.x.swap(2, 3);
        self.x.swap(3, 4);
        self.x.swap(0, 1);

        // Intermediate variables with x (parallelize the independent operations)
        let x_0_0: u64 = self.x[0];
        let x_1_1: u64 = self.x[1];

        self.x.par_iter_mut().for_each(|x| *x ^= (!x_1_1) & x_0_0);

        // Linear layer (parallelize the independent operations)
        self.x
            .par_iter_mut()
            .for_each(|x| *x ^= x.rotate_right(19) ^ x.rotate_right(28));
        self.x.swap(1, 2);
        self.x
            .par_iter_mut()
            .for_each(|x| *x ^= x.rotate_right(61) ^ x.rotate_right(39));
        self.x.swap(2, 3);
        self.x
            .par_iter_mut()
            .for_each(|x| *x ^= x.rotate_right(1) ^ x.rotate_right(6));
        self.x.swap(3, 4);
        self.x
            .par_iter_mut()
            .for_each(|x| *x ^= x.rotate_right(10) ^ x.rotate_right(17));
        self.x.swap(0, 1);
        self.x
            .par_iter_mut()
            .for_each(|x| *x ^= x.rotate_right(7) ^ x.rotate_right(41));

        self
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

    for chunk in input_blocks.chunks_exact(8) {
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

/// trait for converting string and slices to a vector of u8
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
        self.as_bytes().to_vec() // as bytes does not take ownership
    }
}

// the current implementations is not online. That has to be fixed in future iterations
/// Ascon's padding function. Takes a vector of u8 and returns the padded blockwise state. Assumes
/// the padding cannot fail (for now)
/// not online as of now
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

pub fn vec_u8_to_u64_and_pad_version_2(mut input_blocks: Vec<u8>) -> Vec<u64> {
    // Add a single 1 to the input blocks
    input_blocks.push(0b1000_0000);

    // Calculate the padding needed and append zeros accordingly
    let padding_need = (8 - (input_blocks.len() % 8)) % 8;
    input_blocks.extend(std::iter::repeat(0).take(padding_need));

    // Convert vec<u8> to vec<u64>
    let mut result: Vec<u64> = Vec::new();
    let mut buffer: [u8; 8] = [0; 8]; // Reuse the buffer for better performance

    for chunk in input_blocks.chunks_exact(8) {
        buffer.copy_from_slice(chunk);
        let value = u64::from_be_bytes(buffer);
        result.push(value);
    }

    result
}

pub fn ascon_hash(input_string: &String) -> Vec<u64> {
    // padded input message divided into blocks
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

    // output of Hash is 256 bits = 64*4 -> 4 blocks
    for _ in 0..3 {
        output_blocks.push(s.x[0]);
        s = s.permutation_12_for();
    }
    output_blocks.push(s.x[0]);
    output_blocks
}
