use std::collections::HashSet;
use std::io;
use std::io::Write;

pub fn main() {
    let secret_word: &str = &"Learn Rust".to_lowercase();
    let secret_word_no_spaces = secret_word.replace(" ", "");
    let set_of_correct_letters: HashSet<_> = secret_word_no_spaces.chars().collect();
    let mut guesses: HashSet<char> = HashSet::new();
    let mut correct_chars: HashSet<char> = HashSet::new();

    let mut num_guesses: i32 = 0;
    let max_incorrect_guesses = 6;

    //INTRO
    println!("\n\n\n\n\n\n***** Welcome to hangman! *****\n");
    println!("Lets begin\n");

    /*
     * Beginning of game
     */
    while num_guesses < max_incorrect_guesses {
        println!(
            "Attempts remaining: {}",
            max_incorrect_guesses - num_guesses
        );
        println!("Guessed characters: {:?} \n", guesses);

        let display_word: String = secret_word
            .chars()
            .map(|c| if correct_chars.contains(&c) { c } else { '_' })
            .collect();

        if set_of_correct_letters == correct_chars {
            println!("YOU DID IT");
            println!("/////////////////////////");
            println!("Game over. The secret word was: {}", secret_word);
            println!("Number of guesses: {}", num_guesses);
            println!("/////////////////////////");
            break;
        }

        println!("Word: {}", display_word);
        print!("Enter a guess: ");

        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to get input");

        let guess = match input.chars().next() {
            Some(c) => c.to_ascii_lowercase(),
            None => {
                println!("Invalid input. Please enter a single character.");
                continue;
            }
        };

        println!("");
        if guesses.contains(&guess) {
            println!("\nYou've already guessed that character!!");
            continue;
        }
        if secret_word.contains(guess) {
            println!("Correct guess!");
            correct_chars.insert(guess);
            guesses.insert(guess);

        } else if !guess.is_alphabetic() {
            println!("Invalid character!! \n")
        } else {
            println!("Incorrect guess.");
            num_guesses += 1;
            guesses.insert(guess);
        }
    }

    println!("You lose :(")
}
