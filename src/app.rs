pub fn _cipher(plain_text: &str) {
    let mut _alpha: Vec<&str> = " abcdefghijklnmopqrstuvwxyz".split("").collect();
    let mut _cipher_alpha: Vec<&str> = " zebrascdfghijklmnopqtuvwxy".split("").collect();
    let mut _single_char: Vec<&str> = plain_text.split("").collect();
    
    for char in _single_char.iter() {
        let index = _alpha.iter().position(|str| str == char).unwrap();
        print!("{}", _cipher_alpha[index])
    }
} 