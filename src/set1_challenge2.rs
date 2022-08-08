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
