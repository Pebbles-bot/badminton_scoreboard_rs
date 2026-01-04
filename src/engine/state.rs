// ///////////////////////////////////////////////////////////
/* Copyright (C) 2024 Faseeh Irfan - All Rights Reserved
 * You may use, distribute and modify this code under the
 * terms of the CC-BY Creative commons license.
 *
 * You should have received a copy of the CC- license with
 * this file. If not, look it up or something idk.
 */
// ///////////////////////////////////////////////////////////
#![allow(dead_code)]


/// Struct that contains the gamestate
/// The court is modeled like so:
/// this is not a good modeling; the servestate management functions are not mathematically elegant or side-symmetric
///     _________________
/// P0 |   Q0   |   Q1   | P1
///    |________|________|
/// P2 |   Q2   |   Q3   | P3
///    |________|________|
/// 


struct Player {
    quadrant: i32,
    is_serving: bool,
}

// only really for use in doubles
struct Team {
    team_score: i32,
    next_serving_player: i32,
    pub quadrant_p1: i32,
    pub quadrant_p2: i32,
}


pub enum Gamestate {
    Singles {
        resolved: bool,
        scoretarget: i32,
        last_scored_team: i32,
        quadrant_p1: i32,
        quadrant_p2: i32,
        name_team1: &'static str,
        score_team1: i32,
        name_team2: &'static str,
        score_team2: i32,
        score_array: Vec<i32>           // tracks full score history for streak data metrics
    },
    Doubles {
        resolved: bool,
        scoretarget: i32,
        last_scored_team: i32,
        quadrant_p1: i32,
        quadrant_p2: i32,
        name_team1: &'static str,
        score_team1: i32,
        name_team2: &'static str,
        score_team2: i32,
        score_array: Vec<i32>           // tracks full score history for streak data metrics
    }
}

impl Gamestate {

    // quick and dirty method on the enum for grabbing reso status
    pub fn is_resolved(&self) -> &bool {
        match &self {
            Gamestate::Singles{resolved, ..} => { 
                return resolved;
            },
            Gamestate::Doubles{resolved, ..} => {
                return resolved;
            },
        }
    }

    // quick and dirty method on the enum for grabbing scores
    pub fn get_scores(&self) -> (&i32, &i32) {
        match &self {
            Gamestate::Singles{score_team1, score_team2, ..} => { 
                return (score_team1, score_team2);
            },
            Gamestate::Doubles{score_team1, score_team2, ..} => {
                return (score_team1, score_team2);
            },
        }
    }

    // quick and dirty method on the enum for grabbing names
    pub fn get_names(&self) -> (&str, &str) {
        match &self {
            Gamestate::Singles{name_team1, name_team2, ..} => { 
                return (name_team1, name_team2);
            },
            Gamestate::Doubles{name_team1, name_team2, ..} => {
                return (name_team1, name_team2);
            },
        }
    }

    pub fn get_scorelog(&self) -> &Vec<i32> {
        match &self {
            Gamestate::Singles{score_array, ..} => { 
                return score_array;
            },
            Gamestate::Doubles{score_array, ..} => {
                return score_array;
            },
        }
    }

}


/// make a new gamestate of a singles game
/// a constructor of sorts
pub fn initialize_singles() -> Gamestate {

    return Gamestate::Singles{
        resolved: false,
        scoretarget: 21,
        last_scored_team: 0,
        quadrant_p1: 2,
        quadrant_p2: 1,
        name_team1: "Player 1",
        score_team1: 0,
        name_team2: "Player 2",
        score_team2: 0,
        score_array: Vec::new(),      
    };
}


/// make a new gamestate of a singles game
/// a constructor of sorts
pub fn initialize_doubles() -> Gamestate {

    return Gamestate::Doubles{
        resolved: false,
        scoretarget: 21,
        last_scored_team: 0,
        quadrant_p1: 0,
        quadrant_p2: 2,
        name_team1: "Player 1",
        score_team1: 0,
        name_team2: "Player 2",
        score_team2: 0,
        score_array: Vec::new(),      
    };
}



/// checks if the game is over or not
/// baseline there are two states:
/// score_team1 = scoretarget or score_team2 = scoretarget
/// deuces have to be detected by noticing that ((2 * scoretarget) - (state.score_team1 + state.score_team2) == 2); i.e. 42-20-20=2
/// deuces will raise the scoretarget if the trailing or neutral player scores, so deuces don't need implementation in this function
pub fn check_winning(state: &mut Gamestate) -> usize {

    match state {
        Gamestate::Singles{resolved, scoretarget, score_team1, score_team2, ..} => { 
            if *scoretarget - *score_team1 == 0 {
                println!("Game over! Player 1 wins!");
                *resolved = true;
                return 1;
            } else if *scoretarget - *score_team2 == 0 {
                println!("Game over! Player 2 Wins!");
                *resolved = true;
                return 2;
            }
        },

        Gamestate::Doubles{resolved, scoretarget, score_team1, score_team2, ..} => {
            if *scoretarget - *score_team1 == 0 {
                println!("Game over! Team 1 wins!");
                *resolved = true;
                return 1;
            } else if *scoretarget - *score_team2 == 0 {
                println!("Game over! Team 2 Wins!");
                *resolved = true;
                return 2;
            } 
        },
    }
    
    return 0;
}



/// increments the score of team_1
/// also responsible for changing the score target (or scores) in the event of a decude
/// args:
///     state: the gamestate instance to modify
pub fn score_team1(state: &mut Gamestate) -> usize {


    match state {
        Gamestate::Singles{scoretarget, last_scored_team, quadrant_p1, quadrant_p2, score_team1, score_team2, score_array, ..} => { 
            // bump the counter
            *score_team1 += 1;

            //let advantagecheck: bool = (state.score_team1 - state.score_team2 == 1) && (state.score_team1+1 == state.scoretarget || state.score_team2+1 == state.scoretarget);
            // if both scores are equal and one less than the target (e.g. 20-20 or 15-15) we are in deuce
            // in this case, we also need to increment the score target by 1 (so now a winning score is 22-20 and not 21-20)
            let deucecheck: bool = (*score_team1 - *score_team2 == 0) && (*score_team1+1 == *scoretarget);
            if deucecheck {
                *scoretarget+=1;
                println!("Deuce! Score target is now {}", scoretarget);
            }

            // push the point to the score history
            score_array.push(1);

            // decide the new serving positions
            *last_scored_team = 0;
            *quadrant_p1 = ((*score_team1 % 2) + 2 ) % 3;        // this way even scores map to Q0, odd scores to Q1
            *quadrant_p2 = (*quadrant_p1 + 1 ) % 4;              // Q1 maps to Q2, Q3 maps to Q0

        },

        Gamestate::Doubles{scoretarget, score_team1, score_team2, ..} => {
            // bump the counter
            *score_team1 += 1;

            //let advantagecheck: bool = (state.score_team1 - state.score_team2 == 1) && (state.score_team1+1 == state.scoretarget || state.score_team2+1 == state.scoretarget);
            // if both scores are equal and one less than the target (e.g. 20-20 or 15-15) we are in deuce
            // in this case, we also need to increment the score target by 1 (so now a winning score is 22-20 and not 21-20)
            let deucecheck: bool = (*score_team1 - *score_team2 == 0) && (*score_team1+1 == *scoretarget);
            if deucecheck {
                *scoretarget+=1;
                println!("Deuce! Score target is now {}", scoretarget);
            }


        },
    }

    return 0;
}


/// increments the score of team_2
/// also responsible for changing the score target (or scores) in the event of a decude
/// args:
///     state: the gamestate instance to modify
pub fn score_team2(state: &mut Gamestate) -> usize {

    match state {
        Gamestate::Singles{scoretarget, last_scored_team, quadrant_p1, quadrant_p2, score_team1, score_team2, score_array, ..} => { 
            // bump the counter
            *score_team2 += 1;

            //let advantagecheck: bool = (state.score_team1 - state.score_team2 == 1) && (state.score_team1+1 == state.scoretarget || state.score_team2+1 == state.scoretarget);
            // if both scores are equal and one less than the target (e.g. 20-20 or 15-15) we are in deuce
            // in this case, we also need to increment the score target by 1 (so now a winning score is 22-20 and not 21-20)
            let deucecheck: bool = (*score_team1 - *score_team2 == 0) && (*score_team2+1 == *scoretarget);
            if deucecheck {
                *scoretarget+=1;
                println!("Deuce! Score target is now {}", *scoretarget);
            }

            // push the point to the score history
            score_array.push(2);

            // decide the new serving positions
            *last_scored_team = 1;
            *quadrant_p2 = ((*score_team2 % 2) + 2 ) % 4;        // this way even scores map to Q1, odd scores to Q3
            *quadrant_p1 = (*quadrant_p1 + 3 ) % 4;              // Q0 maps to Q3, Q2 maps to Q1


        },

        Gamestate::Doubles{scoretarget, score_team1, score_team2, ..} => {
            // bump the counter
            *score_team2 += 1;

            //let advantagecheck: bool = (state.score_team1 - state.score_team2 == 1) && (state.score_team1+1 == state.scoretarget || state.score_team2+1 == state.scoretarget);
            // if both scores are equal and one less than the target (e.g. 20-20 or 15-15) we are in deuce
            // in this case, we also need to increment the score target by 1 (so now a winning score is 22-20 and not 21-20)
            let deucecheck: bool = (*score_team1 - *score_team2 == 0) && (*score_team2+1 == *scoretarget);
            if deucecheck {
                *scoretarget+=1;
                println!("Deuce! Score target is now {}", scoretarget);
            }

        },
    }





    return 0;
}