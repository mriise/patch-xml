mod utils;

use indoc::indoc;
use utils::test_patch;

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
        indoc!(r#"<element><subelement>Foo</subelement><subelement>Bar</subelement></element>"#),
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
