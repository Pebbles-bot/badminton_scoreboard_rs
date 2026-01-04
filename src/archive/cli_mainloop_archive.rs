// external crate imports
use inquire::{CustomType, validator::{Validation}};

// local imports
// mod utils;
// use crate::utils::math;

mod engine;
use crate::engine::state;

mod render;
use crate::render::render_cli;


// setup for CLI argument parser


fn main() {

    // first initialize the gamestate
    let mut current_gamestate : state::Gamestate = state::initialize_singles();
    let validator = |input: &usize| if *input > 2 || *input < 1 {
        Ok(Validation::Invalid("Only integers 1-2 are allowed.".into()))
    } else {
        Ok(Validation::Valid)
    };
    render_cli::render_gamestate(&current_gamestate);

    /////////////////////////////
    // main turn loop begins here
    while !(current_gamestate.is_resolved()) {

        let status = CustomType::<usize>::new("Score?")
            .with_validator(validator)
            .prompt();

        
        match status {
            Ok(status) => {
                println!("Updating the gamestate...");

                // actually see the value inside the status
                match status {
                    1 => {
                        println!("Team 1 Scores!");
                        state::score_team1(&mut current_gamestate);
                    },
                    2 => {
                        println!("Team 2 Scores!");
                        state::score_team2(&mut current_gamestate);
                    },
                    _ => {
                        continue;
                    }    
                }

                // after check to see if the game is over
                state::check_winning(&mut current_gamestate);

                // after that render the gamestate again
                render_cli::render_gamestate(&current_gamestate);
            },
            Err(err) => { println!("Error while publishing your status: {}", err); break; },
        }
    }

    
}


