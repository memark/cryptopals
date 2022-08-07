use crate::{english::probability_english, set1_challenge2::fixed_xor};
use rustc_serialize::hex::{FromHex, ToHex};
use std::{collections::BTreeMap, str};

fn detect_single_character_xor(input: Vec<&str>) -> String {
    input
        .iter()
        .flat_map(|i| {
            (0..255_u8)
                .map(|c| {
                    let cc = vec![c].to_hex();
                    let b = cc.repeat(i.len());
                    let r = fixed_xor(i, &b);
                    str::from_utf8(&r.from_hex().unwrap())
                        .unwrap_or_default()
                        .to_string()
                })
                .map(|x| ((probability_english(&x) * 100f64) as u8, x.to_owned()))
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
        let file = std::fs::read_to_string("src/4.txt").unwrap();
        let input = file.split_whitespace().collect();
        assert_eq!(
            detect_single_character_xor(input),
            "Now that the party is jumping\n"
        );
    }
}
