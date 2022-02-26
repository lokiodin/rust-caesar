use std::{io, fs};

use clap::Parser;

use modulo::Mod;

mod cli;
use cli::Cli;

fn rotate_text(text: String, k: usize) -> String {
    let lower_alphabet: String = String::from("abcdefghijklmnopqrstuvwxyz");
    let upper_alphabet: String = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");


    let mut res = String::with_capacity(text.len());
    for c in text.chars() {
        if c.is_uppercase() {
            let i = upper_alphabet.find(c);
            if let Some(indice) = i {   //It is a letter
                let ind = (indice+k).modulo(upper_alphabet.len());
                res.insert_str(res.len(), upper_alphabet.get(ind..ind+1).unwrap());
            } else {
                res.insert(res.len(), c);
            }
        } else {
            let i = lower_alphabet.find(c);
            if let Some(indice) = i {   //It is a letter
                let ind = (indice+k).modulo(upper_alphabet.len());
                res.insert_str(res.len(), lower_alphabet.get(ind..ind+1).unwrap());
            }
            else {
                res.insert(res.len(), c);
            }
        }
    }
    res
}

fn main() {
    let args = Cli::parse();

    if let Some(text) = args.text {
        println!("{}", rotate_text(text, args.key));
    } else if let Some(textfile) = args.textfile {
        let text = fs::read_to_string(textfile)
            .expect("no filetext");
        println!("{}", rotate_text(text, args.key));
    } else {
        let mut text = String::new();
        
        io::stdin()
        .read_line(&mut text)
        .expect("Échec de la lecture de l'entrée utilisateur");
        
        println!("{}", rotate_text(text, args.key));
    }

}
