use xml::xml_tree::XMLTree;
use xml::xml_parser;
mod xml;
use std::env;
use std::time::SystemTime;

fn main() {
    println!("Program started");
    // Collect the arguments from the console
    let args: Vec<String> = env::args().collect();
    // Get an XML tree to work with 
    let now = SystemTime::now();
    let tree = xml_parser::parse_xml_to_tree(&args[1]);
    match now.elapsed() {
        Ok(elapsed) => {
            println!("Tree Built in: {}ms", elapsed.as_millis());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {:?}", e);
        }
    }
    // Functions below do the same thing

    // This function uses the XMLTree to achieve the function
    let tree_info = get_relevant_orders_by_tree(&tree);
   // print_results(tree_info);
    // This function uses the paths functionality to achieve the functionality
    let path_info = get_relevant_orders_by_path(&tree, "/root/order".to_string());
    println!("\n");
   // print_results(path_info);
    
    println!("Program completed");
    
}
fn print_results(vec: Vec<(String, String)>) {
    println!("id    |  amount ");
    println!("----------------");
    for vecs in vec {
        
        println!("{:<6}|  {:<5}", vecs.0, vecs.1)
    }
}
/// Gets all the orders within the XML file that have amounts with inner text that are greater than 100.
/// Returns the vector.  To do so, the user has to access the XML tree.    
fn get_relevant_orders_by_tree(tree: &XMLTree) -> Vec<(String, String)>{
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

fn get_relevant_orders_by_path(tree: &XMLTree, path: String) -> Vec<(String, String)>{
    // Get the root of the given tree 
    let orders = tree.get_elements_at_path(path).unwrap();
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

