pub use derive_merge::MergeProto;

pub trait MergeProto {
    fn merge_proto(&mut self, another: &Self);
}

// #[cfg(test)]
// mod test {

#[derive(MergeProto)]
struct TestStruct {
    a: Option<i32>,
    b: Option<String>,
    c: Option<u32>,
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
    this.merge_proto(&that);
    assert_eq!(this.a, Some(2));
    assert_eq!(this.b, Some("hello".to_string()));
    assert_eq!(this.c, Some(1));
    assert_eq!(this.d, None);
}
// }
