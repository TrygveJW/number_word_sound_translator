#![feature(receiver_trait)]
#![feature(bool_to_option)]
#![feature(map_into_keys_values)]
#![feature(core_intrinsics)]
#![feature(str_split_once)]

extern crate num_cpus;

use cmu_translator::get_word_number_pairs;
use std::io;
use std::io::Write;
use std::path::Path;

pub mod cmu_translator;

fn main() {
    let letter_num_path_str: &'static str = "./letter_number_pairs";

    let letter_num_fp = Path::new(letter_num_path_str);

    let cmu_path_string: &'static str = "./data/cmudict-0.7b-utf";
    let cmu_fp = Path::new(cmu_path_string);

    let translated_words = get_word_number_pairs(letter_num_fp, cmu_fp);

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
            .for_each(|a| println!("{:3} - {:10} - {}", a.number, a.word, a.arpabet_pronon.join(" ")));
    }

}
