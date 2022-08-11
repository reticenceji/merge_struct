# README

这只是一个对过程宏的练手项目

做的事情非常简单，就是给结构体自动实现一个merge方法。

```rust
pub use derive_merge::Merge;

pub trait Merge {
    fn merge(&mut self, another: &Self);
}

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
```

对结构体中所有类型是Option的字段，

- 如果that的值是Some(T)，那么用它更新this
- 如果that的值是None，那么不更新this
- 如果用`#[force]`修饰，那么即使that的值是None，也更新this
