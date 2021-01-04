use crate::patch_structure::{self, QueryChildType};
use crate::xml_structure::bidirectional_xml_tree::*;
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;

pub struct PatchProcessor {
    pub xml_tree: XmlTree,
}

impl PatchProcessor {
    pub fn new(xml_string: &str) -> PatchProcessor {
        PatchProcessor {
            //Encapsulate parsed xml-tree to simplify traversal
            xml_tree: XmlTree::new(&xmltree::Element::parse(xml_string.as_bytes()).unwrap()),
        }
    }
    pub fn write_result(&self, path: &String) {
        match self
            .xml_tree
            .to_xmltree()
            .write(File::create(path).unwrap())
        {
            Ok(_) => {}
            Err(_) => panic!("Error while writing result"),
        }
    }
    //ToDo: return Result-Type
    pub fn apply(&mut self, patch: &QueryChildType) {
        // let path = XmlPath::new();
        //Go through patch rules and apply each on the given xml-structure
        //Work just on one xml structure. Each entry is executed on the result of the previous one
        //Self::apply_query(&(patch), &self.xml_tree.root);

        match patch {
            QueryChildType::SimpleValue(_) => {
                panic!("A simple value on root level is not allowed in the patch file")
            }
            QueryChildType::QuerySet(queries) => {
                Self::apply_queries(
                    queries,
                    &Rc::new(RefCell::new(XmlNode {
                        parent: None,
                        data: XmlNodeData::Element(Element {
                            prefix: None,
                            name: "internal_root".to_string(),
                            applied_regexp: None,
                            children: vec![self.xml_tree.root.clone()],
                        }),
                    })),
                );
            }
        }
    }
    fn apply_queries(
        queries: &Vec<patch_structure::Query>,
        xml_parent_node: &Rc<RefCell<XmlNode>>,
    ) {
        if queries.len() == 0 {
            // If empty set is assigned to a query: Clear the corresponding element
            xml_parent_node.borrow_mut().clear_children();
        } else {
            for query in queries {
                //What do we get for each found query?
                //  - List of selection structures (selection list) that contains
                //      - The individual element name of the children flattened from...
                //          - each pattern entry
                //          - each match per pattern entry
                //      - The RegExp-Match for referencing reasons

                //Constraints:
                //  - Move, copy and modify are not allowed on root-level (no empty path!)
                //  - No filtering, initially. First get an answer for following question:
                //      - How to deal with filtering ($if)? It should be...
                //          - intuitive
                //          - flexible (and/or/greater/lesser/prefix,...)
                //          - referencable

                //What will we do for each found subelement?
                //  1. Run filter ($if). If filter is not matching: Skip!
                //  2. Run apply_query_child_type for each elemment in selection list by appending the path by their individual name
                if query.subqueries.len() == 0
                    && !query.modifier.is_modifying()
                    && query.modification.is_none()
                {
                    // If empty set is assigned to a query: Clear the corresponding element
                    xml_parent_node.borrow_mut().clear_children();
                } else {
                    for subquery in &query.subqueries {
                        let children = xml_parent_node.borrow_mut().children();
                        for child_candidate in children {
                            let name = match &child_candidate.borrow().name() {
                                Some(name) => Some(name.clone()),
                                None => None,
                            };
                            match name {
                                Some(name) => {
                                    if subquery.0.regex.is_match(name.as_str()) {
                                        child_candidate
                                            .borrow_mut()
                                            .set_regex(Some(subquery.0.regex.clone()));
                                        Self::apply_query_child_type(subquery.1, &child_candidate);
                                        child_candidate.borrow_mut().set_regex(None);
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                }
                //  3. Run applyModifications on current path
                /*match &query.modification {
                    None => {}
                    Some(value_type) => {
                        self.xml_tree.modify(&value_type, &path);
                    }
                }*/
                //  4. Run move/copy on current path
                /*match &query.modifier.move_to {
                    None => {}
                    Some(move_expression) => {
                        let move_expression = move_expression.evaluate(&path);
                        let mut path = path.clone();
                        let remaining = path.apply(move_expression);
                    }
                }*/
            }
        }
    }
    /**
    This method applies a QueryChildType on a given XML element. Depending on the type either:
      - a simple value is assigned
      - or the recursion will continue
     **/
    fn apply_query_child_type(query_child_type: &QueryChildType, xml_node: &Rc<RefCell<XmlNode>>) {
        // Do we have a simple value assignment or sub-queries?
        match query_child_type {
            QueryChildType::SimpleValue(v) => {
                // Apply the simple value:
                match v.to_xml_node(xml_node) {
                    None => {
                        // If no XML node is returned, then the simple value indicates a removal of the current XML element:
                        XmlNode::remove(xml_node.clone());
                    }
                    Some(c) => {
                        let children = xml_node.borrow().children();
                        children.for_each(|c| {
                            XmlNode::remove(c);
                        });
                        XmlTree::append(&xml_node, c);
                    }
                }
            }
            QueryChildType::QuerySet(qs) => {
                if qs.len() == 0 {
                    // If empty set is assigned to a query: Clear the corresponding element
                    xml_node.borrow_mut().clear_children();
                } else {
                    Self::apply_queries(qs, &xml_node);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn test_patch(xml_str: &str, yaml_str: &str, expected_result: &str) {
        let mut processor = PatchProcessor::new(xml_str);
        let patch_structure = match patch_structure::parse(&yaml_str.to_string()) {
            Ok(Some(patch)) => patch,
            Ok(None) => panic!("No patch defined!"),
            Err(_) => panic!("Error while reading patch"),
        };
        processor.apply(&patch_structure);
        let mut result_bytes = Vec::new();
        match processor.xml_tree.to_xmltree().write(&mut result_bytes) {
            Ok(_) => {}
            Err(msg) => panic!("Error while writing result: {}", msg),
        }
        let result_str = String::from_utf8(result_bytes).unwrap();
        assert_eq!(
            result_str.as_str(),
            format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>{}"#,
                expected_result
            )
        );
    }

    mod single_query_tests {
        use super::*;

        #[test]
        fn simple_pattern() {
            test_patch(
                indoc!(r#"<element>Foo</element>"#),
                indoc!(
                    r#"
                    element:
                      Bar"#
                ),
                indoc!(r#"<element>Bar</element>"#),
            );
        }
        #[test]
        fn simple_boolean() {
            test_patch(
                indoc!(r#"<element>Foo</element>"#),
                indoc!(
                    r#"
                    element:
                      true"#
                ),
                indoc!(r#"<element>true</element>"#),
            );
        }
        #[test]
        fn simple_unsigned() {
            test_patch(
                indoc!(r#"<element>Foo</element>"#),
                indoc!(
                    r#"
                    element:
                      23"#
                ),
                indoc!(r#"<element>23</element>"#),
            );
        }
        #[test]
        fn simple_signed() {
            test_patch(
                indoc!(r#"<element>Foo</element>"#),
                indoc!(
                    r#"
                    element:
                      -33"#
                ),
                indoc!(r#"<element>-33</element>"#),
            );
        }
        #[test]
        fn simple_remove() {
            test_patch(
                indoc!(r#"<element><subelement>Foo</subelement></element>"#),
                indoc!(
                    r#"
                    element:
                        subelement: ~"#
                ),
                indoc!(r#"<element />"#),
            );
        }
        #[test]
        fn simple_clear() {
            test_patch(
                indoc!(r#"<element><subelement>Foo</subelement></element>"#),
                indoc!(
                    r#"
                    element:
                        subelement: {}"#
                ),
                indoc!(r#"<element><subelement /></element>"#),
            );
        }
        #[test]
        fn simple_double_clear() {
            test_patch(
                indoc!(
                    r#"<element><subelement>Foo</subelement><subelement>Bar</subelement></element>"#
                ),
                indoc!(
                    r#"
                    element:
                        subelement: {}"#
                ),
                indoc!(r#"<element><subelement /><subelement /></element>"#),
            );
        }
        #[test]
        fn regex_query() {
            test_patch(
                indoc!(r#"<element>Foo</element>"#),
                indoc!(
                    r#"
                    el.+:
                      Bar"#
                ),
                indoc!(r#"<element>Bar</element>"#),
            );
        }
        #[test]
        fn no_matching_regex_query() {
            test_patch(
                indoc!(r#"<element>Foo</element>"#),
                indoc!(
                    r#"
                    ela.+:
                      Bar"#
                ),
                indoc!(r#"<element>Foo</element>"#),
            );
        }
        /*        #[test]
                fn borrow_check() {
                    use crate::xml_structure::bidirectional_xml_tree::*;
                    let mut xml_tree = XmlTree::new();
                    XmlTree::append(
                        &mut xml_tree.root,
                        XmlNodeData::Element(Element {
                            prefix: None,
                            name: "device1".to_string(),
                            children: vec![],
                        }),
                    );
                    XmlTree::append(
                        &mut xml_tree.root,
                        XmlNodeData::Element(Element {
                            prefix: None,
                            name: "device2".to_string(),
                            children: vec![],
                        }),
                    );
                    let result: Vec<String> = xml_tree
                        .root
                        .borrow_mut()
                        .children()
                        .filter_map(|c| match &c.borrow().data {
                            XmlNodeData::Element(e) => Some(e.name.clone()),
                            _ => None,
                        })
                        .collect();
                    assert_eq!(result, vec!["device1".to_string(), "device2".to_string()]);
                }
        */
    }
    mod referencing_tests {
        use super::*;
        #[test]
        fn referencing_query_named() {
            test_patch(
                indoc!(r#"<element>Foo</element>"#),
                indoc!(
                    r#"
                    ele(?P<appendix>.+):
                      Referenced [.:appendix]"#
                ),
                indoc!(r#"<element>Referenced ment</element>"#),
            );
        }
        #[test]
        fn referencing_query_indexed() {
            test_patch(
                indoc!(r#"<element>Foo</element>"#),
                indoc!(
                    r#"
                    ele(.+):
                      Referenced [.:1]"#
                ),
                indoc!(r#"<element>Referenced ment</element>"#),
            );
        }
        #[test]
        fn referencing_query_global() {
            test_patch(
                indoc!(r#"<element>Foo</element>"#),
                indoc!(
                    r#"
                    ele(.+):
                      Referenced [.:0]"#
                ),
                indoc!(r#"<element>Referenced element</element>"#),
            );
        }
        #[test]
        fn referencing_query_global_implicite() {
            test_patch(
                indoc!(r#"<element>Foo</element>"#),
                indoc!(
                    r#"
                    ele(.+):
                      Referenced [.]"#
                ),
                indoc!(r#"<element>Referenced element</element>"#),
            );
        }
        #[test]
        fn referencing_query_multiple_level() {
            test_patch(
                indoc!(
                    r#"<element><subelement><subsubelement>Foo</subsubelement></subelement></element>"#
                ),
                indoc!(
                    r#"
                    ele(.+):
                      subelement:
                        subsubelement:
                          Referenced [../../.:1]"#
                ),
                indoc!(
                    r#"<element><subelement><subsubelement>Referenced ment</subsubelement></subelement></element>"#
                ),
            );
        }
        #[test]
        fn referencing_multiple_parallel() {
            test_patch(
                indoc!(
                    r#"<element><subelement1>Foo1</subelement1><subelement2>Foo2</subelement2></element>"#
                ),
                indoc!(
                    r#"
                    element:
                      subelement(?P<senum>.+): Bar[.:senum]
                    "#
                ),
                indoc!(
                    r#"<element><subelement1>Bar1</subelement1><subelement2>Bar2</subelement2></element>"#
                ),
            );
        }
    }
    mod multi_query_tests {
        use super::*;
        #[test]
        fn successive_change() {
            test_patch(
                indoc!(r#"<element>Foo</element>"#),
                indoc!(
                    r#"
                    - element: Bar
                    - element: Baz
                    "#
                ),
                indoc!(r#"<element>Baz</element>"#),
            );
        }
        #[test]
        fn individual_changes() {
            test_patch(
                indoc!(
                    r#"<element><subelement1>Foo1</subelement1><subelement2>Foo2</subelement2></element>"#
                ),
                indoc!(
                    r#"
                    element:
                      - subelement1: Bar1
                      - subelement2: Bar2
                    "#
                ),
                indoc!(
                    r#"<element><subelement1>Bar1</subelement1><subelement2>Bar2</subelement2></element>"#
                ),
            );
        }
    }
    mod filter_tests {
        // use super::*;
    }
    mod move_copy_tests {
        // use super::*;
        /*#[test]
        fn simple_move() {
            test_patch(
                indoc!(r#"<element>Foo</element>"#),
                indoc!(
                    r#"
                    element:
                      $move: new_element
                    "#
                ),
                indoc!(r#"<new_element>Foo</new_element>"#),
            );
        }*/
    }
    /*mod modification_tests {
        use super::*;
        #[test]
        fn simple_update() {
            test_patch(
                indoc!(r#"<element>Foo</element>"#),
                indoc!(
                    r#"
                    element:
                      $modify: Bar
                    "#
                ),
                indoc!(r#"<element>Bar</element>"#),
            );
        }
        #[test]
        fn complex_update() {
            test_patch(
                indoc!(r#"<element><subelement>Foo</subelement></element>"#),
                indoc!(
                    r#"
                    element:
                      $modify:
                        subelement: Bar
                    "#
                ),
                indoc!(r#"<element><subelement>Bar</subelement></element>"#),
            );
        }
        #[test]
        fn complex_update_implicite_creation() {
            test_patch(
                indoc!(r#"<element></element>"#),
                indoc!(
                    r#"
                    element:
                      $modify:
                        subelement: Bar
                    "#
                ),
                indoc!(r#"<element><subelement>Bar</subelement></element>"#),
            );
        }
        #[test]
        fn complex_add() {
            test_patch(
                indoc!(r#"<element><subelement>Foo</subelement></element>"#),
                indoc!(
                    r#"
                    element:
                      $modify:
                        +subelement: Bar
                    "#
                ),
                indoc!(
                    r#"<element><subelement>Foo</subelement><subelement>Bar</subelement></element>"#
                ),
            );
        }
    }*/
}
