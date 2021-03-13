use crate::patch_structure::ReferenceExpression;
use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use xmltree::XMLNode;

pub struct XmlTree {
    pub root: Rc<RefCell<XmlNode>>,
}

impl XmlTree {
    pub fn new(parsed_tree: &xmltree::Element) -> XmlTree {
        let mut xml_tree = XmlTree {
            root: Rc::new(RefCell::new(XmlNode {
                parent: None,
                data: XmlNodeData::Element(Self::parse_from_element(parsed_tree)),
            })),
        };
        Self::add_element_children(&mut xml_tree.root, parsed_tree);
        xml_tree
    }
    fn parse_from_element(xmltree_element: &xmltree::Element) -> Element {
        Element {
            prefix: xmltree_element.prefix.clone(),
            name: xmltree_element.name.clone(),
            applied_regexp: None,
            children: vec![],
        }
    }
    fn add_element_children(parent: &mut Rc<RefCell<XmlNode>>, xmltree_element: &xmltree::Element) {
        for c in &xmltree_element.children {
            match c {
                XMLNode::Element(e) => {
                    let mut child =
                        Self::append(parent, XmlNodeData::Element(Self::parse_from_element(&e)));
                    Self::add_element_children(&mut child, &e);
                }
                XMLNode::Comment(c) => {
                    Self::append(parent, XmlNodeData::Comment(c.clone()));
                }
                XMLNode::CData(c) => {
                    Self::append(parent, XmlNodeData::CData(c.clone()));
                }
                XMLNode::Text(t) => {
                    Self::append(parent, XmlNodeData::Text(t.clone()));
                }
                XMLNode::ProcessingInstruction(k, v) => {
                    Self::append(
                        parent,
                        XmlNodeData::ProcessingInstruction(k.clone(), v.clone()),
                    );
                }
            }
        }
    }
    // Appends `data` to the chain of nodes. The implementation is recursive
    // but one could rewrite it to use a while-let imperative loop instead
    // without too much effort.
    pub fn append(node: &Rc<RefCell<XmlNode>>, data: XmlNodeData) -> Rc<RefCell<XmlNode>> {
        // If the current node is the last one, create a new node,
        // set its prev pointer to the current node, and store it as
        // the node after the current one.
        let rc = Rc::new(RefCell::new(XmlNode {
            parent: Some(Rc::downgrade(&node)),
            data: data.clone(),
        }));
        match &mut node.deref().deref().borrow_mut().deref_mut().data {
            XmlNodeData::Element(element) => element.children.push(rc.clone()),
            _ => panic!("Children can only be added to elements"),
        }
        rc
    }

    pub fn to_xmltree(&self) -> xmltree::Element {
        match &self.root.deref().borrow().data {
            XmlNodeData::Element(e) => Self::element_to_xmltree_element(e),
            _ => panic!("Root node of XML must be an element"),
        }
    }
    fn node_to_xmltree_node(node: &Rc<RefCell<XmlNode>>) -> xmltree::XMLNode {
        match &node.deref().borrow().data {
            XmlNodeData::Element(e) => {
                xmltree::XMLNode::Element(Self::element_to_xmltree_element(&e))
            }
            XmlNodeData::Comment(s) => xmltree::XMLNode::Comment(s.clone()),
            XmlNodeData::CData(s) => xmltree::XMLNode::CData(s.clone()),
            XmlNodeData::Text(s) => xmltree::XMLNode::Text(s.clone()),
            XmlNodeData::ProcessingInstruction(k, v) => {
                xmltree::XMLNode::ProcessingInstruction(k.clone(), v.clone())
            }
        }
    }
    fn element_to_xmltree_element(element: &Element) -> xmltree::Element {
        xmltree::Element {
            prefix: element.prefix.clone(),
            namespace: None,
            namespaces: None,
            name: element.name.clone(),
            attributes: Default::default(),
            children: element
                .children
                .iter()
                .map(|c| Self::node_to_xmltree_node(c))
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct XmlNode {
    pub parent: Option<Weak<RefCell<XmlNode>>>,
    pub data: XmlNodeData,
}

pub enum MoveCopyAction {
    Move,
    Copy,
}

impl XmlNode {
    pub fn children(&self) -> XmlNodeIntoIterator {
        match &self.data {
            XmlNodeData::Element(e) => XmlNodeIntoIterator {
                nodes: e.children.iter().map(Rc::downgrade).collect(),
                index: 0,
            },
            _ => XmlNodeIntoIterator {
                nodes: vec![],
                index: 0,
            },
        }
    }
    pub fn clear_children(&mut self) -> bool {
        match &mut self.data {
            XmlNodeData::Element(e) => {
                e.children.clear();
                true
            }
            _ => false,
        }
    }
    pub fn remove(node: Rc<RefCell<XmlNode>>) -> bool {
        let parent = match &node.borrow().parent {
            None => {
                return false;
            }
            Some(p) => p.upgrade(),
        };
        let parent = match parent {
            None => {
                return false;
            }
            Some(p) => p.clone(),
        };
        node.borrow_mut().parent = None;
        let result = parent.borrow_mut().cleanup_dependencies();
        result
    }
    pub fn cleanup_dependencies(&mut self) -> bool {
        match &mut self.data {
            XmlNodeData::Element(e) => {
                e.children.retain(|e| e.deref().borrow().parent.is_some());
            }
            _ => {
                return false;
            }
        }
        true
    }
    pub fn name(&self) -> Option<String> {
        match &self.data {
            XmlNodeData::Element(e) => Some(e.name.clone()),
            _ => None,
        }
    }
    pub fn set_name(&mut self, new_name: &String) -> bool {
        match &mut self.data {
            XmlNodeData::Element(e) => {
                e.name = new_name.clone();
                true
            }
            _ => false,
        }
    }
    pub fn get_node_info_by_path(
        current_node: Rc<RefCell<XmlNode>>,
        path: Vec<String>,
        auto_create: bool,
    ) -> Rc<RefCell<XmlNode>> {
        let mut current_node = current_node.clone();
        for segment in &path {
            match segment.as_str() {
                ".." => {
                    let new_node = match &current_node.deref().borrow().parent {
                        None => panic!("Path reaches end of XML tree!"),
                        Some(p) => p.upgrade().unwrap().clone(),
                    };
                    current_node = new_node;
                }
                "." => {}
                queried_name => {
                    let children_candidates: Vec<(String, Rc<RefCell<XmlNode>>)> = current_node
                        .deref()
                        .borrow()
                        .children()
                        .filter_map(|c| match c.deref().borrow().name() {
                            None => None,
                            Some(name) => Some((name, c.clone())),
                        })
                        .filter(|(name, _)| name == &String::from(queried_name))
                        .collect();
                    current_node = match &children_candidates.len() {
                        1 => children_candidates.first().unwrap().clone().1,
                        0 => {
                            if auto_create {
                                XmlTree::append(
                                    &current_node,
                                    XmlNodeData::Element(Element {
                                        prefix: None,
                                        name: queried_name.to_string(),
                                        applied_regexp: None,
                                        children: vec![],
                                    }),
                                )
                            } else {
                                panic!("Path not found!")
                            }
                        }
                        _ => panic!("More than one XML node is matching the path!"),
                    };
                }
            }
        }
        current_node
    }
    pub fn set_regex(&mut self, regex: Option<Regex>) {
        match &mut self.data {
            XmlNodeData::Element(e) => e.applied_regexp = regex,
            _ => {}
        }
    }
    pub fn get_regex(&self) -> Option<Regex> {
        match &self.data {
            XmlNodeData::Element(e) => e.applied_regexp.clone(),
            _ => None,
        }
    }
    pub fn deep_clone(node: Rc<RefCell<XmlNode>>) -> Rc<RefCell<XmlNode>> {
        let (node_data, children) = match &node.borrow().data {
            XmlNodeData::Element(e) => (
                XmlNodeData::Element(e.deep_clone()),
                Some(e.children.clone()),
            ),
            XmlNodeData::Comment(s) => (XmlNodeData::Comment(s.clone()), None),
            XmlNodeData::CData(s) => (XmlNodeData::CData(s.clone()), None),
            XmlNodeData::Text(s) => (XmlNodeData::Text(s.clone()), None),
            XmlNodeData::ProcessingInstruction(k, v) => (
                XmlNodeData::ProcessingInstruction(k.clone(), v.clone()),
                None,
            ),
        };
        let cloned = Rc::new(RefCell::new(XmlNode {
            parent: None,
            data: node_data,
        }));
        match children {
            None => {}
            Some(children) => {
                for c in children {
                    let cloned_child = (&XmlNode::deep_clone(c).borrow().data).clone();
                    XmlTree::append(&cloned, cloned_child);
                }
            }
        }
        cloned
    }
    pub fn move_copy_node(
        xml_parent_node: &Rc<RefCell<XmlNode>>,
        move_copy_expression: &ReferenceExpression,
        move_copy: MoveCopyAction,
    ) {
        //Moving parent_node to somewhere else...
        let move_expression = move_copy_expression.evaluate(xml_parent_node);
        let path = move_expression.split("/").map(String::from);
        let mut path = path.collect_vec();
        let new_name = path.pop().unwrap();
        if !new_name.is_empty() {
            if !xml_parent_node.borrow_mut().set_name(&new_name) {
                panic!("Could not set name \"{}\" for XML node.", new_name)
            }
        }
        if !path.is_empty() {
            //Start searching from parent of parent_node (the location of parent_node)...
            let parent_parent_node = match &xml_parent_node.borrow().parent {
                None => panic!("Root node is not allowed to be moved..."),
                Some(p) => p.upgrade().unwrap(),
            };
            let new_parent_node =
                XmlNode::get_node_info_by_path(parent_parent_node.clone(), path, true);
            match move_copy {
                MoveCopyAction::Move => {
                    XmlNode::remove(xml_parent_node.clone());
                    let xml_node_data = xml_parent_node.borrow().data.clone();
                    XmlTree::append(&new_parent_node, xml_node_data);
                }
                MoveCopyAction::Copy => {
                    // Copying...
                    let xml_node_data = XmlNode::deep_clone(xml_parent_node.clone())
                        .borrow()
                        .data
                        .clone();
                    XmlTree::append(&new_parent_node, xml_node_data.clone());
                }
            }
        }
    }
}

impl PartialEq for XmlNode {
    fn eq(&self, other: &Self) -> bool {
        let parents_equal = match (&self.parent, &other.parent) {
            (None, None) => true,
            (Some(s), Some(o)) => match (s.upgrade(), o.upgrade()) {
                (None, None) => true,
                (Some(s), Some(o)) => s.deref().borrow().deref() == o.deref().borrow().deref(),
                (_, _) => false,
            },
            (_, _) => false,
        };
        &self.data == &other.data && parents_equal
    }
}

impl Eq for XmlNode {}

pub struct XmlNodeIntoIterator {
    nodes: Vec<Weak<RefCell<XmlNode>>>,
    index: usize,
}

impl Iterator for XmlNodeIntoIterator {
    type Item = Rc<RefCell<XmlNode>>;
    fn next(&mut self) -> Option<Rc<RefCell<XmlNode>>> {
        let result = match self.nodes.get(self.index) {
            None => None,
            Some(node_ref) => match node_ref.upgrade() {
                None => None,
                Some(node_cell) => Some(node_cell.clone()),
            },
        };
        self.index += 1;
        result
    }
}

#[derive(Clone, Debug)]
pub enum XmlNodeData {
    Element(Element),
    Comment(String),
    CData(String),
    Text(String),
    ProcessingInstruction(String, Option<String>),
}

impl PartialEq for XmlNodeData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (XmlNodeData::Element(e1), XmlNodeData::Element(e2)) => e1.name == e2.name,
            (XmlNodeData::Comment(c1), XmlNodeData::Comment(c2)) => c1 == c2,
            (XmlNodeData::CData(c1), XmlNodeData::CData(c2)) => c1 == c2,
            (
                XmlNodeData::ProcessingInstruction(k1, v1),
                XmlNodeData::ProcessingInstruction(k2, v2),
            ) => k1 == k2 && v1 == v2,
            (XmlNodeData::Text(t1), XmlNodeData::Text(t2)) => t1 == t2,
            (_, _) => false,
        }
    }
}
impl Eq for XmlNodeData {}

#[derive(Clone, Debug)]
pub struct Element {
    /// This elements prefix, if any
    pub prefix: Option<String>,

    /// The name of the Element.  Does not include any namespace info
    pub name: String,

    //This regular expression is set while traversing down the XML-tree. When going back, it is resetted again.
    pub applied_regexp: Option<Regex>,

    pub children: Vec<Rc<RefCell<XmlNode>>>,
}

impl Element {
    pub fn deep_clone(&self) -> Element {
        Element {
            prefix: self.prefix.clone(),
            name: self.name.clone(),
            applied_regexp: self.applied_regexp.clone(),
            children: vec![],
        }
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        let children_equal = self.children.len() == other.children.len()
            && self
                .children
                .iter()
                .zip(&other.children)
                .map(|(c1, c2)| c1.deref().borrow().deref() == c2.deref().borrow().deref())
                .all(|b| b);
        self.prefix == other.prefix && self.name == other.name && children_equal
    }
}

impl Eq for Element {}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn read_xml_tree(xml_str: &str) -> XmlTree {
        let xml_tree = xmltree::Element::parse(xml_str.as_bytes()).unwrap();
        XmlTree::new(&xml_tree)
    }

    mod single_query_tests {
        use super::*;

        fn get_test_xml_tree() -> XmlTree {
            XmlTree {
                root: Rc::new(RefCell::new(XmlNode {
                    parent: None,
                    data: XmlNodeData::Element(Element {
                        prefix: None,
                        name: "element".to_string(),
                        applied_regexp: None,
                        children: vec![],
                    }),
                })),
            }
        }

        #[test]
        fn successful_append() {
            let foo_element = XmlNodeData::Text(String::from("Foo"));
            let mut xmltree = get_test_xml_tree();
            XmlTree::append(&mut xmltree.root, foo_element);
            let root = xmltree.root.deref().borrow();
            assert!(root.parent.is_none());
            let root_element = match &root.data {
                XmlNodeData::Element(e) => Some(e),
                XmlNodeData::Comment(_) => None,
                XmlNodeData::CData(_) => None,
                XmlNodeData::Text(_) => None,
                XmlNodeData::ProcessingInstruction(_, _) => None,
            };
            assert!(root_element.is_some());
            let root_element = root_element.unwrap();
            assert_eq!(root_element.name, String::from("element"));
            assert_eq!(root_element.prefix, None);
            assert_eq!(root_element.children.len(), 1);
            let foo_child = root_element.children.first();
            assert!(foo_child.is_some());
            let foo_child = foo_child.unwrap().deref().borrow();
            assert!(foo_child.parent.is_some());
            let foo_child_parent = match &foo_child
                .parent
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap()
                .deref()
                .borrow()
                .data
            {
                XmlNodeData::Element(e) => Some(e.clone()),
                XmlNodeData::Comment(_) => None,
                XmlNodeData::CData(_) => None,
                XmlNodeData::Text(_) => None,
                XmlNodeData::ProcessingInstruction(_, _) => None,
            };
            assert!(foo_child_parent.is_some());
            let foo_child_parent = foo_child_parent.unwrap();
            assert_eq!(&foo_child_parent, root_element);
            let foo_child = match &foo_child.data {
                XmlNodeData::Element(_) => None,
                XmlNodeData::Comment(_) => None,
                XmlNodeData::CData(_) => None,
                XmlNodeData::Text(t) => Some(t.clone()),
                XmlNodeData::ProcessingInstruction(_, _) => None,
            };
            assert!(foo_child.is_some());
            let foo_child = foo_child.unwrap();
            assert_eq!(foo_child, String::from("Foo"));
        }

        #[test]
        fn simple_element() {
            let foo_element = XmlNodeData::Text(String::from("Foo"));
            let mut xmltree = get_test_xml_tree();
            XmlTree::append(&mut xmltree.root, foo_element);
            assert_eq!(
                read_xml_tree(indoc!(r#"<element>Foo</element>"#)).root,
                xmltree.root
            );
        }
    }
}
