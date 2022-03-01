use xml::xml_element::XMLElement;
use xml::xml_tree::XMLTree;
use xml::xml_parser;
use std::cell::{RefCell, Ref};
mod xml;
use std::env;

fn main() {
    println!("Program started");
    
    let args: Vec<String> = env::args().collect();
    let tree = xml_parser::parse_xml_to_tree(&args[1]);
    println!("Program started");
    let x = get_relevant_orders(&tree);
    let z = tree.get_elements_at_path("/root/order/amount".to_string());
    let y = 1;
    println!("Program completed");
    
}
/// Gets all the orders within the XML file that have amounts with inner text that are greater than 100.
/// Returns the vector.  
fn get_relevant_orders(tree: &XMLTree) -> Vec<(String, String)>{
    // Get the root of the given tree 
    let root = &tree.get_root();
    // Get the children of the root, which represent the orders in the XML file
    let orders = root.get_children();
    // Create the vector that will be returned
    let mut relevant_orders:Vec<(String, String)> = Vec::new();
    // Iterate through the orders
    for order in orders {
        // Get the children of the current order
        let children = order.get_children();
        // Iterate through the children of the current order
        for child in children {
            // Get the inner text of the amount, as a number
            let order_value = &child.get_inner_text().parse::<i32>().unwrap();
            // Determine if the order is relevant
            if order_value > &100i32  {
                let id = order.get_prop("id".to_string()).unwrap();
                let amount = child.get_inner_text().to_string();
                relevant_orders.push((id.1, amount));
            }
        }
    }
    return relevant_orders;
}

