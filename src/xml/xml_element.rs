use std::cell::Ref;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
/// This struct represents an XML Element.  This struct can hold the tag name of the XML element,
/// as well as the properties, the children, and the inner text of the XML element.    
pub struct XMLElement {
    pub(in crate::xml) tag: String,
    pub(in crate::xml) props: Vec<(String, String)>,
    pub(in crate::xml) children: Vec<Rc<RefCell<XMLElement>>>,
    pub(in crate::xml) inner_text: String,
}
impl fmt::Display for XMLElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Create string that contains the properties of the XML element
        let mut map = String::new();
        for key in &self.props {
            map.push_str(&key.0);
            map.push('=');
            map.push_str(&key.1);
            map.push_str("; ");
        }
        // Make a string of information about the XML element
        let mut str = String::new();
        let push_str = format!("tag: {}\nprops: {}\ninnertext: {}\nchildren: \n\n", self.tag, map, self.inner_text);
        str.push_str(&push_str);
        // Get information about the children of the XML element
        self.make_string(1, self.get_children(), &mut str);
        // Return that string
        return write!(f, "{}", str);
    }
}

impl XMLElement {
    /// Creates new XML element.  Takes the tag name and properties of the XML element as parameters
    pub(in crate::xml) fn new(tag: String, props: Vec<(String, String)>) -> XMLElement {
        return XMLElement {
            tag: String::from(tag),
            props: props,
            children: Vec::new(),
            inner_text: String::new(),
        };
    }
    /// Add a child to the current XMLElement
    pub(in crate::xml) fn add_child(&mut self, child: Rc<RefCell<XMLElement>>) {
        self.children.push(child);
    }
    /// Add a property to the current XMLElement
    pub(in crate::xml) fn add_property(&mut self, key: String, value: String) {
        self.props.push((key, value));
    }
    /// Add inner text to the current XMLElement.  This method pushes the new string to
    /// the current existing string within the XML file
    pub(in crate::xml) fn add_text(&mut self, text: String) {
        self.inner_text.push_str(&text);
    }
    /// Returns the tag of the current XMLElement.
    pub fn get_tag(&self) -> &String {
        return &self.tag;
    }
    /// Returns the inner text of the current XMLElement.
    pub fn get_inner_text(&self) -> &String {
        return &self.inner_text;
    }
    /// Returns the props of the current XMLElement.
    pub fn get_props(&self) -> &Vec<(String, String)> {
        return &self.props;
    }
    pub fn get_prop(&self, search: String) -> Option<(String, String)> {
        for prop in &self.props {
            if search.eq(&prop.0) {
                return Some(prop.clone());
            }
        }
        return None
    }
    /// Returns the children ofthe current XMLElement
    pub fn get_children(&self) -> Vec<Ref<'_, XMLElement>> {
        let mut ret_vec = Vec::new();
        for child in &self.children {
            let child_ref = child.as_ref();
            let child_borrow = RefCell::try_borrow(child_ref).unwrap();
            ret_vec.push(child_borrow);
        }
        return ret_vec;
    }
    fn make_string(&self, tree_depth: usize, children: Vec<Ref<'_, XMLElement>>, string: &mut String) {
        let ident:usize = 5;
        let width = 2;
        let curr_ident = ident * tree_depth;
        if children.is_empty(){
            return;
        }
        for child in children {
            let mut map = String::new();
            for key in child.get_props() {
                map.push_str(&key.0);
                map.push('=');
                map.push_str(&key.1);
                map.push_str("; ");
            }

            let push_str = format!(
            "{:>curr_ident$}tag: {}\n{:>curr_ident$}props: {}\n{:>curr_ident$}innertext: {}\n{:<curr_ident$}children: \n\n"," ", child.tag," ", map, " ", child.inner_text," ");
            string.push_str(&push_str);
            self.make_string(tree_depth + 1, child.get_children(), string);
        }
    }
}
