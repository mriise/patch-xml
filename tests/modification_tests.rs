mod utils;

use indoc::indoc;
use utils::test_patch;

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
        indoc!(r#"<element><subelement>Foo</subelement><subelement>Bar</subelement></element>"#),
    );
}
