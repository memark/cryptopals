use std::str;

fn hamming_distance(a: &str, b: &str) -> u8 {
    a.bytes()
        .zip(b.bytes())
        .map(|(x, y)| (x ^ y).count_ones() as u8)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(hamming_distance("this is a test", "wokka wokka!!!"), 37);
    }

    // #[test]
    // fn test() {
    //     let file = std::fs::read("data/6.txt").unwrap();
    //     assert_eq!(
    //         repeating_key_xor("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal", "ICE"),
    //         "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
    //     );
    // }
}
