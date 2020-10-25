/* This module is used to create the metadata tags associated with the blogg
 * 
 * 
 * 
 * 
 */


//some standard libraries
use std::env;
use std::fs;
//~ use std::mem;


//the yaml deps
extern crate yaml;
use std::io::BufReader;
use yaml::constructor::*;



fn main() {




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


