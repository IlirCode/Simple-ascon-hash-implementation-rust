// some of their implementations
fn round(x: [u64; 5], c: u64) -> [u64; 5] {
    // S-box layer
    let x0 = x[0] ^ x[4];
    let x2 = x[2] ^ x[1] ^ c; // with round constant
    let x4 = x[4] ^ x[3];
    //println!("After their first step of S-box x0 = {:x}, x1 = {:x}, x2 = {:x}, x3 = {:x} and x4 = {:x}",x0, x[1], x2, x[3], x4);

    let tx0 = x0 ^ (!x[1] & x2);
    let tx1 = x[1] ^ (!x2 & x[3]);
    let tx2 = x2 ^ (!x[3] & x4);
    let tx3 = x[3] ^ (!x4 & x0);
    let tx4 = x4 ^ (!x0 & x[1]);
    //println!("Before their last step of S-box x0 = {:x}, x1 = {:x}, x2 = {:x}, x3 = {:x} and x4 = {:x}",tx0, tx1, (tx2), tx3, tx4);

    let tx1 = tx1 ^ tx0;
    let tx3 = tx3 ^ tx2;
    let tx0 = tx0 ^ tx4;

    // linear layer
    let x0 = tx0 ^ tx0.rotate_right(9);
    let x1 = tx1 ^ tx1.rotate_right(22);
    let x2 = tx2 ^ tx2.rotate_right(5);
    let x3 = tx3 ^ tx3.rotate_right(7);
    let x4 = tx4 ^ tx4.rotate_right(34);
    [
        tx0 ^ x0.rotate_right(19),
        tx1 ^ x1.rotate_right(39),
        !(tx2 ^ x2.rotate_right(1)),
        tx3 ^ x3.rotate_right(10),
        tx4 ^ x4.rotate_right(7),
    ]
}

fn permute_12_theirs(mut arr: [u64; 5]) -> [u64; 5] {
    arr = round(
        round(
            round(
                round(
                    round(
                        round(
                            round(
                                round(
                                    round(round(round(round(arr, 0xf0), 0xe1), 0xd2), 0xc3),
                                    0xb4,
                                ),
                                0xa5,
                            ),
                            0x96,
                        ),
                        0x87,
                    ),
                    0x78,
                ),
                0x69,
            ),
            0x5a,
        ),
        0x4b,
    );
    arr
}

#[cfg(test)]
mod test_permutations {
    pub const IV: u64 = 0x00400c0400000100;
    use ascon_hash_implementation::State;

    use crate::test_permutation::{permute_12_theirs, round};

    #[test]
    fn one_round() {
        // numbers taken from the test for the official permutation
        let mut s: State = State::new(
            0x0123456789abcdef,
            0x23456789abcdef01,
            0x456789abcdef0123,
            0x6789abcdef012345,
            0x89abcde01234567f,
        )
        .single_permutation(0x1f);

        let s_compare = State::new(
            0x3c1748c9be2892ce,
            0x5eafb305cd26164f,
            0xf9470254bb3a4213,
            0xf0428daf0c5d3948,
            0x281375af0b294899,
        );

        assert_eq!(s, s_compare);
    }

    #[test]
    fn one_round_more() {
        // random numbers from the ascon cargo
        // numbers taken from the test for the official permutation
        let mut s: State = State::new(
            0x0123456789abcdef,
            0x23456789abcdef01,
            0x456789abcdef0123,
            0x6789abcdef012345,
            0x89abcde01234567f,
        )
        .single_permutation(0x4b);

        // their shit
        let mut their_state: [u64; 5] = round(
            [
                0x0123456789abcdef,
                0x23456789abcdef01,
                0x456789abcdef0123,
                0x6789abcdef012345,
                0x89abcde01234567f,
            ],
            0x4b,
        );
        let mut s_theirs = State::new(
            their_state[0],
            their_state[1],
            their_state[2],
            their_state[3],
            their_state[4],
        );

        assert_eq!(s, s_theirs);

        // 2: the 0 case
        let mut s: State = State::new(0x0, 0x0, 0x0, 0x0, 0x0).single_permutation(0x4b);

        let mut their_state: [u64; 5] = round([0x0, 0x0, 0x0, 0x0, 0x0], 0x4b);
        let mut s_theirs = State::new(
            their_state[0],
            their_state[1],
            their_state[2],
            their_state[3],
            their_state[4],
        );

        assert_eq!(s, s_theirs);

        // maximum numbers
        let mut s: State = State::new(
            0xffff_ffff_ffff_ffff,
            0xffff_ffff_ffff_ffff,
            0xffff_ffff_ffff_ffff,
            0xffff_ffff_ffff_ffff,
            0xffff_ffff_ffff_ffff,
        )
        .single_permutation(0x4b);

        let mut their_state: [u64; 5] = round(
            [
                0xffff_ffff_ffff_ffff,
                0xffff_ffff_ffff_ffff,
                0xffff_ffff_ffff_ffff,
                0xffff_ffff_ffff_ffff,
                0xffff_ffff_ffff_ffff,
            ],
            0x4b,
        );
        let mut s_theirs = State::new(
            their_state[0],
            their_state[1],
            their_state[2],
            their_state[3],
            their_state[4],
        );
    }

    #[test]
    fn initialization_12_rounds_compair_their_result() {
        // numbers taken from the test for the official permutation
        let mut s: State = State::new(0x00400c0000000100, 0x0, 0x0, 0x0, 0x0).permutation_12_for();

        let s_compare = State::new(
            0xee9398aadb67f03d,
            0x8bb21831c60f1002,
            0xb48a92db98d5da62,
            0x43189921b8f8e3e8,
            0x348fa5c9d525e140,
        );

        assert_eq!(s, s_compare);
    }

    #[test]
    fn initialization_12_rounds_compare_ready_result() {
        // numbers taken from the test for the official permutation
        let mut s: State = State::new(
            0x00400c0400000100, // wrong initial vector but as long as it works out that is fine
            0x0,
            0x0,
            0x0,
            0x0,
        )
        .permutation_12_for();

        let mut s_compare = State::new(
            0xee9398aadb67f03d,
            0x8bb21831c60f1002,
            0xb48a92db98d5da62,
            0x43189921b8f8e3e8,
            0x348fa5c9d525e140,
        );

        let mut arr_theirs_input: [u64; 5] = [0x00400c0400000100, 0x0, 0x0, 0x0, 0x0];

        let mut their_sol = permute_12_theirs(arr_theirs_input);

        s_compare = State::new(
            their_sol[0],
            their_sol[1],
            their_sol[2],
            their_sol[3],
            their_sol[4],
        );
        assert_eq!(s, s_compare);
    }
} // end of the tests
