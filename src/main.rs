#![feature(receiver_trait)]
#![feature(bool_to_option)]
#![feature(map_into_keys_values)]
#![feature(core_intrinsics)]
#![feature(str_split_once)]

extern crate num_cpus;

use cmu_translator::get_word_number_pairs;
use std::io;
use std::io::Write;

pub mod cmu_translator;

// strippe tall og strippe (1)
fn main() {
    //let xml_fp : &'static str = "/home/trygve/Development/projects/number_word_sound_translator/data/enwiktionary-20200901-pages-articles.xml";
    //let logg_fp: &'static str =
    "/home/trygve/Development/projects/number_word_sound_translator/logg.txt";
    let lnp_fp: &'static str =
        "/home/trygve/Development/projects/number_word_sound_translator/letter_number_pairs";

    let cmu_fp: &'static str =
        "/home/trygve/Development/projects/number_word_sound_translator/data/cmudict-0.7b-utf";

    //multithred_parse(xml_fp);

    let translated_words = get_word_number_pairs(lnp_fp, cmu_fp);
    //translated_words.iter().for_each(|l| println!("{}-{}", l.word, l.number));

    // for n in 0..1000 {
    //     if !translated_words
    //         .iter()
    //         .any(|nwp| nwp.number == n.to_string())
    //     {
    //         println!("NO FOR {}", n);
    //     }
    // }

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
            .for_each(|a| println!("{}-{}-{}", a.number, a.word, a.arpabet_pronon.join(" ")));

        //println!("num is {}", nmr);
    }

    //read_xml_title_content(xml_fp);
    //let a = get_letter_number_pair(lnp_fp);

    //let abc = WordNumberTranslator::new(lnp_fp);

    //match_word_to_number(logg_fp, abc);

    // println!("Hello, world!");
}
