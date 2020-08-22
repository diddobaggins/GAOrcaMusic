static NOTES: &'static [&str] = &["C", "D", "E", "f", "G", "A", "B", ".", ".", ".", ".", "."];
static ALL_NOTES: &'static [&str] = &["C", "c", "D", "d", "E", "F", "f", "G", "g", "A", "a", "B"];
static FREQUENCY_MATRIX: &'static [f64] = &[1.0, 0.081, 0.236, 0.029, 0.45, 0.58, 0.0757, 0.83, 0.1025, 0.53, 0.0219, 0.192];

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rand::Rng;
mod compress;
use compress::lz77_compress;

fn main() {
    let popu = gen_new_pop();
    // guide = "DfGD.fGD.DfGABAGf..f..fGG...G......"
    // notes = vec!["C", "D", "E", "f", "G", "A", "B", ".", ".", ".", ".", "."];
    // all_notes = vec!["C", "c", "D", "d", "E", "F", "f", "G", "g", "A", "a", "B"];
    // frequency_matrix = vec![1, .081, .236, .029, .45, .58, .0757, .83, .1025, .53, .0219, .192];

    // for number in 1..5 {
    //     let mut gene = "".to_string();
    //     for number in 1..34 {
    //         let ind = rand::thread_rng().gen_range(0, 9);
    //         gene.push_str(&notes[ind]);
    //     }
    //     let mut name = "i".to_string();
    //     let num = number.to_string();
    //     let ext = ".txt".to_string();
    //     name.push_str(&num);
    //     name.push_str(&ext);
    //
    //     let path = Path::new(&name);
    //     let display = path.display();
    //
    //     // Open a file in write-only mode, returns `io::Result<File>`
    //     let mut file = match File::create(&path) {
    //         Err(why) => panic!("couldn't create {}: {}", display, why),
    //         Ok(file) => file,
    //     };
    //
    //     // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    //     match file.write_all(gene.as_bytes()) {
    //         Err(why) => panic!("couldn't write to {}: {}", display, why),
    //         Ok(_) => println!("successfully wrote to {}", display),
    //     }
    // }
}

fn gen_new_pop<'a>() -> Vec<Vec<Vec<&'a str>>> {
    let mut popu = vec![];
    for i in 0..32 {
        let mut genes = vec![vec![], vec![]];
        for j in 0..35 {
            for z in 0..2 {
                let ind = rand::thread_rng().gen_range(0, 9);
                genes[z].push(NOTES[ind]);
            }
        }
        popu.push(genes);
    }
    popu
}
