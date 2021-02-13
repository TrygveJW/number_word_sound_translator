use std::collections::HashMap;
use std::io::BufRead;
use std::{fs, io};
use unicode_segmentation::UnicodeSegmentation;

struct SoundNumberPair {
    number: i32,
    letter: Vec<String>,
}

pub struct WordNumberPair {
    pub number: String,
    pub word: String,
    pub arpabet_pronon: Vec<String>,
}

struct CmuWord {
    word: String,
    arpabet_pronon: Vec<String>,
}

struct WordNumberTranslator {
    number_sound_map: HashMap<i32, Vec<String>>,
}

impl WordNumberTranslator {
    pub fn new(map_file_name: &'static str) -> WordNumberTranslator {
        let num_map = get_letter_number_pair(map_file_name);

        return WordNumberTranslator {
            number_sound_map: num_map,
        };
    }

    fn translate_cmu_word(&self, word: CmuWord) -> WordNumberPair {
        let mut pronon_num = String::new();
        for pronon in &word.arpabet_pronon {
            // let p = if pronon.contains("1234567890") {
            //     pronon
            //         .chars()
            //         .filter(|c| !c.is_numeric())
            //         .map(|c| c.to_string())
            //         .collect()
            // //let a: Vec<char> =
            // //a.join("")
            // } else {
            //     pronon
            // };
            let mut found = false;
            for (k, v) in &self.number_sound_map {
                for t_value in v {
                    if *t_value == *pronon {
                        found = true;
                        pronon_num.push(k.to_string().parse().unwrap())
                    }
                }
            }
        }

        return WordNumberPair {
            word: word.word,
            number: pronon_num,
            arpabet_pronon: word.arpabet_pronon,
        };
    }
}

fn get_letter_number_pair(file_path: &'static str) -> HashMap<i32, Vec<String>> {
    let file: std::fs::File = fs::File::open(file_path).unwrap();

    let mut bufered_reader = io::BufReader::new(file);

    // the sizes of the map word
    let mut size_map: HashMap<i32, Vec<String>> = HashMap::new();

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

    // size_map.iter().for_each(|(k, v)| {
    //     println!("NUM-{}", k);
    //     for s in v {
    //         println!("{}", s);
    //     }
    //
    //     println!("###############")
    // });
    return size_map;
}

fn strip_newline(inp: String) -> String {
    return match inp.strip_suffix("\n") {
        None => inp,
        Some(strpped) => String::from(strpped),
    };
}

fn load_words(file_name: &'static str) -> Vec<CmuWord> {
    let file: std::fs::File = fs::File::open(file_name).unwrap();

    let mut bufered_reader = io::BufReader::new(file);
    let mut values: Vec<CmuWord> = Vec::new();
    let mut buffer = String::new();

    for n in 0..56 {
        // need to move down exactly 56
        bufered_reader.read_line(&mut buffer);
    }
    buffer.clear();
    let mut bytes_read = bufered_reader.read_line(&mut buffer).unwrap();

    buffer = strip_newline(buffer);

    while bytes_read > 0 {
        let (word, mut pronon) = buffer.split_once("  ").unwrap();

        let pronon_vec: Vec<String> = pronon
            .split(" ")
            .map(|s| s.chars().filter(|c| !c.is_numeric()).collect::<String>())
            .collect();

        let save_obj = CmuWord {
            word: word.parse().unwrap(),
            arpabet_pronon: pronon_vec,
        };

        values.push(save_obj);

        buffer.clear();
        bytes_read = bufered_reader.read_line(&mut buffer).unwrap();
        buffer = strip_newline(buffer);
    }

    return values;
}

pub fn get_word_number_pairs(
    letter_num_file: &'static str,
    cmu_word_file: &'static str,
) -> Vec<WordNumberPair> {
    let translator = WordNumberTranslator::new(letter_num_file);

    let raw_words = load_words(cmu_word_file);

    let mut translated_words: Vec<WordNumberPair> = Vec::new();

    for word in raw_words {
        translated_words.push(translator.translate_cmu_word(word));
    }

    return translated_words;
}

// pub fn multithred_parse(file_path: &'static str) {
//     let segments = get_segment_run_list(file_path);
//
//     let mut recive_list: Vec<SoundNumberPair> = Vec::new();
//     let mut rx_list: Vec<Receiver<Vec<SoundNumberPair>>> = Vec::new();
//     // let (tx,rx): (Sender<Vec<WiktionaryPage>>, Receiver<Vec<WiktionaryPage>>) = mpsc::chanel();
//
//     for segment in segments {
//         let (tx, rx): (Sender<Vec<SoundNumberPair>>, Receiver<Vec<SoundNumberPair>>) =
//             mpsc::channel();
//         rx_list.push(rx);
//         // let tx = tx.clone();
//
//         thread::spawn(move || {
//             let mut res = read_segment(segment, file_path);
//             tx.send(res).unwrap();
//         });
//     }
//
//     for reci in rx_list {
//         let mut abc = reci.recv().unwrap();
//         recive_list.append(&mut abc);
//     }
//
//     println!("num found: {}", recive_list.len());
//
//     let mut log = File::create("./logg.txt").unwrap();
//
//     for mut page in recive_list {
//         // page.push("\n".parse().unwrap());
//         let save_str = format!("{}@{}\n", page.word, page.ipa_pronontiation);
//         log.write(save_str.as_ref());
//     }
// }
