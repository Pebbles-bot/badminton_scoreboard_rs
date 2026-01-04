// ///////////////////////////////////////////////////////////
/* Copyright (C) 2024 Faseeh Irfan - All Rights Reserved
 * You may use, distribute and modify this code under the
 * terms of the CC-BY Creative commons license.
 *
 * You should have received a copy of the CC- license with
 * this file. If not, look it up or something idk.
 */
// ///////////////////////////////////////////////////////////



/// checks if the game is over or not
/// logically: a row/col/diag line is represented with a number 1 * 2^x * 3^o, where x is the number of Xes in the line
/// and o is the number of Os in the line. This way Xes and Os can intermix but the game only ends if one of the lines is 
/// 8 or 27
/// then if the product of all the numbers in total is 2^5 * 3^4 =  2592 or 2^4 * 3^5 = 3888, it means every cell is filled and the game must be over
pub fn check_winning(state: &mut Gamestate) -> usize {

    return 0;
}


/// checks if the game is over or not
/// logically: a row/col/diag line is represented with a number 1 * 2^x * 3^o, where x is the number of Xes in the line
/// and o is the number of Os in the line. This way Xes and Os can intermix but the game only ends if one of the lines is 
/// 8 or 27
/// then if the product of all the numbers in total is 2^5 * 3^4 =  2592 or 2^4 * 3^5 = 3888, it means every cell is filled and the game must be over
pub fn check_serves(state: &mut Gamestate) -> usize {

    return 0;
}



/// simulate the act of placing an X in a board
/// this corresponds to multiplying the appropriate board point by 3
/// also fails to act if the value of the cell is >1, meaning its a filled cell
/// args:
///     row: the index of the row, from 0-2
///     col: the index of the column, from 0-2
///     state: the gamestate instance to modify
pub fn score_p1(row: usize, col: usize, state: &mut Gamestate) -> usize {

    return 0;
}

