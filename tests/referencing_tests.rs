mod utils;

use indoc::indoc;
use utils::test_patch;

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
        indoc!(r#"<element><subelement><subsubelement>Foo</subsubelement></subelement></element>"#),
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
