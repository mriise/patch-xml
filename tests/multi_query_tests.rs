mod utils;

use indoc::indoc;
use utils::test_patch;

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
