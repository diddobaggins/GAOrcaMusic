static NOTES: &'static [&str] = &["C", "D", "E", "f", "G", "A", "B", ".", ".", ".", ".", ".", "."];
static ALL_NOTES: &'static [&str] = &["C", "c", "D", "d", "E", "F", "f", "G", "g", "A", "a", "B"];
static FREQUENCY_MATRIX: &'static [f64] = &[0.3, 0.081, 0.236, 0.029, 0.45, 0.58, 0.0757, 0.83, 0.1025, 0.53, 0.0219, 0.192];

use std::time::{Instant};
use rand::Rng;
mod compress;
use compress::lz77_compress;
use std::cmp;
mod median;
use statrs::statistics::Median;
use rand::distributions::{Normal, Distribution};



fn main() {
    let guide = "DfGD.fGD.DfGABAGf..f..fGG...G......".to_string();
    let mut fitnesses_each_gen = vec![];
    let mut cons_each_gen = vec![];
    let mut dist_each_gen = vec![];
    let mut popu = gen_new_pop();
    for i in 0..4 { //iterate through each generation
        let mut fitness = vec![];
        let mut d = vec![];
        let mut c = vec![];
        for indiv in popu.iter() { //go through every individual in population and add fitness to vector
            let distance = ncd_inverse(&indiv[0], &guide);
            let consonance = cons(indiv);
            d.push(distance.clone());
            c.push(consonance.clone());
            fitness.push(distance + consonance);
        }
        let threshold = &fitness.median();
        fitnesses_each_gen.push(get_avg(&fitness)); //used to get averages of each generation for graphing purposes
        cons_each_gen.push(get_avg(&c));
        dist_each_gen.push(get_avg(&d));
        let mating = get_mating_pool(&popu, &fitness, *threshold); //get vector of parents who are supposed to mate
        if i == 3 { //if at the end of our algorithm, exit
            let max_value = fitness.iter().cloned().fold(-1./0. /* -inf */, f64::max);
            let index = fitness.iter().position(|&r| r == max_value).unwrap();
            println!("{:?}", popu[index]);
            println!("{:?}", fitnesses_each_gen);
            println!("{:?}", cons_each_gen);
            println!("{:?}", dist_each_gen);
            break;
        }
        popu = update(mating); //update mating pool to be fittest parents + created children
    }
}

fn get_avg(fitnesses: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;
    for i in 0..fitnesses.len() {
        sum+=fitnesses[i];
    }
    sum / fitnesses.len() as f64
}

fn cons(indiv: &Vec<String>) -> f64 { //returns consonance value
    let mut cons: f64 = 0.0;
    for i in 0..indiv[0].len() {
        let note_1 = &indiv[0][i..i+1];
        let note_2 = &indiv[1][i..i+1];
        if note_1 == "." && note_2 != "." {
            cons += 0.0;
            continue;
        }
        else if note_1 != "." && note_2 == "." {
            cons += 0.7;
            continue;
        }
        else if note_1 == "." && note_2 == "." {
            cons += 0.35;
            continue;
        }
        let one = ALL_NOTES.iter().position(|&r| r == note_1).unwrap();
        let two = ALL_NOTES.iter().position(|&r| r == note_2).unwrap();
        let i1 = one as i32;
        let i2 = two as i32;
        let separation = cmp::max(i1-i2, i2-i1) as usize;
        cons += FREQUENCY_MATRIX[separation];
    }
    cons / 15.0
}

fn get_mating_pool(popu: &Vec<Vec<String>>, fitness: &Vec<f64>, threshold: f64) -> Vec<Vec<String>> { //gets mating pool
    let mut mating = vec![];
    let mut checker = vec![];
    for i in 0..popu.len() {
        if fitness[i] >= threshold {
            let popu_i_copy = popu[i].clone();
            mating.push(popu_i_copy);
            checker.push(fitness[i]);
        }
    }
    mating = mating[..16].to_vec();
    mating
}

fn gen_new_pop<'a>() -> Vec<Vec<String>> { //randomizes first generation
    let mut popu = vec![];
    for _i in 0..32 {
        let mut genes = vec!["".to_string(), "".to_string()];
        for _j in 0..35 {
            for z in 0..2 {
                let ind = rand::thread_rng().gen_range(0, 9);
                genes[z].push_str(NOTES[ind]);
            }
        }
        popu.push(genes);
    }
    popu
}

fn update(mating: Vec<Vec<String>>) -> Vec<Vec<String>> { //returns the population after chlidren is added to it
    let mut new_popu = mating.clone();
    for i in (0..mating.len()).step_by(2) {
        let mom = &mating[i];
        let dad = &mating[i+1];
        let mating_results = make_baby(mom, dad);
        new_popu.push(mating_results.0);
        new_popu.push(mating_results.1);
    }
    new_popu
}

fn make_baby(mom: &Vec<String>, dad: &Vec<String>) -> (Vec<String>, Vec<String>) { //mates two parents
    let mut mutate_1 = 0;
    let mut mutate_2 = 0;
    let mut mutate_3 = 0;
    let mut mutate_4 = 0;
    let mut mutate_note_1 = "".to_string();
    let mut mutate_note_2 = "".to_string();
    let mut mutate_note_3 = "".to_string();
    let mut mutate_note_4 = "".to_string();
    let normal = Normal::new(18.0, 1.0);
    let r1 = normal.sample(&mut rand::thread_rng()) as usize;
    let r2 = normal.sample(&mut rand::thread_rng()) as usize;
    let r3 = normal.sample(&mut rand::thread_rng()) as usize;
    let r4 = normal.sample(&mut rand::thread_rng()) as usize;
    // make first baby
    let mut baby1 = format!("{}{}", &mom[0][..r1], &dad[0][r1..]);
    let mut baby2 = format!("{}{}", &mom[1][..r2], &dad[1][r2..]);
    if rand::thread_rng().gen_range(0,100) < 70 {
        mutate_1 = rand::thread_rng().gen_range(0, baby1.len());
        mutate_note_1 = NOTES[rand::thread_rng().gen_range(0, 13)].to_string();
        baby1 = format!("{}{}{}", &baby1[..mutate_1].to_string(), mutate_note_1, baby1[mutate_1+1..].to_string());
    }
    if rand::thread_rng().gen_range(0,100) < 70 {
        mutate_2 = rand::thread_rng().gen_range(0, baby1.len());
        mutate_note_2 = NOTES[rand::thread_rng().gen_range(0, 13)].to_string();
        baby2 = format!("{}{}{}", &baby2[..mutate_2].to_string(), mutate_note_2, baby1[mutate_2+1..].to_string());
    }
    // make second baby
    let mut baby3 = format!("{}{}", &dad[0][..r3], &mom[0][r3..]);
    let mut baby4 = format!("{}{}", &dad[1][..r4], &mom[1][r4..]);
    if rand::thread_rng().gen_range(0,100) < 70 {
        mutate_3 = rand::thread_rng().gen_range(0, baby1.len());
        mutate_note_3 = NOTES[rand::thread_rng().gen_range(0, 13)].to_string();
        baby3 = format!("{}{}{}", &baby3[..mutate_3].to_string(), mutate_note_3, baby3[mutate_3+1..].to_string());
    }
    if rand::thread_rng().gen_range(0,100) < 70 {
        mutate_4 = rand::thread_rng().gen_range(0, baby1.len());
        mutate_note_4 = NOTES[rand::thread_rng().gen_range(0, 13)].to_string();
        baby4 = format!("{}{}{}", &baby4[..mutate_4].to_string(), mutate_note_4, baby4[mutate_4+1..].to_string());
    }
    let baby_first = vec![baby1, baby2];
    let baby_second = vec![baby3, baby4];
    (baby_first, baby_second)
}

fn ncd_inverse(indiv: &String, guide: &String) -> f64 { //returns distance value
    //println!("{:?}, {:?}", indiv, guide);
    let x = lz77_compress(indiv.as_bytes());
    let y = lz77_compress(guide.as_bytes());
    let indiv_guide = format!("{}{}", indiv, guide);
    //println!("{:?}", indiv_guide);
    let guide_indiv = format!("{}{}", guide, indiv);
    let xy = lz77_compress(indiv_guide.as_bytes());
    let yx = lz77_compress(guide_indiv.as_bytes());
    //println!("x: {:?} y: {:?} xy: {:?} yx: {:?}", x, y, xy, yx);
    let numerator = cmp::max(xy.len() - x.len(), yx.len() - y.len()) as f64;
    let denominator = cmp::max(x.len(), y.len()) as f64;
    1.0 / (numerator / denominator)
}
