# README

It's a template to write rust derive macro.

The trait is designed for named struct.
- If the field of struct has attribute `#[exclude]`, `this.field` remain the same.
- ElIf the field of struct is not `Option`: `this.field = that.field.clone()`
- ElIf the field of struct has attribute `#[force]`: `this.field = that.field.clone()`
- ElIf the field of struct is `Option` and doesn't have attribute `#[force]`:
    - If `that.field.is_some()`: `this.field = that.field.clone()`
    - If `that.field.is_none()`: `this.field` remain the same.


```rust
pub trait MergeProto {
    fn merge_proto(&mut self, another: &Self);
}

#[derive(MergeProto)]
struct TestStruct {
    a: Option<i32>,
    b: Option<String>,
    c: Option<u32>,
    #[force]
    d: Option<i32>,
    #[exclude]
    e: Option<i32>
}
let mut this = TestStruct {
    a: Some(1),
    b: None,
    c: Some(1),
    d: Some(1),
    e: None
};
let that = TestStruct {
    a: Some(2),
    b: Some("hello".to_string()),
    c: None,
    d: None,
    e: Some(1)
};

this.merge_proto(&that);
assert_eq!(this.a, Some(2));
assert_eq!(this.b, Some("hello".to_string()));
assert_eq!(this.c, Some(1));
assert_eq!(this.d, None);
assert_eq!(this.e, None);
```
