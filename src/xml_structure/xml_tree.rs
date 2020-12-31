use crate::patch_structure::{ModificationType, Value, ValueType};
use crate::xml_structure::xml_path::{PathSegment, XmlPath};
use regex::Regex;
use xmltree::XMLNode;

pub struct XmlTree {
    pub xml_tree: xmltree::Element,
}

impl XmlTree {
    pub fn new(xml_tree: xmltree::Element) -> XmlTree {
        XmlTree {
            //Encapsulate parsed xml-tree to simplify traversal
            xml_tree: xmltree::Element {
                prefix: None,
                namespace: None,
                namespaces: None,
                name: "".to_string(),
                attributes: Default::default(),
                children: vec![xmltree::XMLNode::Element(xml_tree)],
            },
        }
    }

    pub fn get_element_by_path(&mut self, path: &XmlPath) -> &mut xmltree::Element {
        let mut current_element = &mut self.xml_tree;
        for segment in &path.segments {
            match current_element.get_mut_child(segment.name.clone()) {
                None => panic!(
                    "Error, built xml path contains a segment that does not exist in xml tree"
                ),
                Some(e) => current_element = e,
            }
        }
        current_element
    }
    pub fn get_children_names(&mut self, path: &XmlPath) -> Vec<String> {
        let mut result = Vec::new();
        for x in &self.get_element_by_path(&path).children {
            match x {
                XMLNode::Element(e) => result.push(e.name.clone()),
                _ => {}
            }
        }
        result
    }

    fn get_sub_element_list(
        &mut self,
        value: &Value,
        path: &XmlPath,
    ) -> Vec<(String, ValueType, ModificationType)> {
        let element = self.get_element_by_path(path);
        value
            .subvalues
            .iter()
            .map(|(mod_type, v)| {
                let evaluated_name = mod_type.evaluate(path);
                match element.get_mut_child(evaluated_name.clone()) {
                    None => (
                        evaluated_name,
                        v.clone(),
                        ModificationType::Add(mod_type.get_expression().clone()),
                    ),
                    Some(e) => (e.name.clone(), v.clone(), mod_type.clone()),
                }
            })
            .collect()
    }
    pub fn modify(&mut self, value_type: &ValueType, path: &XmlPath) {
        match value_type {
            ValueType::SimpleValue(v) => {
                let mut element = self.get_element_by_path(path);
                match v.to_xml_node(&path) {
                    None => element.children.clear(),
                    Some(n) => element.children = vec![n],
                }
            }
            ValueType::ComplexValues(values) => {
                for value in values {
                    let sub_elements = self.get_sub_element_list(value, path);
                    for (name, value_type, create) in sub_elements {
                        let mut sub_path = path.clone();
                        sub_path.segments.push(PathSegment {
                            name: name.to_string().clone(),
                            regex: Regex::new(name.as_str()).unwrap(),
                        });
                        match create {
                            ModificationType::Modify(_) => {
                                self.modify(&value_type, &sub_path);
                            }
                            ModificationType::Replace(_) => {
                                self.get_element_by_path(&path).take_child(name.clone());
                                self.get_element_by_path(&path).children.push(
                                    xmltree::XMLNode::Element(xmltree::Element {
                                        prefix: None,
                                        namespace: None,
                                        namespaces: None,
                                        name,
                                        attributes: Default::default(),
                                        children: vec![],
                                    }),
                                );
                                //ToDo: Does not work with multiple same named elements. Must be fixed!
                                self.modify(&value_type, &sub_path);
                            }
                            ModificationType::Add(_) => {
                                self.get_element_by_path(&path).children.push(
                                    xmltree::XMLNode::Element(xmltree::Element {
                                        prefix: None,
                                        namespace: None,
                                        namespaces: None,
                                        name,
                                        attributes: Default::default(),
                                        children: vec![],
                                    }),
                                );
                                //ToDo: Does not work with multiple same named elements. Must be fixed!
                                self.modify(&value_type, &sub_path);
                            }
                        }
                    }
                }
            }
        }
    }
}
