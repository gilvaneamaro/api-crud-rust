use std::io;

fn main() {
    println!("Hello,{0}! Thats your first code in Rust! Lets change to {1}", "Gilvane Amaro", "Ruby?");

    let mut input = String::new();

    println!("Lets play a game, enter with the word:");

    io::stdin().read_line(&mut input).expect("Falha ao ler a linha");
    let input = input.trim();

    for i in 1..5 {
        println!("i: {}", i);
    }
    println!("Sua palavra foi {}", input);
}

