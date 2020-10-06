use crate::trygvejw::utils::vector_utils;
use std::ptr::null;
use std::{fs, io};
use crate::wiktionary::segment_structs;
use std::io::{BufRead, Write, Seek, SeekFrom};
use std::fs::File;

use std::sync::mpsc;
use std::thread;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;


enum SegmentTypes {
    Pronunciation,
}

struct Segment {}

pub struct WiktionaryPage {
    pub word: String,
    pub ipa_pronontiation: String,
}

impl Segment {
    const LETTER_ARR: [char; 27] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ' ',
    ];
    const WORD_TYPES: [&'static str; 13] = [
        "Adjective",
        "Adverb",
        "Conjunction",
        "Determiner",
        "Interjection",
        "Noun",
        "Numeral",
        "Particle",
        "Postposition",
        "Preposition",
        "Pronoun",
        "Proper noun",
        "Verb",
    ];

    fn split_lines(lines: &String) -> Vec<String> {
        lines.split("\n").map(|x| String::from(x)).collect()
    }

    fn is_word_english(lines: &Vec<&mut String>) -> bool {
        // vector_utils::index_of_first_match<String(|x: String| {x.trim().sta})
        //line.contains("{{en-")
        true
    }
}

fn get_string_between(prefix: &str, suffix: &str, seartch_str: &String) -> Option<String> {
    let split = seartch_str.splitn(2, prefix);
    let split = split.last()?.splitn(2, suffix).next()?.to_string();
    Some(split)
}

fn is_page_of_language(compairLang: &String, lines: &Vec<String>) -> bool {
    let lang_line_index = vector_utils::index_of_first_match(|x: &String| x.contains("=="), &lines);
    if lang_line_index > 0 {
        return match get_string_between(
            "==",
            "==",
            lines.get(lang_line_index).unwrap_or(&String::from(" ")),
        ) {
            Some(s) => *compairLang == s,
            None => false,
        };
    }
    false
}

fn get_page_word(lines: &Vec<String>) -> Option<String> {
    let lang_line_index =
        vector_utils::index_of_first_match(|x: &String| x.contains("<title>"), &lines);
    if lang_line_index > 0 {
        return match lines.get(lang_line_index) {
            Some(v) => get_string_between("<title>", "</title>", v),
            None => None,
        };
    }
    None
}

fn get_page_IPA_pronontiation(lines: &Vec<String>) -> Option<String> {
    let pronotiation_index =
        vector_utils::index_of_first_match(|x: &String| x.contains("===Pronunciation==="), &lines);
    let mut ret_str = String::new();
    let mut best = String::new();
    for n in 1..10 {
        let line = lines.get(pronotiation_index + n)?;

        if (line.contains("US") || line.contains("GA")) && line.contains("{{IPA|en|/"){
            let split = line.splitn(2, "{{IPA|en|/");
            best = split.last()?.splitn(2, "/").next()?.to_string();
        } if line.contains("{{IPA|en|/") && best.is_empty() {
            let split = line.splitn(2, "{{IPA|en|/");
            best = split.last()?.splitn(2, "/").next()?.to_string();
        } else if line == " " {
            if best.is_empty() {
                return None;
            } else {
                if best.contains("("){

                }

                return Some(best);
            }

        }
    }

    return None;
}

pub fn parse_page_block(lines: &Vec<String>) -> Option<WiktionaryPage> {
    // let mut lines: Vec<String> = block.split("\n").map(|x| String::from(x)).collect();

    let is_english = is_page_of_language(&String::from("English"), &lines);
    if is_english {
        let word_option = get_page_word(&lines)?;

        if !word_option.contains(":") {
            // print!("word is: {}\n", word_option);
            let ipa = get_page_IPA_pronontiation(lines)?;
            // print!("pronotiation is: \n{}\n", ipa);
            return Some(WiktionaryPage {
                word: word_option,
                ipa_pronontiation: ipa,
            });
        }
    }

    None
}

pub struct FileSegment {
     start: usize,
     end: usize,
}
fn get_segment_run_list(file_name: &'static str) -> Vec<FileSegment> {
    let cpus = num_cpus::get() as u64;


    let file: std::fs::File = fs::File::open(file_name).unwrap();
    let file_size = file.metadata().unwrap().len() as u64;
    let interval =  (file_size/cpus) as u64;
    let mut start_pos_list: Vec<FileSegment>  = Vec::new();

    let mut buffer = String::new();
    let mut bytes_read = 1;

    let mut bufered_reader = io::BufReader::new(file);

    //println!("finding for {} cores", cpus);

    for n in 0..(cpus-1) {
        //println!("core {}", n);
        let seek_start = if start_pos_list.is_empty() { 0 } else { start_pos_list.last().unwrap().end +1};
        let mut possision = seek_start + interval as usize;
        bufered_reader.seek(SeekFrom::Start(possision as u64));

        buffer.clear();
        while !buffer.contains("</page>") {
            if possision >= file_size as usize {
                println!("yup not ideal");
                break
            }
            buffer.clear();
            bytes_read = bufered_reader.read_line(&mut buffer).unwrap();

            possision = possision + bytes_read;
        }

        start_pos_list.push(FileSegment{start:seek_start, end:possision});
    }
    //println!("all cores done");
    start_pos_list.push(FileSegment{start: start_pos_list.last().unwrap().end +1, end: file_size as usize });
    return start_pos_list;


}


pub fn read_segment(segment: FileSegment, file_name: &'static str) -> Vec<WiktionaryPage> {
    let file: std::fs::File = fs::File::open(file_name).unwrap();

    let mut bufered_reader = io::BufReader::new(file);
    let mut vals: Vec<WiktionaryPage> = Vec::new();
    let mut page_buffer: Vec<String> = Vec::new();
    let mut buffer = String::new();
    let mut bytes_read = 1;
    let mut possision = segment.start;
    bufered_reader.seek(SeekFrom::Start(possision as u64));
    while bytes_read > 0{


        buffer.clear();
        bytes_read = bufered_reader.read_line(&mut buffer).unwrap();
        possision += bytes_read;
        if possision >= segment.end {
            break;
        }
        page_buffer.push(buffer.clone());


        if buffer.contains("</page>") {
            let page = segment_structs::parse_page_block(&page_buffer);
            if let Option::Some(v) = page {
                vals.push(v);
            }

            page_buffer.clear();
        }
    }

    return vals;

}


pub fn multithred_parse(file_path: &'static str){
    let segments = get_segment_run_list(file_path);

    let mut recive_list: Vec<WiktionaryPage> = Vec::new();
    let mut rx_list: Vec<Receiver<Vec<WiktionaryPage>>> = Vec::new();
    // let (tx,rx): (Sender<Vec<WiktionaryPage>>, Receiver<Vec<WiktionaryPage>>) = mpsc::chanel();

    for segment in segments{
        let (tx,rx): (Sender<Vec<WiktionaryPage>>, Receiver<Vec<WiktionaryPage>>) = mpsc::channel();
        rx_list.push(rx);
        // let tx = tx.clone();

        thread::spawn(move||{
            let mut res = read_segment(segment, file_path);
            tx.send( res).unwrap();


        });

    }

    for reci in rx_list  {
       let mut abc =  reci.recv().unwrap();
       recive_list.append(&mut abc);
    }

    println!("num found: {}", recive_list.len());

    let mut log = File::create("./logg.txt").unwrap();

    for mut page in recive_list {
        // page.push("\n".parse().unwrap());
        let save_str = format!("{}@{}\n", page.word, page.ipa_pronontiation);
        log.write(save_str.as_ref());
    }


}

pub fn read_xml_title_content(file_path: &'static str) {
    let file: std::fs::File = fs::File::open(file_path).unwrap();

    let mut bufered_reader = io::BufReader::new(file);
    let mut vals: Vec<WiktionaryPage> = Vec::new();
    let mut page_buffer: Vec<String> = Vec::new();
    let mut buffer = String::new();
    let mut bytes_read = 1;
    while bytes_read > 0{


        // for n in 0..10000 {
        buffer.clear();
        bytes_read = bufered_reader.read_line(&mut buffer).unwrap();
        page_buffer.push(buffer.clone());

        // if res == 0 || vals.len() > 10000 {
        //     println!("ferdig ved n: {}", n);
        //     break;
        // }

        if buffer.contains("</page>") {
            // print!("block start \n");
            let page = segment_structs::parse_page_block(&page_buffer);
            if let Option::Some(v) = page {
                vals.push(v);
                // println!("Word: {}", v.word);
                // println!("pronotiation: {}\n\n", v.ipa_pronontiation);
            }

            // let title = buffer
            //     .trim()
            //     .strip_prefix("<title>")
            //     .unwrap()
            //     .strip_suffix("</title>")
            //     .unwrap();
            // vals.push(title.parse().unwrap());
            page_buffer.clear();

            // print!("new block\n\n");
        }
    }
    println!("num found: {}", vals.len());

    let mut log = File::create("./logg.txt").unwrap();

    for mut page in vals {
        // page.push("\n".parse().unwrap());
        let save_str = format!("{}@{}\n", page.word, page.ipa_pronontiation);
        log.write(save_str.as_ref());
    }
}
