use std::collections::{HashMap, HashSet};
use std::io::{BufRead, Write};
use std::path::Path;
use std::{fs, io};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct WordIpaPair {
    pub(crate) word: String,
    pub(crate) ipa_symbols: Vec<String>,
}

#[derive(Debug)]
pub struct WordNumberPair {
    pub number: String,
    pub word: String,
    pub ipa_symbols: Vec<String>,
}

pub struct WordNumberTranslator {
    number_sound_map: HashMap<i32, Vec<String>>,
    translated_words: Vec<WordNumberPair>,
}

impl WordNumberTranslator {
    pub fn new(file_path: &Path) -> WordNumberTranslator {
        let map = get_letter_number_pair(file_path);
        return WordNumberTranslator {
            number_sound_map: map,
            translated_words: Vec::new(),
        };
    }

    pub fn get_ipa_symbol_frequencies(&self){
        let all_symbols: Vec<_> = self.translated_words.iter().flat_map(|wnp| wnp.ipa_symbols.iter()).collect();
        let used_syms : HashSet<_> = self.number_sound_map.iter().flat_map(|(_,b)| b.iter()).collect();

        let mut tracker: HashMap<&String, i32> = HashMap::new();

        for sym in all_symbols{
            // if used_syms.contains(sym){
                if let Some(v) = tracker.get_mut(sym){
                    *v += 1;
                }else {
                    tracker.insert(sym,1);
                }
            // }
        }
        let mut tracked_list: Vec<(&String, i32)> = tracker.into_iter().collect();
        tracked_list.sort_by_key(|v| v.1);
        tracked_list.reverse();
        for (k,v) in tracked_list{
            let mut target_num = -1;

            let num = for (num, syms) in &self.number_sound_map{
                if syms.contains(k){
                    target_num = num.clone();
                    break;
                }
            };

            println!("{:<8?} - {:<2?} - {:?}", v,target_num, k)
        }


    }

    fn translate_word(&self, word: WordIpaPair) -> WordNumberPair {
        let mut pronon_num = String::new();
        for pronon in &word.ipa_symbols {
            let mut found = false;
            for (k, v) in &self.number_sound_map {
                for t_value in v {
                    if *t_value == *pronon {
                        found = true;
                        //println!("{}",k.to_string());
                        pronon_num.push_str(&*k.to_string())
                    }
                    if found {
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }

        return WordNumberPair {
            word: word.word,
            number: pronon_num,
            ipa_symbols: word.ipa_symbols,
        };
    }

    pub fn add_new_words(&mut self, words: Vec<WordIpaPair>) {
        for pair in words {
            let word_num_dat = self.translate_word(pair);
            self.translated_words.push(word_num_dat)
            // self.number_sound_map.insert(word_num_dat.number.clone().parse().unwrap(), word_num_dat)
        }
    }

    pub fn start_loop(&self) {
        let translated_words = &self.translated_words;

        loop {
            print!("input number: ");
            let _ = io::stdout().flush();

            let mut nmr = String::new();

            io::stdin()
                .read_line(&mut nmr)
                .expect("Failed to read line");

            translated_words
                .iter()
                .filter(|nwp| nwp.number == nmr.strip_suffix("\n").unwrap())
                .for_each(|a| {
                    println!(
                        "{:3} - {:10} - {}",
                        a.number,
                        a.word,
                        a.ipa_symbols.join(" ")
                    )
                });
        }
    }
}

fn get_letter_number_pair(file_path: &Path) -> HashMap<i32, Vec<String>> {
    let file: std::fs::File = fs::File::open(file_path).unwrap();

    let mut bufered_reader = io::BufReader::new(file);

    // the sizes of the map word
    let mut size_map: HashMap<i32, Vec<String>> = HashMap::new();

    let mut buffer = String::new();
    let mut bytes_read = bufered_reader.read_line(&mut buffer).unwrap();
    while bytes_read > 0 {
        if buffer.len() > 0 {
            let mut number_buf: Vec<&str> = Vec::new();
            let mut chars = buffer.graphemes(true);
            let mut char = chars.next().unwrap();
            while char != "=" && char != "" {
                number_buf.push(char.clone());
                char = chars.next().unwrap();
            }

            let num: i32 = number_buf.join("").parse::<i32>().unwrap();

            let mut tmp_val: Vec<String> = Vec::new();

            for character in chars {
                if character.eq(",") || character.eq("\n") {
                    if !tmp_val.is_empty() {
                        if !size_map.contains_key(&num) {
                            size_map.insert(num.clone(), Vec::new());
                        }
                        size_map
                            .get_mut(&num)
                            .unwrap()
                            .push(tmp_val.clone().join(""));
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

    return size_map;
}
