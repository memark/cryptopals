use serialize::{
    base64::{self, ToBase64},
    hex::FromHex,
};

fn hex_to_base64(input: &str) -> String {
    input.from_hex().unwrap().to_base64(base64::STANDARD)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
        assert_eq!(
            result,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
