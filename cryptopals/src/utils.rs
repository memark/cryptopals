#![allow(dead_code)]

pub fn fixed_xor(input: &[u8], key: &[u8]) -> Vec<u8> {
    input
        .iter()
        .zip(key.iter())
        .map(|(x, y)| x ^ y)
        .collect::<Vec<u8>>()
}

pub fn repeating_key_xor(input: &[u8], key: &[u8]) -> Vec<u8> {
    fixed_xor(input, &key.repeat(input.len()))
}

pub fn hamming_distance<'a, T>(a: T, b: T) -> u8
where
    T: IntoIterator<Item = &'a u8>,
{
    a.into_iter()
        .zip(b.into_iter())
        .map(|(x, y)| (x ^ y).count_ones() as u8)
        .sum()
}

pub fn pkcs7_padding(block: &str, block_length: usize) -> String {
    let len_diff = block_length - block.len();
    let padding_char = len_diff as u8 as char;
    let padding_string = vec![padding_char; len_diff].into_iter().collect::<String>();
    [block, &padding_string].concat()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn challenge_1_hex_to_base64() {
        assert_eq!(
            base64::encode(hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap())            ,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }

    #[test]
    fn challenge_2_fixed_xor() {
        assert_eq!(
            fixed_xor(
                &hex::decode("1c0111001f010100061a024b53535009181c").unwrap(),
                &hex::decode("686974207468652062756c6c277320657965").unwrap()
            ),
            hex::decode("746865206b696420646f6e277420706c6179").unwrap()
        );
    }

    #[test]
    fn challenge_5_repeating_key_xor() {
        assert_eq!(
            repeating_key_xor(
                b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
                b"ICE",
            ),
            hex::decode("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").unwrap(),
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
    fn challenge_9_implement_pkcs_7_padding() {
        assert_eq!(
            pkcs7_padding("YELLOW SUBMARINE", 20),
            "YELLOW SUBMARINE\x04\x04\x04\x04"
        );
    }
}
