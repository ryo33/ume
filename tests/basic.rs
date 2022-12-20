use ume::ume;

#[test]
fn test_simple() {
    fn simple() -> String {
        format!(
            "{}",
            ume! {
                use std::collections::HashMap;

                pub struct Foo {
                    pub bar: HashMap<String, String>,
                }
            }
        )
    }
    assert_eq!(
        simple(),
        "use std :: collections :: HashMap ; pub struct Foo\n{ pub bar : HashMap < String, String >, }"
    );
}

#[test]
fn test_interpolation() {
    let name = r#""Ryo""#;
    let ret = ume! {
        fn main() {
            println!("Hello, {}!", #name);
        }
    }
    .to_string();
    assert_eq!(ret, "fn main() { println! (\"Hello, {}!\", \"Ryo\") ; }");
}

#[test]
fn test_multiple() {
    let a = r#"use a::b;"#;
    let b = "let a = 1;";
    let c = "let b = 2;";
    let ret = ume! {
        #a
        fn main() {
            #b
            #c
        }
    }
    .to_string();
    assert_eq!(ret, "use a::b; fn main() { let a = 1; let b = 2; }");
}

#[test]
fn test_example() {
    let use_something = ume!(
        use something::prelude::*;
    );
    let let_one = ume!(let one = 1;);
    let let_two = ume!(let two = 2;);
    let ret = ume! {
        #use_something
        fn main() {
            #let_one
            #let_two
            println!("{one} {two}");
        }
    }
    .to_string();
    assert_eq!(ret, "use something :: prelude :: * ; fn main() { let one = 1 ; let two = 2 ; println! (\"{one} {two}\") ; }");
}

#[test]
fn test_attributes() {
    let ret = format!(
        "{}",
        ume! {
            #![crate_type = "lib"]
            #[test]
            fn test_foo() {}
        }
    );
    assert_eq!(ret, r#"#! [crate_type = "lib"] #[test] fn test_foo() {}"#);
}

#[test]
fn test_doc_comments() {
    let ret = format!(
        "{}",
        ume! {
            /// This is a doc comment.
            /// Line 2
            fn foo() {}
        }
    );
    assert_eq!(ret, r#"#[doc = " This is a doc comment."] #[doc = " Line 2"] fn foo() {}"#);
}
