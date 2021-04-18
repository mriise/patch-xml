mod utils;

use indoc::indoc;
use utils::test_patch;

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
