pub mod set1;
pub mod set2;

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
    use english::probability_english_percent;
    use std::collections::BTreeMap;

    (0..255_u8)
        .map(|c| {
            let b = get_repeated_key(c, input.len());
            let r = fixed_xor(input, &b);
            hex_to_utf8(&r)
        })
        .map(|x| (probability_english_percent(&x), x.to_owned()))
        .collect::<BTreeMap<u8, String>>()
        .values()
        .next_back()
        .unwrap()
        .to_owned()
}

fn get_repeated_key(c: u8, l: usize) -> String {
    hex::encode(vec![c]).repeat(l)
}

fn hex_to_utf8(h: &str) -> String {
    use std::str;

    str::from_utf8(&hex::decode(h).unwrap())
        .unwrap_or_default()
        .to_owned()
}

pub fn detect_single_character_xor(input: Vec<&str>) -> String {
    use english::probability_english_percent;
    use std::{collections::BTreeMap, str};

    input
        .iter()
        .flat_map(|i| {
            (0..255_u8)
                .map(|c| {
                    let key = hex::encode([c]).repeat(i.len());
                    let r = hex::decode(fixed_xor(i, &key)).unwrap();
                    str::from_utf8(&r).unwrap_or_default().to_owned()
                })
                .map(|x| (probability_english_percent(&x), x.to_owned()))
        })
        .collect::<BTreeMap<u8, String>>()
        .values()
        .next_back()
        .unwrap()
        .to_owned()
}

pub fn repeating_key_xor(input: &str, key: &str) -> String {
    let r = input
        .chars()
        .zip(key.chars().cycle())
        .map(|(v, k)| v as u8 ^ k as u8)
        .collect::<Vec<u8>>();

    hex::encode(r)
}

pub fn hamming_distance<'a, T>(a: T, b: T) -> u8
where
    T: IntoIterator<Item = &'a u8>,
{
    a.into_iter()
        .zip(b.into_iter())
        .map(|(x, y)| (x ^ y).count_ones() as u8)
        .sum()
}

pub fn break_repeating_key_xor(cipher_text_base64: &str) -> String {
    use itertools::Itertools;

    let cipher_text = base64::decode(cipher_text_base64).unwrap();

    let keysizes = 2..=40_usize;
    let best_keysizes = keysizes
        .map(|ks| {
            let first = &cipher_text[0..ks];
            let second = &cipher_text[ks..ks * 2];
            let distance = hamming_distance(first, second) as f32 / ks as f32;
            println!("keylength {:?} - distance {:?}", ks, distance);

            (ks, distance)
        })
        .sorted_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .take(5)
        .map(|x| {
            println!("candidate keylength {:?} - distance {:?}", x.0, x.1);
            x
        })
        .map(|x| x.0)
        .collect_vec();

    let best_keysize = best_keysizes[0];
    println!("best_keysize {:#?}", best_keysize);

    // HACK: Kolla så att transposen fungerar
    // let cipher_text = vec![1, 2, 3, 4, 5, 6];
    // println!("cipher_text {:#?}", cipher_text);
    // let best_keysize = 2;
    // println!("best_keysize {:#?}", best_keysize);

    let padding = calculate_padding(cipher_text.len(), best_keysize);
    let cipher_text_with_padding = [&cipher_text[..], &vec![0; padding]].concat();
    println!("cipher_text.len() {:#?}", cipher_text.len());
    println!("padding {:#?}", padding);
    println!(
        "cipher_text_with_padding.len() {:#?}",
        cipher_text_with_padding.len()
    );

    let mut transposed_cipher_text_with_padding = vec![0; cipher_text_with_padding.len()];
    transpose::transpose(
        &cipher_text_with_padding,
        &mut transposed_cipher_text_with_padding,
        best_keysize,
        cipher_text_with_padding.len() / best_keysize,
    );
    println!("output_array {:#?}", transposed_cipher_text_with_padding);

    let transposed_blocks = transposed_cipher_text_with_padding
        .as_slice()
        .chunks(cipher_text_with_padding.len() / best_keysize)
        .collect_vec();
    println!("transposed_blocks {:#?}", transposed_blocks);

    let res = transposed_blocks
        .iter()
        .take(1)
        .map(|block| single_byte_xor_cipher(&hex::encode(block)))
        .collect_vec();
    println!("res {:#?}", res);

    "".to_owned()
}

fn calculate_padding(total_length: usize, block_size: usize) -> usize {
    if total_length % block_size > 0 {
        block_size - (total_length % block_size)
    } else {
        0
    }
}

pub fn pkcs7_padding(block: &str, block_length: usize) -> String {
    let len_diff = block_length - block.len();
    let padding_char = len_diff as u8 as char;
    let padding_string = vec![padding_char; len_diff].iter().collect::<String>();
    [block, &padding_string].concat()
}
