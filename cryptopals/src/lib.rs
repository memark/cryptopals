mod utils;

use rayon::prelude::*;
use std::collections::BTreeMap;
use std::str;
use utils::*;

pub fn single_byte_xor_cipher(input: &str) -> String {
    get_best_match(get_possible_plaintexts_for_single_byte_xor_cipher(input))
}

pub fn detect_single_character_xor(input: Vec<&str>) -> String {
    get_best_match(
        input
            .par_iter()
            .flat_map_iter(|i| get_possible_plaintexts_for_single_byte_xor_cipher(i)),
    )
}

fn get_possible_plaintexts_for_single_byte_xor_cipher(input: &str) -> Vec<String> {
    let input2 = hex::decode(input).unwrap();

    (0..255_u8)
        .map(|c| {
            let key = [c].repeat(input2.len());
            let result = fixed_xor(&HexString::from(input), &key);
            str::from_utf8(&result).unwrap_or_default().to_owned()
        })
        .collect()
}

fn get_best_match<T>(plaintexts: T) -> String
where
    T: IntoParallelIterator<Item = String>,
{
    use english::probability_english_percent;

    plaintexts
        .into_par_iter()
        .map(|x| (probability_english_percent(&x), x.to_owned()))
        .collect::<BTreeMap<u8, String>>()
        .into_iter()
        .next_back()
        .unwrap()
        .1
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // #[test]
    // fn challenge_3_single_byte_xor_cipher() {
    //     assert_eq!(
    //         single_byte_xor_cipher(
    //             "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
    //         ),
    //         "Cooking MC's like a pound of bacon"
    //     );
    // }

    // #[test]
    // fn challenge_4_detect_single_character_xor() {
    //     let file = std::fs::read_to_string("data/4.txt").unwrap();
    //     let input = file.split_whitespace().collect();
    //     assert_eq!(
    //         detect_single_character_xor(input),
    //         "Now that the party is jumping\n"
    //     );
    // }

    #[ignore = "broken"]
    #[test]
    fn challenge_6_break_repeating_key_xor() {
        use itertools::Itertools;

        let file = std::fs::read_to_string("data/6.txt")
            .unwrap()
            .split('\n')
            .join("");
        assert_eq!(break_repeating_key_xor(&file), "wää");

        /*
        Jag behöver testdata för att lösa det här problemet känner jag.

        Nån plaintext som jag kan kryptera med en kortare upprepad nyckel.
        XOR är ju reversibelt, så det borde väl bara vara att använda samma funktion jag redan har för att skapa min ciphertext!
        */
    }

    #[test]
    fn transpose() {
        use itertools::Itertools;

        // Create a 2D array in row-major order: the rows of our 2D array are contiguous,
        // and the columns are strided
        let input_array = vec![1, 2, 3, 4, 5, 6];
        println!("{:#?}", input_array);

        let blocks = input_array.as_slice().chunks(3).collect_vec();
        println!("{:#?}", blocks);

        // Treat our 6-element array as a 2D 3x2 array, and transpose it to a 2x3 array
        let mut output_array = vec![0; 6];
        transpose::transpose(&input_array, &mut output_array, 3, 2);

        // The rows have become the columns, and the columns have become the rows
        let expected_array = vec![1, 4, 2, 5, 3, 6];
        assert_eq!(output_array, expected_array);

        let transposed_blocks = output_array.as_slice().chunks(2).collect_vec();
        println!("{:#?}", transposed_blocks);
    }
}
