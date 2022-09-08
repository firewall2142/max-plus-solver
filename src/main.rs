use std::env;
use mpsolverLib::{parser::parse_file, solve};

fn main () {
    let args : Vec<String> = env::args().collect();
    let filename = match args.get(1) {
        Some(x) => x,
        None => panic!("filename not given"),
    };
    
    let (problem, translator) = parse_file(filename);
    match solve(problem) {
        None => eprintln!("No solution found!"),
        Some(x) => {
            for (var, val) in translator.solution_kvp(&x) {
                println!("{} = {:?}", var, val)
            }
        }
    }

}