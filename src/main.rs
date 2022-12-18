use std::io;

fn main() {
    // Display the game instructions
    println!("Welcome to Rock, Paper, Scissors!\n");
    println!("To play, enter one of the following options:");
    println!("1. Rock");
    println!("2. Paper");
    println!("3. Scissors\n");

    // Play the game
    loop {
        // Get the player's choice
        println!("Enter your choice:");
        let player_choice = read_input();

        // Generate the computer's choice
        let computer_choice = generate_computer_choice();

        // Determine the winner
        let result = determine_winner(player_choice, computer_choice);

        // Display the result
        println!("The computer chose {}.", computer_choice);
        match result {
            0 => println!("It's a tie!"),
            1 => println!("You win!"),
            2 => println!("The computer wins!"),
            _ => println!("Invalid choice. Please try again.")
        }

        // Prompt the player to play again
        println!("\nPlay again? (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() != "y" {
            break;
        }
    }
}

// Read a single line of input from the user and parse it as an integer
fn read_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid number")
}

// Generate a random choice for the computer
fn generate_computer_choice() -> &'static str {
    let choice = rand::random::<u8>() % 3;
    match choice {
        0 => "Rock",
        1 => "Paper",
        2 => "Scissors",
        _ => "Invalid"
    }
}

// Determine the winner of the game
fn determine_winner(player_choice: i32, computer_choice: &str) -> i32 {
    let player_choice = player_choice - 1;
    let computer_choice = match computer_choice {
        "Rock" => 0,
        "Paper" => 1,
        "Scissors" => 2,
        _ => 3
    };

    match (player_choice, computer_choice) {
        (0, 0) => 0,
        (0, 1) => 2,
        (0, 2) => 1,
        (1, 0) => 1,
        (1, 1) => 0,
        (1, 2) => 2,
        (2, 0) => 2,
        (2, 1) => 1,
        (2, 2) => 0,
        _ => 3
    }
}
