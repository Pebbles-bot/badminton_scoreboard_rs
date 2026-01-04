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

//////////////////////////////////////////////////////////////
//
pub fn factorial(n: i32) -> i128 {

    let mut answer: i128 = 1;
    
    for s in 1..n+1 {
        answer = answer * s as i128;
    }

    return answer;

}


fn partial_factorial(n: i32, k: i32) -> i128 {

    let mut answer: i128 = 1;
    
    for s in k+1..n+1 {
        answer = answer * s as i128;
    }

    return answer;

}


pub fn choose(n: i32, k: i32) -> i128 {

    if k > (n-k) {
        return partial_factorial(n, k) / factorial(n-k);
    }  else { 
        return partial_factorial(n, n-k) / factorial(k);
    }

}

