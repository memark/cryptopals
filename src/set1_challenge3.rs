use rustc_serialize::hex::{FromHex, ToHex};

use crate::{english::probability_english, set1_challenge2::fixed_xor};
use std::{collections::BTreeMap, str};

fn singe_byte_xor_cipher(input: &str) -> String {
    (0..255_u8)
        .map(|c| {
            let cc = vec![c].to_hex();
            let b = cc.repeat(input.len());
            let r = fixed_xor(input, &b);
            str::from_utf8(&r.from_hex().unwrap())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            singe_byte_xor_cipher(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
            ),
            "Cooking MC's like a pound of bacon"
        );
    }
}
