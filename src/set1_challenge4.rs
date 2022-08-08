use crate::{english::probability_english_percent, set1_challenge2::fixed_xor};
use std::{collections::BTreeMap, str};

fn detect_single_character_xor(input: Vec<&str>) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "too slow"]
    #[test]
    fn test() {
        // read()?
        let file = std::fs::read_to_string("data/4.txt").unwrap();
        let input = file.split_whitespace().collect();
        assert_eq!(
            detect_single_character_xor(input),
            "Now that the party is jumping\n"
        );
    }
}
