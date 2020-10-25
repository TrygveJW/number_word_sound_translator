#![feature(receiver_trait)]
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};

use crate::wiktionary::segment_structs::{WiktionaryPage, read_xml_title_content, multithred_parse};
use wiktionary::segment_structs;
use std::str::Chars;

mod trygvejw;
mod wiktionary;

extern crate num_cpus;

const LETTER_ARR: [char; 27] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', ' ',
];

struct SoundNumberPair{
    number: i32,
    letter: Vec<&'static str>
}




fn make_page_from_str(page_str: &String)-> Option<WiktionaryPage>{
    let mut  split  = page_str.split("@");
    return Some(WiktionaryPage{word: split.next()?.to_string(), ipa_pronontiation:  split.next()?.to_string() });
}

fn load_word_objects(file_name: &'static str) -> Vec<WiktionaryPage>{
    let file: std::fs::File = fs::File::open(file_name).unwrap();

    let mut bufered_reader = io::BufReader::new(file);
    let mut values: Vec<WiktionaryPage> = Vec::new();
    let mut buffer = String::new();
    let mut bytes_read = bufered_reader.read_line(&mut buffer).unwrap();
    while bytes_read > 0{




        let page = make_page_from_str(&buffer);

        if let Option::Some(v) = page {
            //println!("Word: {}", v.word);
            //println!("pronotiation: {}\n\n", v.ipa_pronontiation);
            values.push(v)
        }

        buffer.clear();
        bytes_read = bufered_reader.read_line(&mut buffer).unwrap();
        //println!("{}", bytes_read);
    }
    return values;
}

fn match_word_to_number(file_name: &'static str) {

    let pages = load_word_objects(&file_name);

    let mut ipa_letters : Vec<char> = Vec::new();

    for page in pages{
        let word_char = page.ipa_pronontiation.chars();

        for char in word_char{
            if !ipa_letters.contains(&char){
                ipa_letters.push(char.clone());
            }
        }
    }

    for char in &ipa_letters {
        println!("{}", char);
    }

    print!("num chars: {}", ipa_letters.len());

    /*print!("input number: ");
    io::stdout().flush();


    let mut nmr = String::new();

    io::stdin()
        .read_line(&mut nmr)
        .expect("Failed to read line");*/




}







fn main() {
    let s = String::from("sss");
    let xml_fp : &'static str = "/home/trygve/Development/projects/number_word_sound_translator/data/enwiktionary-20200901-pages-articles.xml";
    let logg_fp : &'static str = "/home/trygve/Development/projects/number_word_sound_translator/logg.txt";
    //multithred_parse(xml_fp);

    //read_xml_title_content(xml_fp);
    match_word_to_number(logg_fp);


    // println!("Hello, world!");
}
