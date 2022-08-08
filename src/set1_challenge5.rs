use std::str;

fn repeating_key_xor(input: &str, key: &str) -> String {
    let r = input
        .chars()
        .zip(key.chars().cycle())
        .map(|(v, k)| v as u8 ^ k as u8)
        .collect::<Vec<u8>>();

    hex::encode(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            repeating_key_xor("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal", "ICE"),        
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
        );
    }
}
