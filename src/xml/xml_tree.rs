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

    pub fn get_elements_at_path(&self, path: String ) -> Option<Vec<XMLElement>>{
        let mut ret_vec = Vec::new();
        let mut ret_children = Vec::new();
        let path_vec: Vec<&str> = path.split('/').collect();
        let root_node = &self.get_root();
        let root_tag = root_node.get_tag();
        if path_vec.get(1).unwrap().ne(root_tag) {
            return None;
        }

        self.get_elements_at_path_helper(&root_node.children, &mut ret_vec, path_vec, 2);
        if ret_vec.is_empty() {
            return None;
        }
        for child in ret_vec {
            ret_children.push(child);
        }
        return Some(ret_children);
    }
    fn get_elements_at_path_helper(&self, children: &Vec<Rc<RefCell<XMLElement>>>, ret_children: &mut Vec<XMLElement>, path: Vec<&str>, tree_depth: usize) {
        if children.is_empty() || path.get(tree_depth).is_none(){
            
            return;
        }
        let mut new_vec = Vec::new();
        for child in children {
            let child_ref = child.as_ref();
            let child_borrow = RefCell::try_borrow(child_ref).unwrap();
            let tag = child_borrow.get_tag();
            if tag.eq(path.get(tree_depth).unwrap()) {
                let child_ref = child.clone();
                new_vec.push(child_ref);
            }
            if tag.eq(path.last().unwrap()) {
                let child_ref = child.as_ref();
                let child_borrow = RefCell::try_borrow(child_ref).unwrap(); 
                ret_children.push(child_borrow.clone());
            }
        }
       self.get_elements_at_path_helper(&new_vec, ret_children, path, tree_depth + 1)
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
