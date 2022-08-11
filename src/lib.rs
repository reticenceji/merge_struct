pub use derive_merge::Merge;

pub trait Merge {
    fn merge(&mut self, another: &Self);
}

// #[cfg(test)]
// mod test {

#[derive(Merge)]
struct TestStruct {
    a: ::std::option::Option<i32>,
    b: ::std::option::Option<String>,
    c: ::std::option::Option<u32>,
    #[force]
    d: Option<i32>,
}

#[test]
fn test() {
    let mut this = TestStruct {
        a: Some(1),
        b: None,
        c: Some(1),
        d: Some(1),
    };
    let that = TestStruct {
        a: Some(2),
        b: Some("hello".to_string()),
        c: None,
        d: None,
    };
    this.merge(&that);
    assert_eq!(this.a, Some(2));
    assert_eq!(this.b, Some("hello".to_string()));
    assert_eq!(this.c, Some(1));
    assert_eq!(this.d, None);
}
