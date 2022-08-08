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
        assert_eq!(hamming_distance("this is a test", "wokka wokka!!!"), 37);
    }
}
