use std::io;

fn main() {
    let input = get_input("Text to analyse : ").to_lowercase();
    let nb_letter = count_letter(&input);
    let nb_words = count_words(&input);
    let nb_sentences = count_sentences(&input);
    let index: i32 = calculate_index(nb_letter, nb_words, nb_sentences);

    display_results(index);
}

fn display_results(index: i32) {
    // Interpret the index as a grade, and displays it.
    if index < 1 {
        println!("Before grade 1.");
    } else if index > 16 {
        println!("Grade 16+.");
    } else {
        println!("Grade {}.", index)
    }
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
    // Return the nb of letters in a given input ref str.
    let mut count: i32 = 0;
    for character in text.chars() {
        if character as u32 >= 97 && character as u32 <= 122 { // Check if is letter with decimal unicode index
            count += 1;
        }
    }
    count
}

fn count_words(text: &str) -> i32 {
    // Return the nb of words in a given input ref str.
    // Considering a word is a group of char separated by a space.
    let mut count: i32 = 1;
    for character in text.chars() {
        if character == ' ' {
            count += 1;
        }
    }
    count
}

fn count_sentences(text: &str) -> i32 {
    // Return the nb of sentences in a given input ref str.
    // Considering a sentence is a group of char separated by a '.', '?' or '!'.
    let mut count: i32 = 0;
    let ponctuation = ['.', '?', '!'];
    for character in text.chars() {
        if ponctuation.contains(&character) {
            count += 1;
        }
    }
    count
}

fn calculate_index(nb_letter: i32, nb_words: i32, nb_sentences: i32) -> i32 {
    // Calculate the level of a text withe the Coleman-Liau index formula.
    // Takes the nb of letters, words and sentences, and return the index.
    let l = ((nb_letter as f32)/(nb_words as f32))*100.0;
    let s = ((nb_sentences as f32)/(nb_words as f32))*100.0;
    let index= 0.0588 * l - 0.296 * s - 15.8;
    index.round() as i32
}