use lazy_static::lazy_static;
use regex::Regex;
use std::cell::{RefCell};
use std::rc::Rc;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::xml::xml_element::XMLElement;
use crate::xml::xml_tree::XMLTree;

/// This function takes a string as an argument, and returns 
/// an XMLTree struct.
pub fn parse_xml_to_tree(file_path: &str) -> XMLTree {
    // Open the file with the given parameter. If it does not exist,
    // the function panics.
    let file = File::open(file_path).unwrap();
    // Open a file reader.  
    let reader = BufReader::new(file);
    // Create the XML tree that will be returned to the caller.
    let mut tree = XMLTree {
        root: None,
        current_path: Vec::new()
    };
    // Iterate through each line of the XML file
    for (index, line) in reader.lines().enumerate() {
        // Get the result of each line in the form of a string.  
        let line = line.unwrap();
        // Once the line is known, pass the line to the state machine router, along with the 
        // tree
        processing_router(line, &mut tree);


    }
    return tree;
    
}
/// This function serves as the main router for the state machine.  
/// There are four scenarios: 
///     *The method encounters a starting tag -> hanle the starting tag
///     *The method encounters an ending tag -> handle the ending tag
///     *The method encounters regular text -> handle the inner text
///     *The method encounters a empty line -> return to the main function
fn processing_router(line: String, tree: &mut XMLTree) {
    // Compile regex only once
    lazy_static! {
        static ref TEXT: Regex = Regex::new("[^<]").unwrap();
    }
    let trim_line = line.trim();

    if trim_line.is_empty() {
        return; // no need to continue execution on this line
    } else if trim_line.starts_with("</") {
        handle_exit_tag(trim_line, tree); // handle the exit tag
    } else if trim_line.starts_with('<') {
        handle_entry_tag(trim_line, tree); // handle the entry tag
    } else if TEXT.is_match(trim_line) {
        handle_characters(trim_line, tree); // handle the text
    }
}
/// Handles any entry tags that the machine comes across.  The method matches the full opening tag 
/// with regex, and then looks for just the '<' and the tag name.  Upon finding this, the method strips the
/// tag name off of the line, and sends the remainining line to another method to get any properties.  
/// The '<' and the tag name is then used to create an XMLElement, which is then added to the XMLTree.
/// The newly created XMLElement is added to the XMLTree's path for easy tracking.  After this, the line passed
/// in is stripped of the information already processed, and the modified line is sent to the processor.       
fn handle_entry_tag(line: &str, tree: &mut XMLTree)  {
    // Compile regex only once
    lazy_static! {
        static ref RE1: Regex = Regex::new(r#"<[a-z]+[a-z0-9]*( *[a-z]+[a-z0-9 ]*="[a-z0-9 ]*")*>"#).unwrap();
        static ref RE2: Regex = Regex::new(r"<[a-z]+[a-z0-9]*").unwrap();
    }
    // Find entry tag in its entirety   
    let full_tag_ptr = RE1.captures(line).expect(line).get(0).unwrap().as_str();
    // Get just the tag name from the entry tag
    let potential_props_prefix = RE2.captures(full_tag_ptr).expect(full_tag_ptr).get(0).unwrap().as_str();
    // Create a string without the tag name, leaving property information behind.  
    let potential_props = full_tag_ptr.to_string().strip_prefix(potential_props_prefix).unwrap().to_string();
    // Create a new HashMap to store properties
    let mut prop_map = Vec::new();
    // Handle the properties
    handle_properties(potential_props, &mut prop_map);
    
    // Get tag name
    let tag_name = potential_props_prefix.strip_prefix('<').unwrap();
    // Create new XMLElement with the given information.  
    let element = XMLElement::new(tag_name.to_string(), prop_map);
    // If there is no root to the tree...
    if tree.root.is_none() {
        // Create one and insert it in the tree!
        let x =  tree.root.insert(Rc::new(RefCell::new(element)));
        // Push the newly created Element to the path for tracking
        tree.current_path.push(Rc::clone(x));
    // Otherwise append the newly created element to a parent
    } else {
        // Get the current parent
        let parent = tree.current_path.last_mut().unwrap();
        // Create one owner for the new mutable XMLElement
        let elem_wrapper = Rc::new(RefCell::new(element));
        // Create a second owner for the new XMLElement, which points to the same
        // section of memory
        let elem_wrapper_2 = Rc::clone(&elem_wrapper);
        // Transfer ownership of the pointer to the new XMLElement to the parent element
        RefCell::try_borrow_mut(parent).unwrap().add_child(elem_wrapper);
        // Transfer ownership of the other pointer to the list keeping track of the current path
        tree.current_path.push(elem_wrapper_2);
    }
    // Strip the current line of the information already processed
    let new_line = line.strip_prefix(full_tag_ptr).expect(&line.to_string());
    // Return contents of line to continue processing
    processing_router(new_line.to_string(), tree);
}
/// Handles any closing tags that the machine comes across.  The method matches the full closing tag 
/// with regex, and then looks for just the tag name. After this, the line passed
/// in is stripped of the information already processed, and the modified line is sent to the processor.
/// Finally, XMLTree's path disposes of the final XMLElement, since that is no longer the current parent
fn handle_exit_tag(line: &str, tree: &mut XMLTree) {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r#"</[a-z]+[a-z0-9]*>"#).unwrap();
        static ref RE2: Regex = Regex::new(r"[a-z]+[a-z0-9]*").unwrap();
    }

    // Find exit tag in its entirety
    let exit_tag = RE1.captures(line).expect(line).get(0).unwrap().as_str();
    // Get contents after the exit tag
    let ret_line = line.strip_prefix(exit_tag).expect(line);
    // Pop the current parent from the path
    tree.current_path.pop();
    // Return contents of the exit tag
    processing_router(ret_line.to_string(), tree);
    
}
/// Function for processing characters.  The characters are matched with a 
/// regular expression.  These characters are then added to the parent's inner
/// text property.  The given line is then trimmed of the characters, and returned
/// to the processing router.  
fn handle_characters(line: &str, tree: &mut XMLTree) {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"[^\s&<]+").unwrap();
    }
    // Find characters at beginning of line
    let y = RE1.captures(line).expect(line).get(0).unwrap().as_str();
    // Split the line in portions before and after those characters
    let lin_split: Vec<&str> = line.splitn(2, y).collect();
    // Get the characters in the line after the character string
    let ret_line = lin_split.get(1).unwrap();
    // Get parent of the text
    let root_cell = tree.current_path.last().unwrap();
    // Push text to the parent
    RefCell::try_borrow_mut(root_cell).unwrap().add_text(y.to_string());
    // Return the trimmed line to the processing router
    processing_router(ret_line.to_string(), tree);
}
/// Handle the properties within the XML opening tag. Properties are handled recursively.
/// One property is processed at a time.  First a regex is matched, and that property
/// is fed into a hashmap that the parent has.  The string is then trimmed and the process repeats
/// until an end tag is found.    
fn handle_properties(potential_props: String, prop_map: &mut Vec<(String, String)>) {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r#"[a-z]+[a-z0-9] {0,}?= *"[a-z0-9 ]*""#).unwrap();
        static ref RE2: Regex = Regex::new(r"[a-z]+[a-z0-9]*").unwrap();
    }
    // Trim whitespace from string
    let potential_props_trim = potential_props.trim();
    // End condition for recursion
    if potential_props_trim.eq(">") {
        return;
    }
    // Match potential props
    let prop = RE1.captures(potential_props_trim).unwrap().get(0).unwrap().as_str();
    // Split string around the "=" sign; only two strings can be made
    let tuple: Vec<&str> = prop.splitn(2, '=').collect();
    // Insert key-value pair into the hashmap of the parent
    prop_map.push((tuple.get(0).unwrap().to_string(), tuple.get(1).unwrap().to_string()));
    // Strip the returned string of the information that has already been processed
    let props = potential_props_trim.strip_prefix(prop).unwrap();
    // Process any new properties, if any
    handle_properties(props.to_string(), prop_map);
}

