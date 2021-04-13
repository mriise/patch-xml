use std::cell::RefCell;
use std::rc::Rc;

use crate::patch_structure::{ComplexQuery, ComplexValue, ModificationValue, Query};
use crate::xml_structure::bidirectional_xml_tree::*;
use std::fs::File;

pub struct PatchProcessor {
    pub xml_tree: XmlTree,
}

impl PatchProcessor {
    pub fn new(xml_string: &str) -> PatchProcessor {
        PatchProcessor {
            xml_tree: XmlTree::new(&xmltree::Element::parse(xml_string.as_bytes()).unwrap()),
        }
    }
    #[allow(dead_code)]
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
    pub fn apply(&mut self, patch: &Query) {
        //Go through patch rules and apply each on the given xml-structure
        //Work just on one xml structure. Each entry is executed on the result of the previous one
        Self::apply_query(
            &patch,
            &Rc::new(RefCell::new(XmlNode {
                parent: None,
                //Encapsulate parsed xml-tree to simplify traversal
                data: XmlNodeData::Element(Element {
                    prefix: None,
                    name: "internal_root".to_string(),
                    attributes: Vec::new(),
                    applied_regexp: None,
                    children: vec![self.xml_tree.root.clone()],
                }),
            })),
        );
    }
    /**
    This method applies a QueryChildType on a given XML element. Depending on the type either:
      - a simple value is assigned
      - or the recursion will continue
     **/
    fn apply_query(query: &Query, xml_node: &Rc<RefCell<XmlNode>>) {
        // Do we have a simple value assignment or sub-queries?
        match query {
            Query::Simple(v) => {
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
            Query::ComplexVec(v) => v
                .iter()
                .for_each(|q| Self::apply_complex_query(q, xml_node)),
            Query::Complex(complex_query) => Self::apply_complex_query(&complex_query, &xml_node),
        }
    }

    fn apply_complex_query(complex_query: &ComplexQuery, xml_node: &Rc<RefCell<XmlNode>>) {
        let ComplexQuery {
            subqueries,
            modification,
            modifier,
        } = complex_query;
        if subqueries.len() == 0
            && modification.is_none()
            && modifier.copy.is_none()
            && modifier.move_to.is_none()
        {
            // If empty set is assigned to a query: Clear the corresponding element
            xml_node.borrow_mut().clear_children();
        } else {
            for (regex, query) in subqueries {
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
                let children = xml_node.borrow_mut().children();
                for child_candidate in children {
                    let name = match &child_candidate.borrow().name() {
                        Some(name) => Some(name.clone()),
                        None => None,
                    };
                    match name {
                        Some(name) => {
                            if regex.regex.is_match(name.as_str()) {
                                child_candidate
                                    .borrow_mut()
                                    .set_regex(Some(regex.regex.clone()));
                                Self::apply_query(&query, &child_candidate);
                                child_candidate.borrow_mut().set_regex(None);
                            }
                        }
                        None => {}
                    }
                }
            }
            //  3. Run applyModifications on current path
            match &modification {
                None => {}
                Some(value_type) => {
                    Self::modify(&value_type, &xml_node);
                }
            }
            //  4. Run move/copy on current path
            match &modifier.copy {
                None => {}
                Some(copy_expression) => {
                    XmlNode::move_copy_node(&xml_node, copy_expression, MoveCopyAction::Copy)
                }
            }
            match &modifier.move_to {
                None => {}
                Some(move_expression) => {
                    XmlNode::move_copy_node(&xml_node, move_expression, MoveCopyAction::Move)
                }
            }
        }
    }
    pub fn modify(value_type: &ModificationValue, current_node: &Rc<RefCell<XmlNode>>) {
        match value_type {
            ModificationValue::SimpleValue(v) => {
                current_node.borrow_mut().clear_children();
                match v.to_xml_node(&current_node) {
                    None => {}
                    Some(n) => {
                        XmlTree::append(current_node, n);
                    }
                }
            }
            ModificationValue::ComplexValue(complex_value) => {
                Self::modify_by_complex_value(&current_node, complex_value)
            }
            ModificationValue::ComplexValueVec(v) => v.iter().for_each(|complex_value| {
                Self::modify_by_complex_value(&current_node, complex_value)
            }),
        }
    }

    fn modify_by_complex_value(current_node: &&Rc<RefCell<XmlNode>>, complex_value: &ComplexValue) {
        let ComplexValue {
            subvalues,
            attributes,
            ..
        } = complex_value;
        for (mod_type, value_type) in subvalues {
            let mut updated = false;
            if mod_type.mod_type.is_modify() {
                for child in current_node.borrow().children() {
                    //ToDo: Evaluation must be applied correctly
                    let name = child.borrow().name();
                    if name.is_some() && name.unwrap() == mod_type.identifier.evaluate(current_node)
                    {
                        updated = true;
                        Self::modify(value_type, &child);
                    }
                }
            }
            if updated == false && !mod_type.mod_type.is_replace() {
                let new_child = XmlTree::append(
                    current_node,
                    XmlNodeData::Element(Element {
                        prefix: None,
                        name: mod_type.identifier.evaluate(current_node),
                        attributes: Vec::new(),
                        applied_regexp: None,
                        children: vec![],
                    }),
                );
                Self::modify(value_type, &new_child);
            }
        }
        match attributes {
            Some(attributes) => {
                for (patch_attribute_name, patch_attribute_value) in attributes {
                    match &mut current_node.borrow_mut().data {
                        XmlNodeData::Element(e) => {
                            match (
                                e.attributes
                                    .iter_mut()
                                    .find(|(key, _)| key == patch_attribute_name)
                                    .map(|(_, v)| v),
                                patch_attribute_value.eval_to_string(current_node),
                            ) {
                                (None, None) => {
                                    //In this case, an unavailable attribute should be removed. We could throw an error here
                                }
                                (Some(_), None) => e
                                    .attributes
                                    .retain(|(name, _)| name != patch_attribute_name),
                                (None, Some(value)) => {
                                    e.attributes.push((patch_attribute_name.clone(), value))
                                }
                                (Some(target_attribute_value), Some(value)) => {
                                    *target_attribute_value = value
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;
    use crate::patch_structure;

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
    }
    mod attribute_tests {
        use super::*;

        #[test]
        fn simple_unpatched_attribute() {
            test_patch(
                indoc!(
                    r#"<element attr1="value1" attr2="value2" attr3="value3" attr4="value4" attr5="value5">Foo</element>"#
                ),
                indoc!(r#"element: Bar"#),
                indoc!(
                    r#"<element attr1="value1" attr2="value2" attr3="value3" attr4="value4" attr5="value5">Bar</element>"#
                ),
            );
        }
        #[test]
        fn simple_patched_attribute() {
            test_patch(
                indoc!(r#"<element attr1="value1" attr2="value2">Foo</element>"#),
                indoc!(
                    r#"
                element:
                    $modify:
                        $attributes:
                            attr1: "new value1"
                            attr2: ~
                            attr3: "new value3"
                "#
                ),
                indoc!(r#"<element attr1="new value1" attr3="new value3">Foo</element>"#),
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
                      subelement1: Bar1
                      subelement2: Bar2
                    "#
                ),
                indoc!(
                    r#"<element><subelement1>Bar1</subelement1><subelement2>Bar2</subelement2></element>"#
                ),
            );
        }
    }
    /*mod filter_tests {
        use super::*;
    }*/
    mod move_copy_tests {
        use super::*;

        #[test]
        fn simple_rename() {
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
        }
        #[test]
        fn simple_move() {
            test_patch(
                indoc!(
                    r#"<element><subelement><subsubelement>Foo</subsubelement></subelement></element>"#
                ),
                indoc!(
                    r#"
                    element:
                      subelement:
                        subsubelement:
                          $move: ../subelement2/
                    "#
                ),
                indoc!(
                    r#"<element><subelement /><subelement2><subsubelement>Foo</subsubelement></subelement2></element>"#
                ),
            );
        }
        #[test]
        fn simple_change_and_move() {
            test_patch(
                indoc!(
                    r#"<element><subelement><subsubelement>Foo</subsubelement></subelement></element>"#
                ),
                indoc!(
                    r#"
                    element:
                      subelement:
                        subsubelement:
                          $move: ../subelement2/
                          $modify:
                              subsubsubelement: 34
                    "#
                ),
                indoc!(
                    r#"<element><subelement /><subelement2><subsubelement>Foo<subsubsubelement>34</subsubsubelement></subsubelement></subelement2></element>"#
                ),
            );
        }
        #[test]
        fn simple_move2() {
            test_patch(
                indoc!(
                    r#"<element><subelement><subsubelement>Foo</subsubelement></subelement></element>"#
                ),
                indoc!(
                    r#"
                    element:
                      subelement:
                        subsubelement:
                          $move: subelement2/
                    "#
                ),
                indoc!(
                    r#"<element><subelement><subelement2><subsubelement>Foo</subsubelement></subelement2></subelement></element>"#
                ),
            );
        }
        #[test]
        fn simple_copy() {
            test_patch(
                indoc!(
                    r#"<element><subelement><subsubelement>Foo</subsubelement></subelement></element>"#
                ),
                indoc!(
                    r#"
                    element:
                      subelement:
                        subsubelement:
                          $copy: ../subelement2/
                    "#
                ),
                indoc!(
                    r#"<element><subelement><subsubelement>Foo</subsubelement></subelement><subelement2><subsubelement>Foo</subsubelement></subelement2></element>"#
                ),
            );
        }
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
