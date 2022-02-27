use lazy_static::lazy_static;
use regex::Regex;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod xml_element;
mod xml_tree;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    // let mut path: Vec<String> = Vec::new();
    // let mut path_map: HashMap<Vec<String>, Vec<XMLElement>> = HashMap::new();
    // let mut pos: Vec<Box<XMLElement>> = Vec::new();
    let mut tree = XMLTree {
        root: None,
        current_path: Vec::new()
    };
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let mut line = line.unwrap();

        processing_router(line, &mut tree);
        // else if text.is_match(&line){
        //     x = handle_text(line);
        // }

    }
    print!("{}", tree);
    
}
fn processing_router(line: String, tree: &mut XMLTree) {
    lazy_static! {
        static ref OPEN_BRACKET: Regex = Regex::new("[^<]").unwrap();
    }
    let trim_line = line.trim();

    if trim_line.is_empty() {
        return; // no need to continue execution on this line
    } else if trim_line.starts_with("</") {
        handle_exit_tag(trim_line.to_string(), tree);
    } else if trim_line.starts_with("<") {
        handle_entry_tag(trim_line.to_string(), tree);
    } else if OPEN_BRACKET.is_match(&trim_line) {
        handle_characters(trim_line.to_string(), tree);
    }
}

fn handle_entry_tag(line: String, tree: &mut XMLTree)  {
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
    
    let element = XMLElement::new(loc.to_string(), prop_map);
   
    if tree.root.is_none() {
        let x =  tree.root.insert(Rc::new(RefCell::new(element)));
        tree.current_path.push(Rc::clone(x));
    } else {
        let parent = tree.current_path.last_mut().unwrap();
        // Create element wrapper
        let elem_wrapper = Rc::new(RefCell::new(element));
        let elem_wrapper_2 = Rc::clone(&elem_wrapper);

        RefCell::try_borrow_mut(parent).unwrap().add_child(elem_wrapper);
        // Push clone of that wrapper to the current path
        tree.current_path.push(elem_wrapper_2);
    }
    // Return contents of the entry tag
    processing_router(new_line.to_string(), tree);
}
fn handle_exit_tag(line: String, tree: &mut XMLTree) {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r#"</[a-z]+[a-z0-9]*>"#).unwrap();
        static ref RE2: Regex = Regex::new(r"[a-z]+[a-z0-9]*").unwrap();
    }

    // Find exit tag in its entirety
    let y = RE1.captures(&line).expect(&line).get(0).unwrap().as_str();
    // Get contents after the exit tag
    let b = line.strip_prefix(y).expect(&line);
    // Return contents of the exit tag
    processing_router(b.to_string(), tree);
    tree.current_path.pop();
}

fn handle_characters(line: String, tree: &mut XMLTree) {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"[^\s&<]+").unwrap();
    }
    // Find characters at beginning of line
    let y = RE1.captures(&line).expect(&line).get(0).unwrap().as_str();
    // Split the line in portions before and after those characters
    let x: Vec<&str> = line.splitn(2, y).collect();
    // Get the characters in the line after the character string
    let a = x.get(1).unwrap();

    let root_cell = tree.current_path.last().unwrap();
    RefCell::try_borrow_mut(root_cell).unwrap().add_text(y.to_string());

    processing_router(a.to_string(), tree);
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

