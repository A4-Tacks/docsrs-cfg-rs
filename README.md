Quick use `doc_cfg` on docsrs

```rust
use docsrs_cfg::docsrs_cfg;
#[docsrs_cfg(feature = "std")]
fn foo() {}
```

Expand to:

```rust
#[cfg(any(docsrs, feature = "std"))]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
fn foo() {}
```
