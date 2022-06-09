use crate::number_word_translator::WordIpaPair;
use crate::util::strip_newline;
use encoding_rs::mem::Latin1Bidi::Latin1;
use encoding_rs::{ISO_8859_10, WINDOWS_1252};
use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, Write};
use std::path::Path;
use std::thread::current;
use std::{fs, io};

use crate::symbol_maps::get_xsampa_to_ipa_map;
use encoding_rs_io::DecodeReaderBytesBuilder;

fn try_get_cache() -> Option<Vec<WordIpaPair>> {
    let cache_file = Path::new("nst_cache");

    let file: std::fs::File = fs::File::open(cache_file).ok()?;

    let mut bufered_reader = io::BufReader::new(file);

    let mut values: Vec<WordIpaPair> = Vec::new();
    let mut buffer = String::new();

    buffer.clear();
    let mut bytes_read = bufered_reader.read_line(&mut buffer).unwrap();
    buffer = strip_newline(buffer);

    while bytes_read > 0 {
        // let (word, pronon) = buffer.split_once("  ").unwrap();

        let pronon_vec: Vec<&str> = buffer.split("@@").collect();
        let (word_vec, pronon_parts) = pronon_vec.split_at(1);
        // println!("{:?} - {:?}", word, xsampa_word);

        let save_obj = WordIpaPair {
            word: word_vec.first().unwrap().clone().parse().unwrap(),
            ipa_symbols: pronon_parts.iter().map(|s| s.clone().to_string()).collect(),
        };

        values.push(save_obj);

        buffer.clear();
        bytes_read = bufered_reader.read_line(&mut buffer).unwrap();
        buffer = strip_newline(buffer);
    }

    return Some(values);
}

fn save_to_cache(encoded: &Vec<WordIpaPair>) {
    let cache_file = Path::new("nst_cache");

    let file: std::fs::File = fs::File::create(cache_file).unwrap();
    let mut bufered_writer = io::BufWriter::new(file);

    for pair in encoded {
        let line = format!("{}@@{}\n", pair.word, pair.ipa_symbols.join("@@"));
        bufered_writer.write(line.as_bytes());
    }
    bufered_writer.flush();
}

pub fn parse_nst(file_name: &Path) -> Vec<WordIpaPair> {
    match try_get_cache() {
        None => {}
        Some(hit) => return hit,
    }

    let mut is_on_line = 0;
    let stop_at_liner = 20000;

    let translate_map = get_xsampa_to_ipa_map();
    let biggest_translate_key = translate_map.keys().max_by_key(|x| x.len()).unwrap().len();

    let file: std::fs::File = fs::File::open(file_name).unwrap();

    let mut bufered_reader = io::BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(file),
    );
    let mut values: Vec<WordIpaPair> = Vec::new();
    let mut buffer = String::new();

    // for _n in 0..56 {
    //     // need to move down exactly 56
    //     let _ = bufered_reader.read_line(&mut buffer);
    // }
    buffer.clear();
    let mut bytes_read = bufered_reader.read_line(&mut buffer).unwrap();

    buffer = strip_newline(buffer);

    while bytes_read > 0 {
        // let (word, pronon) = buffer.split_once("  ").unwrap();

        let pronon_vec: Vec<String> = buffer
            .split(";")
            // .map(|s| s.chars().filter(|c| !c.is_numeric()).collect::<String>())
            .map(|s| s.chars().collect::<String>())
            .collect();
        let xsampa_word = pronon_vec.get(11).unwrap(); //.strip_prefix("\\\"").unwrap();
        let word = pronon_vec.get(0).unwrap(); //.strip_prefix("\\\"").unwrap();
                                               // println!("{:?} - {:?}", word, xsampa_word);

        let mut ipa_word: Vec<String> = Vec::new();
        let mut idx = 0;

        while idx < xsampa_word.len() {
            let mut current_match_size = 0;
            let mut current_match = None;
            for k in translate_map.keys() {
                let k_size = k.len();
                if k_size >= current_match_size {
                    // println!("{}", &xsampa_word[idx..(idx + k_size)]);
                    // println!("{}", k);
                    if *k == &xsampa_word[idx..min((idx + k_size), xsampa_word.len())] {
                        current_match = Some(k);
                        current_match_size = k_size
                    }
                }
            }
            let sub_value = translate_map.get(current_match.unwrap());

            ipa_word.push(sub_value.unwrap().to_string());
            idx += current_match_size;
        }

        // println!("{:?}", pronon_vec.get(11).unwrap());
        // println!("{:?}, {:?}", word, ipa_word);
        let save_obj = WordIpaPair {
            word: word.parse().unwrap(),
            ipa_symbols: ipa_word,
        };

        values.push(save_obj);

        buffer.clear();
        bytes_read = bufered_reader.read_line(&mut buffer).unwrap();
        buffer = strip_newline(buffer);

        // if is_on_line > stop_at_liner {
        //     return values;
        // } else {
        //     is_on_line += 1
        // }
    }
    save_to_cache(&values);

    return values;
}
