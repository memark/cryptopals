pub fn probability_english_percent(text: &str) -> u8 {
    (probability_english(text) * 100f64) as u8
}

// Below code is from
// https://gist.github.com/elasticdog/c8c11f6cb5f00aa90611

use std::char;
use std::collections::btree_map::Entry::*;
use std::collections::BTreeMap;
use std::str;

/// Uses the Bhattacharyya coefficient to determine if text is likely to be English.
///
/// Higher is better.
fn probability_english(text: &str) -> f64 {
    // count of the number of times a character occurs in the given text
    let mut count: BTreeMap<char, f64> = BTreeMap::new();
    for letter in text.chars() {
        match count.entry(letter.to_ascii_uppercase()) {
            Vacant(entry) => {
                entry.insert(1f64);
            }
            Occupied(mut entry) => *entry.get_mut() += 1f64,
        }
    }

    // total number of characters in the given text
    let total = text.len() as f64;

    // relative frequency of letters in the English language
    let mut english_frequencies: BTreeMap<char, f64> = BTreeMap::new();
    english_frequencies.insert('A', 0.0651738);
    english_frequencies.insert('B', 0.0124248);
    english_frequencies.insert('C', 0.0217339);
    english_frequencies.insert('D', 0.0349835);
    english_frequencies.insert('E', 0.1041442);
    english_frequencies.insert('F', 0.0197881);
    english_frequencies.insert('G', 0.0158610);
    english_frequencies.insert('H', 0.0492888);
    english_frequencies.insert('I', 0.0558094);
    english_frequencies.insert('J', 0.0009033);
    english_frequencies.insert('K', 0.0050529);
    english_frequencies.insert('L', 0.0331490);
    english_frequencies.insert('M', 0.0202124);
    english_frequencies.insert('N', 0.0564513);
    english_frequencies.insert('O', 0.0596302);
    english_frequencies.insert('P', 0.0137645);
    english_frequencies.insert('Q', 0.0008606);
    english_frequencies.insert('R', 0.0497563);
    english_frequencies.insert('S', 0.0515760);
    english_frequencies.insert('T', 0.0729357);
    english_frequencies.insert('U', 0.0225134);
    english_frequencies.insert('V', 0.0082903);
    english_frequencies.insert('W', 0.0171272);
    english_frequencies.insert('X', 0.0013692);
    english_frequencies.insert('Y', 0.0145984);
    english_frequencies.insert('Z', 0.0007836);
    english_frequencies.insert(' ', 0.1918182);

    // update the counts to be the relative frequency of letters in the given text
    // and then calculate the Bhattacharyya coefficient as our score
    let mut score = 0.0;
    for letter in english_frequencies.keys() {
        match count.entry(*letter) {
            Vacant(entry) => {
                entry.insert(0.0);
            }
            Occupied(mut entry) => *entry.get_mut() /= total,
        }
        let partition_overlap = count[letter] * english_frequencies[letter];
        score += partition_overlap.sqrt();
    }

    score
}
