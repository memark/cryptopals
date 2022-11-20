fn hex_to_base64(input: &str) -> String {
    base64::encode(hex::decode(input).unwrap())
}

pub fn hex_to_utf8(h: &str) -> String {
    std::str::from_utf8(&hex::decode(h).unwrap())
        .unwrap_or_default()
        .to_owned()
}

pub fn fixed_xor(a: &str, b: &str) -> String {
    let a = hex::decode(a).unwrap();
    let b = hex::decode(b).unwrap();

    let r = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| x ^ y)
        .collect::<Vec<u8>>();

    hex::encode(r)
}

pub fn repeating_key_xor(input: &str, key: &str) -> String {
    let r = input
        .chars()
        .zip(key.chars().cycle())
        .map(|(v, k)| v as u8 ^ k as u8)
        .collect::<Vec<u8>>();

    hex::encode(r)
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
    let padding_string = vec![padding_char; len_diff].iter().collect::<String>();
    [block, &padding_string].concat()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn challenge_1_hex_to_base64() {
        let result = hex_to_base64(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
        );
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
    fn challenge_5_repeating_key_xor() {
        assert_eq!(
            repeating_key_xor(
                "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal",
                "ICE"
            ),
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
    fn challenge_9_implement_pkcs_7_padding() {
        assert_eq!(
            pkcs7_padding("YELLOW SUBMARINE", 20),
            "YELLOW SUBMARINE\x04\x04\x04\x04"
        );
    }
}
