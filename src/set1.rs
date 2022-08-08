#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn challenge_1_hex_to_base64() {
        let result = hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        assert_eq!(
            result,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

    #[test]
    fn challenge_2_fixed_xor() {
        assert_eq!(
            fixed_xor(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            ),
            "746865206b696420646f6e277420706c6179"
        );
    }

    #[test]
    fn challenge_3_single_byte_xor_cipher() {
        assert_eq!(
            single_byte_xor_cipher(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
            ),
            "Cooking MC's like a pound of bacon"
        );
    }

    #[ignore = "too slow"]
    #[test]
    fn challenge_4_detect_single_character_xor() {
        let file = std::fs::read_to_string("data/4.txt").unwrap();
        let input = file.split_whitespace().collect();
        assert_eq!(
            detect_single_character_xor(input),
            "Now that the party is jumping\n"
        );
    }

    #[test]
    fn challenge_5_repeating_key_xor() {
        assert_eq!(
            repeating_key_xor("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal", "ICE"),        
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );
    }

    #[test]
    fn challenge_6_hamming_distance() {
        assert_eq!(
            hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes()),
            37
        );
    }

    #[test]
    fn transpose() {
        use itertools::Itertools;

        // Create a 2D array in row-major order: the rows of our 2D array are contiguous,
        // and the columns are strided
        let input_array = vec![1, 2, 3, 4, 5, 6];
        println!("{:#?}", input_array);

        let blocks = input_array.as_slice().chunks(3).collect_vec();
        println!("{:#?}", blocks);

        // Treat our 6-element array as a 2D 3x2 array, and transpose it to a 2x3 array
        let mut output_array = vec![0; 6];
        transpose::transpose(&input_array, &mut output_array, 3, 2);

        // The rows have become the columns, and the columns have become the rows
        let expected_array = vec![1, 4, 2, 5, 3, 6];
        assert_eq!(output_array, expected_array);

        let transposed_blocks = output_array.as_slice().chunks(2).collect_vec();
        println!("{:#?}", transposed_blocks);
    }
}
