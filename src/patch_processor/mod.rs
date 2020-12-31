use crate::patch_structure::{self, QueryChildType};
use crate::xml_structure::xml_path::{PathSegment, XmlPath};
use crate::xml_structure::xml_tree::XmlTree;
use std::fs::File;
use xmltree::XMLNode;

pub struct PatchProcessor {
    pub xml_tree: XmlTree,
}

impl PatchProcessor {
    pub fn new(xml_string: &str) -> PatchProcessor {
        PatchProcessor {
            //Encapsulate parsed xml-tree to simplify traversal
            xml_tree: XmlTree::new(xmltree::Element::parse(xml_string.as_bytes()).unwrap()),
        }
    }
    pub fn get_root(&self) -> &xmltree::Element {
        self.xml_tree
            .xml_tree
            .children
            .iter()
            .filter_map(xmltree::XMLNode::as_element)
            .nth(0)
            .unwrap()
    }
    pub fn write_result(&self, path: &String) {
        match self.xml_tree.xml_tree.write(File::create(path).unwrap()) {
            Ok(_) => {}
            Err(_) => panic!("Error while writing result"),
        }
    }
    //ToDo: return Result-Type
    pub fn apply(&mut self, patch: &QueryChildType) {
        let path = XmlPath::new();
        //Go through patch rules and apply each on the given xml-structure
        //Work just on one xml structure. Each entry is executed on the result of the previous one
        match patch {
            QueryChildType::SimpleValue(_) => {
                panic!("A simple value on root level is not allowed in the patch file")
            }
            QueryChildType::QuerySet(queries) => {
                self.apply_queries(queries, &path);
            }
        }
    }
    fn apply_queries(&mut self, queries: &Vec<patch_structure::Query>, path: &XmlPath) {
        if queries.len() == 0 {
            // If empty set is assigned to a query: Clear the corresponding element
            self.xml_tree.get_element_by_path(&path).children.clear();
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
                    self.xml_tree.get_element_by_path(&path).children.clear();
                } else {
                    for subquery in &query.subqueries {
                        for child_candidate in self.xml_tree.get_children_names(&path) {
                            if subquery.0.regex.is_match(child_candidate.as_str()) {
                                let mut sub_path = path.clone();
                                sub_path.segments.push(PathSegment {
                                    name: child_candidate,
                                    regex: subquery.0.regex.clone(),
                                });
                                self.apply_query_child_type(subquery.1, &sub_path)
                            }
                        }
                    }
                }
                //  3. Run applyModifications on current path
                match &query.modification {
                    None => {}
                    Some(value_type) => {
                        self.xml_tree.modify(&value_type, &path);
                    }
                }
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
    fn apply_query_child_type(&mut self, query_child_type: &QueryChildType, path: &XmlPath) {
        match query_child_type {
            QueryChildType::SimpleValue(v) => {
                let mut path = XmlPath {
                    segments: path.segments.clone(),
                };
                match v.to_xml_node(&mut &path) {
                    None => {
                        let element_to_remove = path
                            .segments
                            .pop()
                            .expect("Removing a global element is not allowed!");
                        let current_element = self.xml_tree.get_element_by_path(&path);
                        current_element.children.retain(|c| match c {
                            XMLNode::Element(e) => e.name != element_to_remove.name,
                            _ => false,
                        })
                    }
                    Some(c) => {
                        let current_element = self.xml_tree.get_element_by_path(&path);
                        current_element.children = vec![c];
                    }
                }
            }
            QueryChildType::QuerySet(qs) => self.apply_queries(qs, path),
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
        match processor.get_root().write(&mut result_bytes) {
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
    mod modification_tests {
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
    }
}
