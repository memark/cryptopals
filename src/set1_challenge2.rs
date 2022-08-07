use rustc_serialize::hex::{FromHex, ToHex};

pub fn fixed_xor(a: &str, b: &str) -> String {
    let aa = a.from_hex().unwrap();
    let bb = b.from_hex().unwrap();

    aa.iter()
        .zip(bb.iter())
        .map(|(x, y)| x ^ y)
        .collect::<Vec<u8>>()
        .to_hex()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            fixed_xor(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            ),
            "746865206b696420646f6e277420706c6179"
        );
    }
}
