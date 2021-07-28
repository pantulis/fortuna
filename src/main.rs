use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    env::var,
};

use rand::Rng;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

// ---

fn main() {
    let phortune_file = format!("{}/.PHORTUNE.DAT", var("HOME").unwrap());
    let lines = lines_from_file(&phortune_file).expect("Could not load lines");
    let mut items = 0;

    for line in &lines {
        if line == "<ITEM>" {
            items = items + 1;
        }
    }


    let mut rng = rand::thread_rng();
    let selected_fortune = rng.gen_range(0..items-1);
    let mut counted_items = 0;

    println!("-PHORTUNE---------------------------------------");
    println!("");
     
    for line in &lines {
         if line == "<ITEM>" {
             counted_items = counted_items + 1;
         } else if counted_items == selected_fortune {
             println!("{}",line);
         }
    };
    println!("");
    println!("------------------------------------------------");
}