use std::collections::HashMap;
use std::io::BufRead;
use std::{fs, io};

use crate::number_word_translator::{WordIpaPair, WordNumberPair};
use crate::symbol_maps::get_arpabet_to_ipa_map;
use crate::util::strip_newline;
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;

pub fn parse_cmu(file_name: &Path) -> Vec<WordIpaPair> {
    let file: std::fs::File = fs::File::open(file_name).unwrap();

    let mut bufered_reader = io::BufReader::new(file);
    let mut values: Vec<WordIpaPair> = Vec::new();
    let mut buffer = String::new();

    let translate_map = get_arpabet_to_ipa_map();
    for _n in 0..56 {
        // need to move down exactly 56
        let _ = bufered_reader.read_line(&mut buffer);
    }
    buffer.clear();
    let mut bytes_read = bufered_reader.read_line(&mut buffer).unwrap();

    buffer = strip_newline(buffer);

    while bytes_read > 0 {
        let (word, pronon) = buffer.split_once("  ").unwrap();

        let pronon_vec: Vec<String> = pronon
            .split(" ")
            .map(|s| s.chars().filter(|c| !c.is_numeric()).collect::<String>())
            .map(|s| translate_map.get(&*s).unwrap().to_string())
            .collect();

        let save_obj = WordIpaPair {
            word: word.parse().unwrap(),
            ipa_symbols: pronon_vec,
        };

        values.push(save_obj);

        buffer.clear();
        bytes_read = bufered_reader.read_line(&mut buffer).unwrap();
        buffer = strip_newline(buffer);
    }

    return values;
}

// pub fn get_word_number_pairs(letter_num_file: &Path, cmu_word_file: &Path) -> Vec<WordNumberPair> {
//     let translator = WordNumberTranslator::new(letter_num_file);
//
//     let raw_words = load_words(cmu_word_file);
//
//     let mut translated_words: Vec<WordNumberPair> = Vec::new();
//
//     for word in raw_words {
//         translated_words.push(translator.translate_cmu_word(word));
//     }
//
//     return translated_words;
// }
