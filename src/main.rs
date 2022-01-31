use std::io;

fn main() {
    let input = get_input("Text to analyse : ");
    let nb_letter = count_letter(&input);
    let nb_words = count_letter(&input);
    let nb_centences = count_letter(&input);
    let grade: i32 = 
    println!("Your input is : {}.", input);
}

fn get_input(message: &str) -> String {
    // This function displays an informative string to the user, and return his input
    println!("{}", message);
    let mut user_input = String::from("");
    io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input.");
    user_input
}

fn count_letter(text: &str) -> i32 {
    0
}

fn count_words(text: &str) -> i32 {
    0
}

fn count_sentences(text: &str) -> i32 {
    0
}

fn calculate_grade(nb_letter: i32, nb_words: i32, nb_centences: i32) -> i32 {
    0
}