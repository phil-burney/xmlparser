use crate::xml::xml_element::XMLElement;
use std::cell::{Ref, RefCell};
use std::fmt;
use std::rc::Rc;

/// XMLTree is an abstraction for an n-ary tree.  This n-ary tree consists
/// of nodes, and each of these nodes represents an XML Element.
pub struct XMLTree {
    /// Represents the root of the XML tree.  Only visible within the xml crate.
    /// If no root exists, then None is returned.
    pub(in crate::xml) root: Option<Rc<RefCell<XMLElement>>>,
    /// The current location within the tree.  Used to keep track of the
    /// current path when building the tree.  Only visible within the xml crate.
    pub(in crate::xml) current_path: Vec<Rc<RefCell<XMLElement>>>,
}

impl XMLTree {
    /// Returns a ref of the root of the XML tree.  This ref can be used
    /// as an XMLElement would be.  That is, the struct returned can call
    /// all of the methods that an XML element can.       
    pub fn get_root(&self) -> Ref<'_, XMLElement> {
        let r = self.root.as_deref().unwrap();
        let root = RefCell::try_borrow(r).unwrap();
        return root;
    }
    /// Takes a string as a parameter and returns an option of a vector of XMLElements.
    /// These returned XMLElements are XMLElement(s) found on the path
    /// For example, "/root/order" will return a vector of <order> XMLElements.  
    /// The XMLElements are cloned from the tree, as there is no need to reference the 
    /// tree here.  All paths should start with a "/", and there should be no end "/".  
    pub fn get_elements_at_path(&self, path: String ) -> Option<Vec<XMLElement>>{
        // Make the vector that will be returned from the
        let mut ret_vec = Vec::new();
        // Split the path at the "/" for parsing
        let path_vec: Vec<&str> = path.split('/').collect();
        // Get the root of the tree
        let root_node = self.get_root();
        // Get the tag name of the root
        let root_tag = root_node.get_tag();
        // Iterate through the tree recursively
        self.get_elements_at_path_helper(root_node, &mut ret_vec, path_vec, 1);
        // If the vector to be returned is empty, return none.
        if ret_vec.is_empty() {
            return None;
        }
        // Otherwise, return the vector
        return Some(ret_vec);
    }
    /// Helper function for the get_elements_at_path function. Recursively iterates through the tree.  
    fn get_elements_at_path_helper(&self, node: Ref<'_, XMLElement>, ret_children: &mut Vec<XMLElement>, path: Vec<&str>, tree_depth: usize) {
        // Get the tag of the current node
        let tag = node.get_tag();
        // See if the location in the path exists, and if the tag equals the location.
        if path.get(tree_depth).is_some() && tag.ne(path.get(tree_depth).unwrap()) {
            return;
        }
       // Iterate through the children of each node, picking out the nodes to iterate through next
        for child in node.get_children() {
            let x = path.clone();
            
            if x.get(tree_depth).is_some() && tag.eq(x.get(tree_depth).unwrap()) {
                self.get_elements_at_path_helper(child, ret_children, x, tree_depth + 1)
            }
            
        }
        // If the tag of the current node equal the final location in the path, push that node to the vector to be returned
        if tag.eq(path.last().unwrap()) {
            ret_children.push(node.clone());
        }
       
    }
}
impl fmt::Display for XMLTree {
    /// Formats the XML Tree for printing
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let root_cell = self.root.as_deref().unwrap();
        let root = RefCell::try_borrow_mut(root_cell).unwrap();

        return write!(f, "{}", root);
    }
}
