#![feature(receiver_trait)]
#![feature(bool_to_option)]
#![feature(map_into_keys_values)]
#![feature(core_intrinsics)]
#![feature(str_split_once)]

extern crate num_cpus;

use crate::cmu_parser::parse_cmu;
use crate::nst_parser::parse_nst;
use crate::number_word_translator::WordNumberTranslator;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::exit;

pub mod cmu_parser;
mod nst_parser;
mod number_word_translator;
mod symbol_maps;
mod util;

fn main() {
    let letter_num_path_str: &'static str = "./letter_number_pairs";
    let letter_num_fp = Path::new(letter_num_path_str);
    let mut nwt = WordNumberTranslator::new(letter_num_fp);

    let cmu_path_string: &'static str = "./data/cmudict-0.7b-utf";
    let cmu_fp = Path::new(cmu_path_string);
    let res = parse_cmu(cmu_fp);
    nwt.add_new_words(res);

    let letter_num_path_str: &'static str = "./NST_dat.pron";
    let nst_fp = Path::new(letter_num_path_str);
    let nst_res = parse_nst(nst_fp);
    nwt.add_new_words(nst_res);

    nwt.start_loop();

    // let letter_num_path_str: &'static str = "./letter_number_pairs";
    //
    // let letter_num_fp = Path::new(letter_num_path_str);
    //
    // let cmu_path_string: &'static str = "./data/cmudict-0.7b-utf";
    // let cmu_fp = Path::new(cmu_path_string);
    //
    // let translated_words = get_word_number_pairs(letter_num_fp, cmu_fp);
    //
    // // for wnp in &translated_words {
    // //     print!(
    // //         "{} - {} - {}\n",
    // //         wnp.number,
    // //         wnp.word,
    // //         wnp.arpabet_pronon.join(" ")
    // //     )
    // // }
    // // exit(0);
    // loop {
    //     print!("input number: ");
    //     let _ = io::stdout().flush();
    //
    //     let mut nmr = String::new();
    //
    //     io::stdin()
    //         .read_line(&mut nmr)
    //         .expect("Failed to read line");
    //
    //     translated_words
    //         .iter()
    //         .filter(|nwp| nwp.number == nmr.strip_suffix("\n").unwrap())
    //         .for_each(|a| {
    //             println!(
    //                 "{:3} - {:10} - {}",
    //                 a.number,
    //                 a.word,
    //                 a.arpabet_pronon.join(" ")
    //             )
    //         });
    // }
}
