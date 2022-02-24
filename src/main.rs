
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use lazy_static::lazy_static;
lazy_static! {
    static ref RE: Regex = Regex::new(r"[a-z]+[a-z1-9]*>").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    let mut location: Vec<String> = Vec::new();

    let mut dom_tree= Vec::new();
    

    // Our state machine
    let start = XMLParser::new();
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
       
        let mut line = line.unwrap();

        

       processing_router(line, &mut location, &mut dom_tree);
        // else if text.is_match(&line){
        //     x = handle_text(line);
        // }
        
      //  println!("{}", x);
        
        
    }
    
}
fn processing_router(line: String, location: &mut Vec<String>, &mut tree Vec ) {
    lazy_static! {
        static ref RE1: Regex = Regex::new("[^<]").unwrap();
    }
    let trim_line = line.trim();
    if !trim_line.is_empty(){
        println!("{}", trim_line);  
    }
    
    if trim_line.is_empty(){
        return; // no need to continue execution on this line
    }
    else if trim_line.starts_with("</") {
        handle_exit_tag(trim_line.to_string(), location);
    } 
    else if trim_line.starts_with("<"){
        handle_entry_tag(trim_line.to_string(), location);
    } 
    else if RE1.is_match(&trim_line){
        handle_characters(trim_line.to_string(), location);
    }
    
}


fn handle_entry_tag(mut line: String, location: &mut Vec<String>) {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r#"<[a-z]+[a-z1-9]*( {0,}?[a-z]+[a-z1-9] {0,}?= {0,}"[a-z1-9 ]{0,}?")*?>"#).unwrap();
        static ref RE2: Regex = Regex::new(r"[a-z]+[a-z1-9]*").unwrap();
    }
    // Find entry tag in its entirety
    let y = RE1.captures(&line).expect(&line).get(0).unwrap().as_str();
    // Split line based on the entry tag location
    let x: Vec<&str> = line.split(y).collect();
    // Get contents after the entry tag
    let b = x.get(1).unwrap();
    // Push contents of tag to help reference spot in the tree
    location.push(RE2.captures(&line).unwrap().get(0).unwrap().as_str().to_string());
    
    //println!("{}", b);
    // Return contents of the entry tag
    processing_router(b.to_string(), location);

    
}
fn handle_exit_tag(line: String, location: &mut Vec<String>) {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r#"</[a-z]+[a-z1-9]*>"#).unwrap();
        static ref RE2: Regex = Regex::new(r"[a-z]+[a-z1-9]*").unwrap();
    }
    // Find entry tag in its entirety
    let y = RE1.captures(&line).expect(&line).get(0).unwrap().as_str();
    // Split line based on the entry tag location
    let x: Vec<&str> = line.split(y).collect();
    // Get contents after the entry tag
    let b = x.get(1).unwrap();
    // Return contents of the entry tag
    // For each exit tag, there should be an entry tag
    if location.pop().unwrap().as_str().to_string() != RE2.captures(&line).unwrap().get(0).unwrap().as_str().to_string() {
        print!(" NO ")
    }
    processing_router(b.to_string(), location);
}

fn handle_characters(line: String, location: &mut Vec<String>) {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"[^\s&<]+").unwrap();
    }
    // Find characters at beginning of line
    let y = RE1.captures(&line).expect(&line).get(0).unwrap().as_str();
    // Split the line in portions before and after those characters
    let x: Vec<&str> = line.splitn(2,y).collect();
    // Get the characters in the line after the character string
    let a = x.get(1).unwrap();
    //println!("{}", a);
    processing_router(a.to_string(), location);
}

// This is our state machine.
struct XMLParser<S> {
    state: S,
}

/** Start State */
struct XML {

}
// Indicates we found beginning of tag; match tag name, push tag name to array
struct BeginTag {
  
}
// Indicates we found text within an xml file; push text to array.
struct Text {
    
}
// State to be reached upon error or end of file
struct End {
  
}

impl XMLParser<XML> {
    fn new(/* ... */) -> Self {
        // ...
        XMLParser {
            state: XML { /* ... */ },
        }
    }
}


impl From<XMLParser<XML>> for XMLParser<BeginTag> {
    fn from(val: XMLParser<XML>) -> XMLParser<BeginTag> {

        XMLParser {
            // ... attr: val.attr
            state: BeginTag {},
        }
    }
}



impl From<XMLParser<BeginTag>> for XMLParser<XML> {
    fn from(val: XMLParser<BeginTag>) -> XMLParser<XML> {
        XMLParser {
            // ... attr: val.attr
            state: XML { /* ... */ },
        }
    }
}

impl From<XMLParser<XML>> for XMLParser<Text> {
    fn from(val: XMLParser<XML>) -> XMLParser<Text> {

        XMLParser {
            // ... attr: val.attr
            state: Text { /* ... */ },
        }
    }
}

impl From<XMLParser<Text>> for XMLParser<XML> {
    fn from(val: XMLParser<Text>) -> XMLParser<XML> {
        XMLParser {
            // ... attr: val.attr
            state: XML { /* ... */ },
        }
    }
}

impl From<XMLParser<XML>> for XMLParser<End> {
    fn from(val: XMLParser<XML>) -> XMLParser<End> {
        XMLParser {
            // ... attr: val.attr
            state: End { /* ... */ },
        }
    }
}
// fn main() {
//     println!("Hello world");
// }