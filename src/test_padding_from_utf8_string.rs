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
    // UTF-8 can be 1 to 4 bytes long. -> only full bytes
    // 0x80= 0b1000_0000 therefore represents the block that is always appended
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
        // ADD!!: longer example strings
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

mod full_test_of_ascon_hash {
    use crate::{ascon_hash, StringToU8};

    #[test]
    fn check() {
        let mut input_list: Vec<String> = vec![
            "some bytes".to_string(),
            // the following tests had originally been taken from the asconhash.txt provided by https://github.com/RustCrypto/hashes/blob/master/ascon-hash/tests/data/asconhash.txt
            // I assumed the line with MD is supposed to be the resulting hash from the Msg line
            // unsure if Msg is supposed to be a string or not
            /*" ".to_string(),
                "00".to_string(),
                "000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F404142434445464748494A4B4C4D4E4F505152535455565758595A5B5C5D5E5F606162636465666768696A6B6C6D6E6F707172737475767778797A7B7C7D7E7F808182838485868788898A8B8C8D8E8F909192939495969798999A9B9C9D9E9FA0A1A2A3A4A5A6A7A8A9AAABACADAEAFB0B1B2B3B4B5B6B7B8B9BABBBCBDBEBFC0C1C2C3C4C5C6C7C8C9CACBCCCDCECFD0D1D2D3D4D5D6D7D8D9DADBDCDDDEDFE0E1E2E3E4E5E6E7E8E9EAEBECEDEEEFF0F1F2F3F4F5F6F7F8F9FAFBFCFDFEFF000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F404142434445464748494A4B4C4D4E4F505152535455565758595A5B5C5D5E5F606162636465666768696A6B6C6D6E6F707172737475767778797A7B7C7D7E7F808182838485868788898A8B8C8D8E8F909192939495969798999A9B9C9D9E9FA0A1A2A3A4A5A6A7A8A9AAABACADAEAFB0B1B2B3B4B5B6B7B8B9BABBBCBDBEBFC0C1C2C3C4C5C6C7C8C9CACBCCCDCECFD0D1D2D3D4D5D6D7D8D9DADBDCDDDEDFE0E1E2E3E4E5E6E7E8E9EAEBECEDEEEFF0F1F2F3F4F5F6F7F8F9FAFBFCFDFEFF000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F404142434445464748494A4B4C4D4E4F505152535455565758595A5B5C5D5E5F606162636465666768696A6B6C6D6E6F707172737475767778797A7B7C7D7E7F808182838485868788898A8B8C8D8E8F909192939495969798999A9B9C9D9E9FA0A1A2A3A4A5A6A7A8A9AAABACADAEAFB0B1B2B3B4B5B6B7B8B9BABBBCBDBEBFC0C1C2C3C4C5C6C7C8C9CACBCCCDCECFD0D1D2D3D4D5D6D7D8D9DADBDCDDDEDFE0E1E2E3E4E5E6E7E8E9EAEBECEDEEEFF0F1F2F3F4F5F6F7F8F9FAFBFCFDFEFF000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F404142434445464748494A4B4C4D4E4F505152535455565758595A5B5C5D5E5F606162636465666768696A6B6C6D6E6F707172737475767778797A7B7C7D7E7F808182838485868788898A8B8C8D8E8F909192939495969798999A9B9C9D9E9FA0A1A2A3A4A5A6A7A8A9AAABACADAEAFB0B1B2B3B4B5B6B7B8B9BABBBCBDBEBFC0C1C2C3C4C5C6C7C8C9CACBCCCDCECFD0D1D2D3D4D5D6D7D8D9DADBDCDDDEDFE0E1E2E3E4E5E6E7E8E9EAEBECEDEEEFF0F1F2F3F4F5F6F7F8F9FAFBFCFDFEFF".to_string()
            */
        ];

        let mut compare_list: Vec<Vec<u64>> = vec![
            vec![
                0xb742ca75e5703875,
                0x7059cccc6874714f,
                0x9dbd7fc5924a7df4,
                0xe316594fd1426ca8,
            ],
            /*vec![
            0x7346BC14F036E87A,
            0xE03D0997913088F5,
            0xF68411434B3CF8B5,
            0x4FA796A80D251F91],
            vec![
            0x8DD446ADA58A7740,
            0xECF56EB638EF775F,
            0x7D5C0FD5F0C2BBBD,
            0xFDEC29609D3C43A2],
            vec![
            0x2EB89744DE7F9A6F,
            0x47D53DB756BB2F67,
            0xB127DA96762A1C47,
            0xA5D7BFC1F7273F5C]*/
        ];

        let mut calculated_output_list: Vec<Vec<u64>> = Vec::new();

        for input in input_list {
            let output = ascon_hash(&input);
            calculated_output_list.push(output);
        }
        /*
        for output in calculated_output_list {
            print!("The resul is: ");
                for number in output {
                    print!("{:x} ", number);
                }
                print!("\n");}
        */
        assert_eq!(compare_list, calculated_output_list);
    }
}
