/* This module is used to create the metadata tags associated with the blogg
 * 
 * 
 * 
 * 
 */

//I use camel case, so turn off snake_case compiler messages
#![allow(non_snake_case)]

//some standard libraries
use std::env;
use std::fs;
use std::mem;


//the yaml deps
extern crate yaml;
use std::io::BufReader;
use yaml::constructor::*;


/*
 * //these below are ripped from jekyll, trying to follow these attributes 
  compose               
  docs                  
  import                
  build, b              Build your site
  clean                 Clean the site (removes site output and metadata file) without building.
  doctor, hyde          Search site and print specific deprecation warnings
  help                  Show the help message, optionally for a given subcommand.
  new                   Creates a new Jekyll site scaffold in PATH
  new-theme             Creates a new Jekyll theme scaffold
  serve, server, s      Serve your site locally
 
 * 
 * 
 * 
 */


fn main() {
// taken from manual => https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
let commandLineArgs: Vec<String> = env::args().collect();
    println!("{:?}", commandLineArgs);




let mut testVec : Vec<String> = Vec::new(); // testing out the vector things

testVec.push("<meta name='keywords' content=''".to_string()); 
testVec.push("/>".to_string()); 

for x in testVec.iter() {
        print!("{}", x);
    }


println!(""); //extra whitespace
//  
//
//
    //~ println!("Hello, world!");
//
//
//

}


