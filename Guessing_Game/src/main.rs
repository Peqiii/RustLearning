use std::io;

fn main() {
    println!("WELLCOME TO THE GUESSING GAME!");
    println!("INPUT YOUR GUESS：");
    let mut guess = String::new();
    while 1 == 1 {
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("YOU GASSED: {}", guess);
    }
}
