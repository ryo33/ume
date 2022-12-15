use ume::ume;

#[test]
fn test_simple() {
    fn simple() -> String {
        ume! {
            use std::collections::HashMap;

            pub struct Foo {
                pub bar: HashMap<String, String>,
            }
        }
        .into()
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
    };
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
    };
    assert_eq!(ret, "use a::b; fn main() { let a = 1; let b = 2; }");
}
