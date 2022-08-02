## Open Questions

## 30th July

**static lifetime**
when returing a reference to a string from a function, you must specify the static lifetime, indicating that the string literal will not go out of scope once the function scope is dropped.

```rust
impl Get {
    pub fn apply() -> Result<&'static str, Error> {
        let result = "success response";
        Ok(result)
    }
}
```

1. What does #[tokio::main] do?
2. mini redis buffer doesn't have \0 values but our buffer has
3. Undertsand the diff in bytesmute and vector [u8], and why extra \0 was present in the buffer.
4. 'static lifetime difference - blog post
5. Debug fmt blog
6. Difference in Bytes::bytes from {str, io} std, when to use which? Blog topic
7.
