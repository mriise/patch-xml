mod utils;

use indoc::indoc;
use utils::test_patch;

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
        indoc!(r#"<element><subelement><subsubelement>Foo</subsubelement></subelement></element>"#),
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
        indoc!(r#"<element><subelement><subsubelement>Foo</subsubelement></subelement></element>"#),
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
        indoc!(r#"<element><subelement><subsubelement>Foo</subsubelement></subelement></element>"#),
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
        indoc!(r#"<element><subelement><subsubelement>Foo</subsubelement></subelement></element>"#),
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
