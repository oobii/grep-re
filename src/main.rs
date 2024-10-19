#[allow(unused_variables)]

use regex::Regex;
use clap::{App,Arg};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {

    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);

    

    println!("----------------------------------------------------");

    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
         .help("The pattern to search for ")
         .takes_value(true)
         .required(true))
        .get_matches(); 

        let pattern = args.value_of("pattern").unwrap();
        let re = Regex::new(pattern).unwrap(); 

    //let search_term = "bo+k";
    
 //
    let _quote= "\
    Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books. What do we seek through millions of
     pages?";


    for line_ in reader.lines() {
        
        let line = line_.unwrap();
        
        // println!("{} ({} bytes long)",line, line.len());

        match re.find(&line) {
            Some(_) => println!("{}",line),
            None => (),
        }
    }



    let v = vec![1,2,3];
    for n in v.iter().map(|x| x * x) {
        println!("{}",n);
    }


}
