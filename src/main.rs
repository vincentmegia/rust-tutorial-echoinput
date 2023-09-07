fn main() {
    println!("Enter what you what!");
    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read your inputs");

    println!("Hello human, this is what you typed: {guess}");
}
