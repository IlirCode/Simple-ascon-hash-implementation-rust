// for the implementation
#![allow(unused)]
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
#[cfg(test)]
mod tests_strings {
    use crate::StringToU8;

    pub const TEST_STRINGS: [&str; 6] = [
        "!", // 0x21
        "@", // 0x40
        "¥", // 0xc2_a5
        "1", // 0x31
        "0", // 0x30
        " ", // 0x20
    ];

    #[test]
    fn check_bytes_vector() {
        //println!("We get {:x}",TEST_STRINGS[2].string_to_u8()[0]);

        assert_eq!(TEST_STRINGS[0].string_to_u8()[0], 0x21);
        assert_eq!(TEST_STRINGS[1].string_to_u8()[0], 0x40);

        // 2 bytes
        assert_eq!(TEST_STRINGS[2].string_to_u8()[0], 0xc2);
        assert_eq!(TEST_STRINGS[2].string_to_u8()[1], 0xa5);

        assert_eq!(TEST_STRINGS[3].string_to_u8()[0], 0x31);
        assert_eq!(TEST_STRINGS[4].string_to_u8()[0], 0x30);
        assert_eq!(TEST_STRINGS[5].string_to_u8()[0], 0x20);
    }
}

#[cfg(test)]
mod test_padding {
    use crate::{vec_u8_to_u64_and_pad, StringToU8};

    #[test]
    fn check_pads() {
        use crate::vec_u8_to_u64_and_pad;
        let mut input_list: Vec<Vec<u8>> = vec![
            vec![0x10],
            vec![0x00],
            vec![0xff],
            vec![0x0, 0x0, 0x0, 0x0, 0x1],
            vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0], // on the verge of generaing a second block
            vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0], // should generate a second block
            vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1], // ditto
            vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x01],
            vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x01],
            vec![0xff, 0xff, 0xff, 0xff, 0xff],
            vec![
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x01,
            ], // on the edge of not going to a third one
            vec![
                0x0, 0x0, 0x00, 0x0, 0x00, 0x0, 0x0, 0x01, 0x0, 0x0, 0x00, 0x0, 0x00, 0x0, 0x0,
                0x01, 0x0, 0x0, 0x00, 0x0, 0x00, 0x0, 0x0, 0x01,
            ], //, vec![0x0] // Test vector for failing
        ];

        // the single appended 1 is a 0x80 added to the end
        let output_list: Vec<Vec<u64>> = vec![
            vec![0x10_80_00_00_00_00_00_00],
            vec![0x00_80_00_00_00_00_00_00],
            vec![0xff_80_00_00_00_00_00_00],
            vec![0x00_00_00_00_01_80_00_00],
            vec![0x00_00_00_00_00_00_00_80],
            vec![0x00, 0x80_00_00_00_00_00_00_00],
            vec![0x01, 0x80_00_00_00_00_00_00_00],
            vec![0x00_00_00_00_00_00_00_00, 0x01_80_00_00_00_00_00_00],
            vec![0x00_00_00_00_00_00_00_00, 0x00_01_80_00_00_00_00_00],
            vec![0xff_ff_ff_ff_ff_80_00_00],
            vec![0x0, 0x1, 0x80_00_00_00_00_00_00_00],
            vec![0x01, 0x01, 0x01, 0x80_00_00_00_00_00_00_00], //, vec![0x1] // Test vector for failing
        ];

        let mut calculated_output_list: Vec<Vec<u64>> = Vec::new();
        for input in input_list {
            let outputs = vec_u8_to_u64_and_pad(input);
            calculated_output_list.push(outputs);
        }

        assert_eq!(output_list, calculated_output_list);
    }

    #[test]
    fn string_to_pad() {
        // to test the final padding of the function with some examples
        // examples from the pub test string example
        pub const TEST_STRINGS: [&str; 6] = [
            " ", // 0x20
            "!", // 0x21
            "@", // 0x40
            "¥", // 0xc2_a5
            "1", // 0x31
            "0", // 0x30
        ];

        let output_list: Vec<Vec<u64>> = vec![
            vec![0x20_80_00_00_00_00_00_00], // " "
            vec![0x21_80_00_00_00_00_00_00],
            vec![0x40_80_00_00_00_00_00_00],
            vec![0xc2_a5_80_00_00_00_00_00],
            vec![0x31_80_00_00_00_00_00_00],
            vec![0x30_80_00_00_00_00_00_00],
        ];

        let mut calculated_output_list: Vec<Vec<u64>> = Vec::new();
        for input in TEST_STRINGS {
            let outputs = vec_u8_to_u64_and_pad(input.string_to_u8());
            
            calculated_output_list.push(outputs);
        }

        assert_eq!(output_list, calculated_output_list);
        
    }
}

#[cfg(test)]
mod full_test_of_ascon_hash {
    use crate::{ascon_hash, StringToU8};

    #[test]
    fn check() {
        let mut input_list: Vec<String> = vec![
            "00".to_string()
        ];

        let mut compare_list: Vec<Vec<u64>> = vec![vec![
            0x8DD446ADA58A7740,
            0xECF56EB638EF775F,
            0x7D5C0FD5F0C2BBBD,
            0xFDEC29609D3C43A2
        ]];

        let mut calculated_output_list: Vec<Vec<u64>> = Vec::new();
        for input in input_list {
            let output = ascon_hash(&input);
            calculated_output_list.push(output);
        }
        assert_eq!(compare_list, calculated_output_list);
    }
}
