/**
 * Ax = Bx
 * A, B are m x n matrices
 * elements can be either
 * integers or 'ninf'
 * 'ninf' means -inf
 * 
 * Example Inputs:
 * 
 * Sample1:
 * x0+12, x1+23, x5-1 = x3 + 12, x1 + 15
 * x1 = x3
 * x1 , x2+3 = x4
 * 
 * Sample2:
 * a+2, b+32 = uwu + 1
 * uwu = b
 * 
 * Sample Outputs
 * Sample1:  (values may not be correct)
 * x0 = 3
 * x1 = -5
 * x5 = Ninf
 * 
 * Sample2:
 * a = 3
 * b = 2
 * uwu = 2
 * 
 * Sample3:
 * IMPOSSIBLE
 * tried 1000 iterations
 * 
**/

/*
idea:
compile -> 
    BzProblem, 
    Translator

Translator.translate(x) ->
    [("a", Int(32)), ("uwu", Ninf), ...]

*/

use std::io::BufRead;
use std::{fs, io};
use std::collections::HashMap;
use crate::data::{MatStore, Mat};
use crate::{R, BzProblem};

#[derive(Debug)]
struct Translator {
    map : HashMap<String, usize>,
    counter: usize,
}

impl Translator {
    pub fn empty() -> Self {
        Self { map: HashMap::new(), counter: 0 }
    }

    /// Returns the index of variable. Adds the variable
    /// to translator if not already present.
    pub fn get_index(&mut self, name: &str) -> usize {
        let name = name.trim();
        match self.map.get(name) {
            Some(x) => x.clone(),
            None => {
                self.map.insert(name.to_string(), self.counter);
                let res = self.counter;
                self.counter += 1;
                res
            }
        }
    }
}


fn process(filepath: &str) -> (BzProblem, Translator) {
    let file = fs::File::open(filepath).expect("Error openning file");
    let reader = io::BufReader::new(file);

    let mut translator = Translator::empty();
    fn parse_term(input : &str) -> (&str, i32) {
        match input.split_once('+') {
            Some( (lhs, rhs) ) => {
                (lhs, rhs.trim().parse().unwrap())
            },
            None => match input.split_once('-') {
                Some( (lhs, rhs) ) => (lhs, -rhs.trim().parse::<i32>().unwrap()),
                None => (input, 0),
            }
        }
    }

    let mut Astore : MatStore = Vec::new();
    let mut Bstore : MatStore = Vec::new();

    for line in reader.lines() {
        
        let line = line.unwrap();
        let (lhs, rhs) = line.split_once('=').unwrap();
        
        Astore.push(
            lhs.split(',').map(parse_term).map(|(var, num)| {
                (translator.get_index(var), R::Int(num))
            }).collect()
        );

        Bstore.push(
            rhs.split(',').map(parse_term).map(|(var, num)| {
                (translator.get_index(var), R::Int(num))
            }).collect()
        );
    }

    let nrows = Astore.len();
    let ncols = translator.counter;
    let A = Mat { store: Astore, nrows, ncols, emp_val: R::Ninf };
    let B = Mat { store: Bstore, nrows, ncols, emp_val: R::Ninf };
    let bzproblem = BzProblem { A, B, x_init: None };

    return (bzproblem, translator);
}

mod tests {
    use crate::solve;

    use super::*;
    
    #[test]
    fn process_test() {
        let filename = "input/test1.txt";
        let bz = process(filename).0;
        let res = solve(bz).unwrap();
        println!("solution found = {:?}", res)
    }
}