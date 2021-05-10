use std::env;
use std::fs;
use std::process::exit;
use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let arg = env::args().skip(1).collect::<Vec<String>>();
    if arg[0].clone().contains("-") {
        if arg[1].clone().contains("w") {
            let Words = wordnum(arg[2..].to_owned());
            for i in 1..arg.len() {
                println!("Total Number Of Words");
                println!("{}",Words[i-1]);
            }
        }
        else if arg[1].clone().contains("c") {
            let totalChars = charnum(arg[2..].to_owned());
            for i in 1..arg.len() {
                println!("Total Number Of Characters");
                println!("{}",totalChars[i-1]);
            }
        }
        else if arg[1].clone().contains("l"){
            let Lines = linenum(arg[2..].to_owned());
            for i in 1..arg.len() {
                println!("Total Number Of lines");
                println!("{}",Lines[i-1]);
            }
        }
    }
    else {
        let Words = wordnum(arg[0..].to_owned());
        let Lines = linenum(arg[0..].to_owned());
        let totalChars = charnum(arg[0..].to_owned());

        for i in 0..arg.len() {
            println!("Total Number Of lines");
            println!("{}",Lines[i]);
            println!("Total Number Of Words");
            println!("{}",Words[i]);
            println!("Total Number Of Characters");
            println!("{}",totalChars[i]);
        }
    }
}

fn Files(file: String) -> Vec<String> {
    let stuff = fs::read_to_string(file).expect("Unable to read file");
    let split = stuff.split("\n");
    let mut vec: Vec<String> = Vec::new();
    for s in split {
        vec.push(s.to_string());
    }
    return vec;
}

fn wordnum(files: Vec<String>) -> Vec<usize> {
    let mut Words:Vec<usize> =Vec::new();
    for file in files {
        let lines = Files(file);
        let mut words = 0;
        for line in lines {
            let split = line.split(" ");
            for s in split {
                words+=1;
            }
        }
        Words.push(words);
    }
    return Words;
}

fn charzard(string: String, index: usize) -> char {
    let bytes = string.into_bytes();
    let c: char = bytes[index] as char;
    return c;
}

fn charnum(files: Vec<String>) -> Vec<usize> {
    let mut totalChars:Vec<usize> =Vec::new();
    for file in files {
        let lines = Files(file);
        let mut chars = 0;
        for line in lines {
            chars += line.chars().count();
        }
        totalChars.push(chars);
    }
    return totalChars;
}

fn linenum(files: Vec<String>) -> Vec<usize>{
    let mut Lines:Vec<usize> =Vec::new();
    for file in files {
        Lines.push(Files(file).len());
    }
    return Lines;
}

