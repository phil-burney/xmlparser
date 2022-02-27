use std::cell::{RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;


struct XMLElement {
    tag: String,
    props: HashMap<String, String>,
    children: Vec<Rc<RefCell<XMLElement>>>,
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
        let mut children_str = String::new();

        for child in &self.children {
            let x = RefCell::try_borrow_mut(&child).unwrap().to_string();
            children_str.push_str("\n");
            children_str.push_str(&x);
        }
        return write!(f, "\t{{\n\ttag: {} \n\tprops: {} \n\tinnertext: {}\n\tchildren: {}}}", self.tag, map, self.inner_text, children_str);
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
   
    fn add_child(&mut self, child: Rc<RefCell<XMLElement>>) {
        self.children.push(child);
    }
    fn add_property(&mut self, key: String, value: String) {
        self.props.insert(key, value);
    }
    fn add_text(&mut self, text: String) {
        self.inner_text.push_str(&text);
    }
}