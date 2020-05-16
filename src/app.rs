use std::fs::File;
use std::io::Write;
use std::io::Read;

pub fn _cipher_file(plain_text: &str) {
    let mut _alpha: Vec<&str> = " abcdefghijklnmopqrstuvwxyzABCDEFGHIJKLNMOPQRSTUVWXYZ1234567890.,
    ".split("").collect();
    let mut _cipher_alpha: Vec<&str> = " zebrascdfghijklmnopqtuvwxyYXWVUTQPONMLKJIHGFDCSARBEZ0987654321,.
    ".split("").collect();

    let mut _input = String::new();
    let mut _output: String = "".to_string();

    let mut _ifile = File::open(plain_text).expect("Error opening file");

    _ifile.read_to_string(&mut _input).expect("Error reading file");

    let mut _single_char: Vec<&str> = _input.split("").collect();

    let mut _file = File::create("ciphered.txt").expect("error");

    for char in _single_char.iter() {
        let index = _alpha.iter().position(|str| str == char).unwrap();
        _output.push_str(_cipher_alpha[index]);
    }

    _file.write_all(_output.as_bytes()).expect("Unable to write");
    println!("Ciphering successfull.")
}

pub fn _cipher_txt(plain_text: &str) {
    let mut _alpha: Vec<&str> = " abcdefghijklnmopqrstuvwxyz".split("").collect();
    let mut _cipher_alpha: Vec<&str> = " zebrascdfghijklmnopqtuvwxy".split("").collect();
    let mut _single_char: Vec<&str> = plain_text.split("").collect();
    
    for char in _single_char.iter() {
        let index = _alpha.iter().position(|str| str == char).unwrap();
        print!("{}", _cipher_alpha[index])
    }

    println!("\nCiphering successfull.")
}