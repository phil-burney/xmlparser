use std::cell::{RefCell};
use std::rc::Rc;
use std::fmt;
use xml_element;

struct XMLTree {
    root: Option<Rc<RefCell<XMLElement>>>,
    current_path: Vec<Rc<RefCell<XMLElement>>>
}

impl fmt::Display for XMLTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let root_cell = self.root.as_deref().unwrap();
        let root = RefCell::try_borrow_mut(root_cell).unwrap().to_string();

        return write!(f, "{}", root);

    }
}