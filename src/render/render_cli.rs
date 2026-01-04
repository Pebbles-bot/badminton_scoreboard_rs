// ///////////////////////////////////////////////////////////
/* Copyright (C) 2024 Faseeh Irfan - All Rights Reserved
 * You may use, distribute and modify this code under the
 * terms of the CC-BY Creative commons license.
 *
 * You should have received a copy of the CC- license with
 * this file. If not, look it up or something idk.
 */
// ///////////////////////////////////////////////////////////

use crate::engine::state::Gamestate;


pub fn render_gamestate(state: &Gamestate) {


    match state {
        Gamestate::Singles{score_team1, score_team2, score_array, last_scored_team, quadrant_p1, quadrant_p2, ..} => { 
            println!("P1 {} | P2 {}", score_team1, score_team2);
            dbg!(score_array);
            println!("P{} to serve. Player 1 in Quadrant {}, Player 2 in Quadrant {}", last_scored_team, quadrant_p1, quadrant_p2); 
            
        },

        Gamestate::Doubles{score_team1, score_team2, score_array, ..} => {
            println!("P1 {} | P2 {}", score_team1, score_team2);
            dbg!(score_array);
            println!("PN to serve"); 
        },
    }


}