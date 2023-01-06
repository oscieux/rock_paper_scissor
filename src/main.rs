// use std::{io, collections::{HashMap}};
// use rand::seq::SliceRandom;

// fn main() {
//     // launch program
//     println!("Welcome to rock, paper, scissor game!");
//     println!("choose your statement (r, p or s)");

//     // let choices = vec!["r".to_string(), "p".to_string(), "s".to_string()];
//     let choices_alt = HashMap::from([
//         ("r", "rock"),
//         ("p", "paper"),
//         ("s", "scissors",)
//     ]);

//     let choices:Vec<&str> = choices_alt.keys().cloned().collect();
//     // println!("{:?}", choices_keys);
    
//     // initialize user_choice
//     let mut user_choice = String::new();
//     io::stdin().read_line(&mut user_choice)
//         .expect("failed to read statement"); 

//     // initialize computer choice
//     let computer_choice = choices
//         .choose(&mut rand::thread_rng());

//     // declare variable
//     let user_choice_trim = user_choice.trim();
//     let user_choice_convert = choices_alt.get(user_choice_trim).unwrap().to_owned();

//     let computer_choice_string = computer_choice.unwrap().to_owned();
//     let computer_choice_convert = choices_alt.get(computer_choice_string).unwrap().to_owned();

//     // display user and computer choices
//     println!("User choice : {}", user_choice_convert);
//     println!("Computer choice : {}", computer_choice_convert);

//     // // display the winner
//     if user_choice_convert == computer_choice_convert {
//         println!("Draw")
//     } else if (user_choice_trim == "r" && computer_choice_string == "s") 
//     | (user_choice_trim == "s" && computer_choice_string == "p")
//     | (user_choice_trim == "p" && computer_choice_string == "r"){
//         println!(
//             "{} wins against {}. YOU win",
//             capitalize_first_letter(user_choice_convert),
//             computer_choice_convert
//         );
//     } else {
//         println!(
//             "{} wins against {}. COMPUTER wins",
//             capitalize_first_letter(computer_choice_convert),
//             user_choice_convert
//         );
//     };

// }

// fn capitalize_first_letter(s: &str) -> String {
//     s[0..1].to_uppercase() + &s[1..]
// }

use fltk::{prelude::*, *};
use rand::seq::SliceRandom;
fn main() {
    let app = app::App::default();
    
    let mut wind = window::Window::new(
        100,
        100,
        400,
        300,
        "Welcome to rock, paper, scissor game!"
    );
    
    let mut _label_choice = frame::Frame::new(0, 0, 400, 40, "SELECT YOUR WEAPON :");
    
    // buttons initialization
    let x_coord = 65;
    let y_coord = 40;

    let rock_button = button::CheckButton::new(
        x_coord, 
        y_coord, 
        80, 
        40, 
        "Rock"
    );
    let paper_button = button::CheckButton::new(
        x_coord + 100, 
        y_coord, 
        80, 
        40,
         "Paper"
    );
    let scissor_button = button::CheckButton::new(
        x_coord + 200, 
        y_coord, 
        80, 
        40, 
        "Scissor"
    );

    let mut submit_button = button::Button::new(
        x_coord + 100, 
        y_coord + 50, 
        80, 
        20, 
        "Submit"
    );

    let _label_user = frame::Frame::new(x_coord + 25, y_coord + 100, 80, 20, "User Choice");
    let mut output_user = output::Output::new(
        x_coord + 25, 
        y_coord + 120, 
        80, 
        20, 
        ""
    );

    let _label_versus = frame::Frame::new(x_coord + 100, y_coord + 110, 80, 40, "VS");

    let _label_computer = frame::Frame::new(x_coord + 175, y_coord + 100, 80, 20, "Computer Choice");
    let mut output_computer = output::Output::new(
        x_coord + 175, 
        y_coord + 120, 
        80, 
        20, 
        ""
    );

    let _label_winner=frame::Frame::new(x_coord, y_coord + 175, 80, 20, "THE WINNER IS : ");
    let mut output_winner = output::Output::new(
        x_coord + 100, 
        y_coord + 175, 
        100, 
        20, 
        ""
    );

    wind.end();
    wind.show();

    submit_button.set_callback(move |_submit_button| {
        
        // initialize choice variables
        let choices = vec!["Rock".to_string(), "Paper".to_string(), "Scissor".to_string()];
        let mut user_choice = String::new();
        let computer_choice = choices
            .choose(&mut rand::thread_rng());

        if rock_button.value() {
            user_choice = "Rock".to_string();
            // frame.set_label("You choose Rock!");
        }
        if paper_button.value() {
            user_choice = "Paper".to_string();
            // frame.set_label("You choose Paper!")
        }
        if scissor_button.value() {
            user_choice = "Scissor".to_string();
            // frame.set_label("You choose Scissor!")
        }

        let computer_choice_string = computer_choice.unwrap().to_owned();

        output_user.set_value(user_choice.as_str());
        output_computer.set_value(computer_choice_string.as_str());

        // display the winner
        if user_choice == computer_choice_string {
            output_winner.set_value("NOBODY");
        } else if (user_choice.as_str() == "Rock" && computer_choice_string == "Scissor") 
        | (user_choice.as_str() == "Scissor" && computer_choice_string == "Paper")
        | (user_choice.as_str() == "Paper" && computer_choice_string == "Rock"){
            output_winner.set_value("YOU");
        } else {
            output_winner.set_value("COMPUTER");
        };
    });

    app.run().unwrap();

}