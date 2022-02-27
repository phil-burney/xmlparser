use lazy_static::lazy_static;
use regex::Regex;
use std::collections::hash_map::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Clone, Debug)]
struct XMLElement {
    tag: String,
    props: HashMap<String, String>,
    children: Vec<XMLElement>,
    inner_text: String,
}
impl fmt::Display for XMLElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut map = String::new();
        for key in &self.props {
            map.push_str(&key.0);
            map.push('=');
            map.push_str(&key.1);
            map.push_str("; ");
        }
        let mut children = String::new();
        for child in &self.children {
            children.push_str(&child.to_string());
        }
        write!(f, "\n\ttag: {} \n\tprops: {} \n\tchildren: {}", self.tag, map, children)
    }
}

impl XMLElement {
    fn new(tag: String, props: HashMap<String, String>) -> XMLElement {
        return XMLElement {
            tag: String::from(tag),
            props: props,
            children: Vec::new(),
            inner_text: String::new(),
        };
    }
    fn add_child(&mut self, child: XMLElement) {
        self.children.push(child);
    }
    fn add_property(&mut self, key: String, value: String) {
        self.props.insert(key, value);
    }
    fn add_text(&mut self, text: String) {
        self.inner_text = text;
    }
}
struct XMLTree {
    root: Option<XMLElement>,
    current_path: Vec<*mut XMLElement>
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    let mut path: Vec<String> = Vec::new();
    let mut path_map: HashMap<Vec<String>, Vec<XMLElement>> = HashMap::new();
    let mut pos: Vec<Box<XMLElement>> = Vec::new();
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let mut line = line.unwrap();

        processing_router(line, &mut path, &mut path_map);
        // else if text.is_match(&line){
        //     x = handle_text(line);
        // }

        //  println!("{}", x);
    }
    let mut map = String::new();
    map.push_str("------------- \n");
    map.push_str("RESULTS: \n");
    for key in path_map {
        map.push_str("path: ");
        //map.push_str(&key.);
        map.push_str("\nresidents:");
        for ele in &key.1 {
            map.push_str(&ele.to_string());
            map.push(',');
        }
        map.push('\n');
    }
    println!("{}", map);
}
fn processing_router(line: String, path: &mut Vec<String>, map: &mut HashMap<Vec<String>, Vec<XMLElement>>) {
    lazy_static! {
        static ref OPEN_BRACKET: Regex = Regex::new("[^<]").unwrap();
    }
    let trim_line = line.trim();
    // if !trim_line.is_empty(){
    //     println!("{}", trim_line);
    // }

    if trim_line.is_empty() {
        return; // no need to continue execution on this line
    } else if trim_line.starts_with("</") {
        handle_exit_tag(trim_line.to_string(), path, map);
    } else if trim_line.starts_with("<") {
        handle_entry_tag(trim_line.to_string(), path, map);
    } else if OPEN_BRACKET.is_match(&trim_line) {
        handle_characters(trim_line.to_string(), path, map);
    }
}

fn handle_entry_tag(line: String, path: &mut Vec<String>, map: &mut HashMap<Vec<String>, Vec<XMLElement>>)  {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r#"<[a-z]+[a-z0-9]*( *[a-z]+[a-z0-9 ]*="[a-z0-9 ]*")*>"#).unwrap();
        static ref RE2: Regex = Regex::new(r"<[a-z]+[a-z0-9]*").unwrap();
    }
    // Find entry tag in its entirety   
    let full_tag_ptr = RE1.captures(&line).expect(&line).get(0).unwrap().as_str();
    // Get the tag name from the entry tag
    let potential_props_prefix = RE2.captures(&full_tag_ptr).expect(&full_tag_ptr).get(0).unwrap().as_str();
    // Strip the tag name
    let potential_props = full_tag_ptr.to_string().strip_prefix(potential_props_prefix).unwrap().to_string();
    // Create a new HashMap to store properties
    let mut prop_map = HashMap::new();
    // Handle the properties
    handle_properties(potential_props, &mut prop_map);
    let new_line = line.strip_prefix(full_tag_ptr).expect(&line.to_string());
    // Get tag name
    let loc = potential_props_prefix.strip_prefix('<').unwrap();
    // Push location to current path
    path.push(loc.to_string());
    //
    let element = XMLElement::new(loc.to_string(), prop_map);
    let element2 = element.clone();

    let mut par_path = path.clone();
    par_path.pop();

   

    let x = path.clone();

    if map.get(path).is_none() {
        map.insert(x, Vec::new());
    }
    let vec = map.get_mut(path).unwrap();

    

    vec.push(element2);

    //   println!("Element found: {} \nIts path is: {}", vec.last().unwrap(), translate_path(path).as_str());

    // Return contents of the entry tag
    processing_router(new_line.to_string(), path, map);
}
fn handle_exit_tag(line: String, path: &mut Vec<String>, map: &mut HashMap<Vec<String>, Vec<XMLElement>>) {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r#"</[a-z]+[a-z0-9]*>"#).unwrap();
        static ref RE2: Regex = Regex::new(r"[a-z]+[a-z0-9]*").unwrap();
    }

    // Find entry tag in its entirety
    let y = RE1.captures(&line).expect(&line).get(0).unwrap().as_str();
    // Get contents after the entry tag
    let b = line.strip_prefix(y).expect(&line);
    // Return contents of the entry tag
    path.pop();
    processing_router(b.to_string(), path, map);
}

fn handle_characters(line: String, path: &mut Vec<String>, map: &mut HashMap<Vec<String>, Vec<XMLElement>>) {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"[^\s&<]+").unwrap();
    }
    // Find characters at beginning of line
    let y = RE1.captures(&line).expect(&line).get(0).unwrap().as_str();
    // Split the line in portions before and after those characters
    let x: Vec<&str> = line.splitn(2, y).collect();
    // Get the characters in the line after the character string
    let a = x.get(1).unwrap();
    //println!("{}", a);
    processing_router(a.to_string(), path, map);
}

fn handle_properties(potential_props: String, prop_map: &mut HashMap<String, String>) {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r#"[a-z]+[a-z0-9] {0,}?= *"[a-z0-9 ]*""#).unwrap();
        static ref RE2: Regex = Regex::new(r"[a-z]+[a-z0-9]*").unwrap();
    }
    let potential_props_trim = potential_props.trim();
    if potential_props_trim.eq(">") {
        return;
    }
    let prop = RE1.captures(&potential_props_trim).unwrap().get(0).unwrap().as_str();
    let tuple: Vec<&str> = prop.splitn(2, "=").collect();

    prop_map.insert(tuple.get(0).unwrap().to_string(), tuple.get(1).unwrap().to_string());

    let props = potential_props_trim.strip_prefix(prop).unwrap();

    handle_properties(props.to_string(), prop_map);
}

fn translate_path(path: &mut Vec<String>) -> String {
    let mut s = String::new();
    for location in path {
        s.push_str(location);
        s.push('/');
    }
    s.truncate(s.len() - 1);
    return s;
}
