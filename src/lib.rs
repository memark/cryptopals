mod english;
pub mod set1;

pub fn hex_to_base64(input: &str) -> String {
    base64::encode(hex::decode(input).unwrap())
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

pub fn single_byte_xor_cipher(input: &str) -> String {
    use crate::english::probability_english;
    use std::{collections::BTreeMap, str};

    (0..255_u8)
        .map(|c| {
            let cc = hex::encode(vec![c]);
            let b = cc.repeat(input.len());
            let r = fixed_xor(input, &b);
            str::from_utf8(&hex::decode(&r).unwrap())
                .unwrap_or_default()
                .to_string()
        })
        .map(|x| ((probability_english(&x) * 100f64) as u8, x.to_owned()))
        .collect::<BTreeMap<u8, String>>()
        .values()
        .next_back()
        .unwrap()
        .to_string()
}

pub fn detect_single_character_xor(input: Vec<&str>) -> String {
    use crate::english::probability_english_percent;
    use std::{collections::BTreeMap, str};

    input
        .iter()
        .flat_map(|i| {
            (0..255_u8)
                .map(|c| {
                    let key = hex::encode([c]).repeat(i.len());
                    let r = hex::decode(&key).unwrap();
                    str::from_utf8(&r).unwrap_or_default().to_string()
                })
                .map(|x| (probability_english_percent(&x), x.to_owned()))
        })
        .collect::<BTreeMap<u8, String>>()
        .values()
        .next_back()
        .unwrap()
        .to_string()
}

pub fn repeating_key_xor(input: &str, key: &str) -> String {
    let r = input
        .chars()
        .zip(key.chars().cycle())
        .map(|(v, k)| v as u8 ^ k as u8)
        .collect::<Vec<u8>>();

    hex::encode(r)
}

pub fn hamming_distance(a: &str, b: &str) -> u8 {
    a.bytes()
        .zip(b.bytes())
        .map(|(x, y)| (x ^ y).count_ones() as u8)
        .sum()
}
