use std::io;

fn main() {
    let mut word = String::new();
    println!("Digite uma palavra para verificar se é um Palindromo.");

    io::stdin()
    .read_line(&mut word)
    .expect("Erro ao ler a palavra!");

    let word = word.trim();

    if is_palindromo(&word) {
        println!("A palavra: {} é um Palindromo.", word);
    }else {
        println!("A palavra: {} não é um Palindromo.", word);
    }
}

fn is_palindromo(word: &str) -> bool {
    let reverse: String = word.chars().rev().collect();
    word == reverse
}