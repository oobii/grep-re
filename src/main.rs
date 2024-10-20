#[allow(unused_variables)]

use regex::Regex;
use clap::{App,Arg};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {

    println!("----------------------------------------------------");

    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
         .help("The pattern to search for ")
         .takes_value(true)
         .required(true))
        .arg(Arg::with_name("input")
         .help("File to read from ")
         .takes_value(true)
         .required(true))
        .get_matches(); 

        let pattern = args.value_of("pattern").unwrap();
        let re = Regex::new(pattern).unwrap(); 

        let input = args.value_of("input").unwrap_or("-");
    //let search_term = "bo+k";
    
 
        if input == "-"  {
            let stdin = io::stdin();
            let reader1 = stdin.lock();
            for line_ in reader1.lines() {
                let line = line_.unwrap();
                match re.find(&line) {
                    Some(_) => println!("{}",line),
                    None => (),
                }
            }
        } else {
            let f = File::open(input).unwrap();
            let reader2 = BufReader::new(f);
            
            for line_ in reader2.lines() {
                let line = line_.unwrap();
                match re.find(&line) {
                    Some(_) => println!("{}",line),
                    None => (),
                }
            }
    
        }
        


    



    let v = vec![1,2,3];
    for n in v.iter().map(|x| x * x) {
        println!("{}",n);
    }


}
