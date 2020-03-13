use crate::Dispatch;
use crate::DispatchTable;

fn greet(params: &str) -> String {
    let mut greeting = String::from("Hello ");
    greeting.push_str(params);
    greeting
}

fn echo(params: &str) -> String {
    params.to_string()
}

fn lifetime_test<'a>(params: &(&'a [u8],)) -> &'a [u8] {
    params.0
}

fn bound_lifetime<'a, 'b: 'a>(params: &(&'a [u8], &'b [u8])) -> (&'a [u8], &'b [u8]) {
    *params
}

#[test]
fn expected_output() {
    let mut map = DispatchTable::new();
    map.insert("greet", Box::new(greet));
    map.insert("echo", Box::new(echo));

    assert_eq!(map.call(&"echo", "asd").unwrap(), String::from("asd"));
    assert_eq!(
        map.call(&"greet", "Skarlett").unwrap(),
        String::from("Hello Skarlett")
    );
}

#[test]
fn accepts_lifetime() {
    let mut map = DispatchTable::new();
    map.insert("lifetime", Box::new(lifetime_test));
    let buf = b"lifetime";
    let x = map.call(&"lifetime", &(buf,)).unwrap();
    assert_eq!(buf, x);
}

#[test]
fn accepts_bound_lifetimes() {
    let mut map = DispatchTable::new();
    map.insert("lifetime", Box::new(bound_lifetime));
    let buf = b"lifetime";
    let (a, b) = map.call(&"lifetime", &(buf, &buf[1..3])).unwrap();
    assert_eq!(buf, a);
    assert_eq!(b, b"if");
}

#[test]
fn has_proper_borrow_semantics() {
    let mut map = DispatchTable::new();
    map.insert("foo".to_string(), Box::new(echo));
    assert_eq!(map.call("foo", "Hello").unwrap(), String::from("Hello"));
    let _x = map.get("foo");
    map.remove(&"abc".to_string());
    let _b = map.contains_key("bar");
}
