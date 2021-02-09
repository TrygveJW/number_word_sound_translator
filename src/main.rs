#![feature(receiver_trait)]
#![feature(bool_to_option)]
#![feature(map_into_keys_values)]
#![feature(core_intrinsics)]

use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
use unicode_segmentation::UnicodeSegmentation;

use crate::wiktionary::segment_structs::{
    multithred_parse, read_segment, read_xml_title_content, WiktionaryPage,
};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::intrinsics::{prefetch_read_instruction, size_of};
use std::iter::Map;
use std::panic::resume_unwind;
use std::str::{CharIndices, Chars};
use wiktionary::segment_structs;

mod trygvejw;
mod wiktionary;

extern crate num_cpus;

const LETTER_ARR: [char; 27] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', ' ',
];

struct SoundNumberPair {
    number: i32,
    letter: Vec<&'static str>,
}

struct WordNumberPair {
    number: String,
    word: String,
    ipa: String,
}

fn make_page_from_str(page_str: &String) -> Option<WiktionaryPage> {
    let mut split = page_str.split("@");
    return Some(WiktionaryPage {
        word: split.next()?.to_string(),
        ipa_pronontiation: split.next()?.to_string(),
    });
}

fn strip_newline(inp: String) -> String {
    return match inp.strip_suffix("\n") {
        None => inp,
        Some(strpped) => String::from(strpped),
    };
}
fn load_word_objects(file_name: &'static str) -> Vec<WiktionaryPage> {
    let file: std::fs::File = fs::File::open(file_name).unwrap();

    let mut bufered_reader = io::BufReader::new(file);
    let mut values: Vec<WiktionaryPage> = Vec::new();
    let mut buffer = String::new();
    let mut bytes_read = bufered_reader.read_line(&mut buffer).unwrap();

    buffer = strip_newline(buffer);

    // buffer
    //     .as_bytes()
    //     .iter()
    //     .for_each(|a| println!("{}", char::from(*a)));

    while bytes_read > 0 {
        let page = make_page_from_str(&buffer);

        if let Option::Some(v) = page {
            //println!("Word: {}", v.word);
            //println!("pronotiation: {}\n\n", v.ipa_pronontiation);
            values.push(v)
        }

        buffer.clear();
        bytes_read = bufered_reader.read_line(&mut buffer).unwrap();
        buffer = strip_newline(buffer);
        //println!("{}", bytes_read);
    }
    return values;
}

fn match_word_to_number(file_name: &'static str, translator: WordNumberTranslator) {
    let pages = load_word_objects(&file_name);

    let mut ipa_letters: Vec<String> = Vec::new();

    for page in pages {
        let a = page.ipa_pronontiation.graphemes(true);
        let word_char = page.ipa_pronontiation.chars();

        for char in a {
            let as_string = String::from(char);

            if !ipa_letters.contains(&as_string) {
                ipa_letters.push(as_string);
            }
        }
    }

    let word_objects = load_word_objects(file_name);
    let translated_words = translator.translate_page(word_objects);
    translated_words
        .iter()
        .for_each(|a| println!("{}-{}-{}", a.ipa, a.number, a.word));

    for char in &ipa_letters {
        //println!("{}", char);
    }

    // println!("num chars: {}", ipa_letters.len());

    loop {
        print!("input number: ");
        io::stdout().flush();

        let mut nmr = String::new();

        io::stdin()
            .read_line(&mut nmr)
            .expect("Failed to read line");

        translated_words
            .iter()
            .filter(|nwp| nwp.number == nmr.strip_suffix("\n").unwrap())
            .for_each(|a| println!("{}-{}", a.number, a.word));

        println!("num is {}", nmr);
    }
}

fn add_to_letter_number_map(
    mut size_map: HashMap<i32, HashMap<i32, Vec<String>>>,
    num: i32,
    tmp_val: &mut Vec<String>,
) -> HashMap<i32, HashMap<i32, Vec<String>>> {
    let val_size = tmp_val.len() as i32;
    // chek if the a map with this size keys exist
    (!size_map.contains_key(&val_size)).then(|| size_map.insert(val_size.clone(), HashMap::new()));
    // chek if a submap for this letter exists
    (!size_map.get_mut(&val_size).unwrap().contains_key(&num)).then(|| {
        size_map
            .get_mut(&val_size)
            .unwrap()
            .insert(num.clone(), Vec::new())
    });

    size_map
        .get_mut(&val_size)
        .unwrap()
        .get_mut(&num)
        .unwrap()
        .push(tmp_val.clone().join(""));

    return size_map;
}

fn get_letter_number_pair(file_path: &'static str) -> HashMap<i32, HashMap<i32, Vec<String>>> {
    let file: std::fs::File = fs::File::open(file_path).unwrap();

    let mut bufered_reader = io::BufReader::new(file);

    // the sizes of the map word
    let mut size_map: HashMap<i32, HashMap<i32, Vec<String>>> = HashMap::new();

    let mut buffer = String::new();
    let mut bytes_read = bufered_reader.read_line(&mut buffer).unwrap();
    while bytes_read > 0 {
        if buffer.len() > 0 {
            let mut chars = buffer.graphemes(true);
            let num: i32 = chars.next().unwrap().parse::<i32>().unwrap();
            chars.next();
            //assert_eq!(chars.next().unwrap(), "=".parse().unwrap());

            let mut tmp_val: Vec<String> = Vec::new();

            for character in chars {
                //println!("YYYYYYYYY  {}", character);
                if character.eq(",") || character.eq("\n") {
                    if !tmp_val.is_empty() {
                        size_map = add_to_letter_number_map(size_map, num, &mut tmp_val);
                        tmp_val.clear();
                    }
                } else {
                    tmp_val.push(character.to_string());
                }
            }
        }

        buffer.clear();
        bytes_read = bufered_reader.read_line(&mut buffer).unwrap();
    }

    // size_map.iter().for_each(|a| {
    //     println!("WORD Len {}", a.0);
    //     a.1.iter().for_each(|b| {
    //         println!("WORD idx {}", b.0);
    //         b.1.iter().for_each(|c| {
    //             println!("{}", c);
    //         })
    //     })
    // });
    return size_map;
}

struct WordNumberTranslator {
    number_sound_map: HashMap<i32, HashMap<i32, Vec<String>>>,
}

impl WordNumberTranslator {
    pub fn new(map_file_name: &'static str) -> WordNumberTranslator {
        let num_map = get_letter_number_pair(map_file_name);

        return WordNumberTranslator {
            number_sound_map: num_map,
        };
    }

    pub fn translate_page(&self, words: Vec<WiktionaryPage>) -> Vec<WordNumberPair> {
        let mut hit_indexes: Vec<&i32> = self.number_sound_map.keys().clone().collect();
        hit_indexes.sort_by(|a, b| b.cmp(a));
        hit_indexes.iter().for_each(|a| println!("{}", a));
        let mut ret_vec = Vec::new();

        for page in words {
            let mut save_nmr = String::new();
            let mut win_start: i32 = 0;
            let mut win_end: i32 = 0;

            let char_vec: Vec<String> = page
                .ipa_pronontiation
                .graphemes(true)
                .map(|g| String::from(g))
                .collect();

            let word_size = char_vec.len() as i32;
            while win_start < word_size {
                let mut found_one = false;
                for index in &hit_indexes {
                    win_end = win_start + **index;
                    if win_end > word_size {
                        continue;
                    }

                    let word_slice = &char_vec[win_start as usize..win_end as usize].join("");
                    for size_map in self.number_sound_map.get(index) {
                        for (k, v) in size_map {
                            for pattern in v {
                                if page.word.eq("Tivoli") {
                                    println!("aaaaaaaaaaaaa");
                                    println!("{}", word_slice);
                                    println!("{}", pattern);
                                }

                                if word_slice.eq(pattern) {
                                    if page.word.eq("Tivoli") {
                                        println!("HIT");
                                        println!("{}", word_slice);
                                        println!("{}", pattern);
                                        println!("HIT");
                                    }
                                    save_nmr += k.to_string().as_str();
                                    win_start += **index;
                                    found_one = true;
                                }
                                if found_one {
                                    break;
                                }
                            }
                            if found_one {
                                break;
                            }
                        }
                        if found_one {
                            break;
                        }
                    }
                    if found_one {
                        break;
                    }
                }

                if !found_one {
                    win_start += 1
                }
            }
            if !save_nmr.is_empty() {
                ret_vec.push(WordNumberPair {
                    word: page.word,
                    number: save_nmr,
                    ipa: page.ipa_pronontiation,
                })
            }
        }

        return ret_vec;
    }
}

fn map_words_to_numbers(map_file_name: &'static str, word_file_name: &'static str) {
    //let translator = get_letter_number_pair(map_file_name);
    let mut word_objects = load_word_objects(word_file_name);

    for page in word_objects {}
}

fn main() {
    let xml_fp : &'static str = "/home/trygve/Development/projects/number_word_sound_translator/data/enwiktionary-20200901-pages-articles.xml";
    let logg_fp: &'static str =
        "/home/trygve/Development/projects/number_word_sound_translator/logg.txt";
    let lnp_fp: &'static str =
        "/home/trygve/Development/projects/number_word_sound_translator/letter_number_pairs";
    //multithred_parse(xml_fp);

    //read_xml_title_content(xml_fp);
    //let a = get_letter_number_pair(lnp_fp);

    let abc = WordNumberTranslator::new(lnp_fp);

    match_word_to_number(logg_fp, abc);

    // println!("Hello, world!");
}
