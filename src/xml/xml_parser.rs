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
        current_path: Vec::with_capacity(1000)
    };
    // Iterate through each line of the XML file
    for (index, line) in reader.lines().enumerate() {
        // Get the result of each line in the form of a string.  
        let line = line.unwrap();
        // Once the line is known, pass the line to the state machine router, along with the 
        // tree
        processing_router(line.as_str(), &mut tree);


    }
    return tree;
    
}
/// This function serves as the main router for the state machine.  
/// There are four scenarios: 
///     *The method encounters a starting tag -> hanle the starting tag
///     *The method encounters an ending tag -> handle the ending tag
///     *The method encounters regular text -> handle the inner text
///     *The method encounters a empty line -> return to the main function
fn processing_router(line: &str, tree: &mut XMLTree) {

    let trim_line = line.trim();

    if trim_line.is_empty() {
        return; // no need to continue execution on this line
    } else if trim_line.starts_with("</") {
        handle_exit_tag(trim_line, tree); // handle the exit tag
    } else if trim_line.starts_with('<') {
        handle_entry_tag(trim_line, tree); // handle the entry tag
    } else {
        handle_characters(trim_line, tree); // handle the text
    }
}
/// Handles any entry tags that the machine comes across.  The method matches the full opening tag 
/// with regex, and then looks for just the '<' and the tag name.  Upon finding this, the method strips the
/// tag name off of the line, and uses the remaining line to get any properties.  
/// The '<' and the tag name is then used to create an XMLElement, which is then added to the XMLTree.
/// The newly created XMLElement is added to the XMLTree's path for easy tracking.  After this, the line passed
/// in is stripped of the information already processed, and the modified line is sent to the processor.       
fn handle_entry_tag(line: &str, tree: &mut XMLTree)  {
    // Find entry tag in its entirety   
    let full_tag_itr: Vec<&str> = line.split_inclusive('>').collect();
    let full_tag_ptr = full_tag_itr[0];
    // Get any potential new properties
    let potential_props_itr: Vec<&str> = full_tag_ptr.split_whitespace().collect();
    let potential_props_prefix_1;
    // Create the vector that will hold the properties
    let mut prop_map = Vec::new();
    // If there are no properties, trim the end of the XML tag.  
    if potential_props_itr.len() == 1 {
        potential_props_prefix_1 = potential_props_itr[0].trim_end_matches('>');
    } 
    // Otherwise, process the properties!
    else {
       // potential_props_prefix_1 = potential_props_itr.get(0).unwrap();
        potential_props_prefix_1 = potential_props_itr[0];
        // Create a string without the tag name, leaving property information behind.
        let potential_props = full_tag_ptr.strip_prefix(potential_props_prefix_1).unwrap().to_string();
        // Trim the end of the XML tag to make sure that any properties are free of whitespace and the '>'
        let potential_props_trim = potential_props.trim_end_matches(|c| c == ' ' || c == '>');
        // Collect the properties and process them, adding them to what will become the properties field of the node
        let props: Vec<&str> = potential_props_trim.split_ascii_whitespace().collect();
        for prop in props {
            let tuple = prop.split_once("=").unwrap();
            prop_map.push((tuple.0.to_string(), tuple.1.to_string()));
        }
    }
    
    // Get tag name
    let tag_name = potential_props_prefix_1.strip_prefix('<').unwrap();
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
    let new_line = line.strip_prefix(full_tag_ptr).unwrap();
    // Return contents of line to continue processing
    processing_router(new_line, tree);
}
/// Handles any closing tags that the machine comes across.  The method matches the full closing tag 
/// with regex, and then looks for just the tag name. After this, the line passed
/// in is stripped of the information already processed, and the modified line is sent to the processor.
/// Finally, XMLTree's path disposes of the final XMLElement, since that is no longer the current parent
fn handle_exit_tag(line: &str, tree: &mut XMLTree) {

    // Find exit tag in its entirety
    let exit_tag_itr: Vec<&str> = line.split_inclusive('>').collect();
    let exit_tag = exit_tag_itr[0];
    // Get contents after the exit tag
    let ret_line = line.strip_prefix(exit_tag).expect(line);
    // Pop the current parent from the path
    tree.current_path.pop();
    // Return contents of the exit tag
    processing_router(ret_line, tree);
    
}
/// Function for processing characters.  The characters are matched with a 
/// regular expression.  These characters are then added to the parent's inner
/// text property.  The given line is then trimmed of the characters, and returned
/// to the processing router.  
fn handle_characters(line: &str, tree: &mut XMLTree) {
   
    // Find characters at beginning of line

    let text_iter: Vec<&str> = line.splitn(2, '<').collect();
    let text_wrap = text_iter[0];

    // Split the line in portions before and after those characters
    let lin_split: Vec<&str> = line.splitn(2, text_wrap).collect();
    // Get the characters in the line after the character string
    let ret_line = lin_split[1];
    // Get parent of the text
    let root_cell = tree.current_path.last().unwrap();
    // Push text to the parent
    RefCell::try_borrow_mut(root_cell).unwrap().add_text(text_wrap.to_string());
    // Return the trimmed line to the processing router
    processing_router(ret_line, tree);
}

