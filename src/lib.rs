#![doc = include_str!("../README.md")]

use proc_macro::*;
use proc_macro::Spacing::*;
use proc_macro::Delimiter::*;
use proc_macro::TokenTree as T;


#[doc = include_str!("../README.md")]
#[proc_macro_attribute]
pub fn docsrs_cfg(attr: TokenStream, item: TokenStream) -> TokenStream {
    macro_impl("docsrs", attr, item)
}

/// Like [`docsrs_cfg!()`], but use `doc` instead of `docsrs`
///
/// ```
/// use docsrs_cfg::doc_cfg;
/// #[doc_cfg(feature = "std")]
/// fn foo() {}
/// ```
///
/// Expand to:
///
/// ```
/// #[cfg(any(doc, feature = "std"))]
/// #[cfg_attr(doc, doc(cfg(feature = "std")))]
/// fn foo() {}
/// ```
#[proc_macro_attribute]
pub fn doc_cfg(attr: TokenStream, item: TokenStream) -> TokenStream {
    macro_impl("doc", attr, item)
}

fn macro_impl(kind: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    let span = Span::call_site();
    [
        TokenStream::from(T::Punct(Punct::new('#', Joint))),
        TokenStream::from(T::Group(Group::new(Bracket, [
            T::Ident(Ident::new("cfg", span)),
            T::Group(Group::new(Parenthesis, [
                T::Ident(Ident::new("any", span)),
                T::Group(Group::new(Parenthesis, [
                    T::Ident(Ident::new(kind, span)).into(),
                    T::Punct(Punct::new(',', Alone)).into(),
                    attr.clone(),
                ].into_iter().collect())),
            ].into_iter().collect())),
        ].into_iter().collect()))),
        TokenStream::from(T::Punct(Punct::new('#', Spacing::Joint))),
        TokenStream::from(T::Group(Group::new(Bracket, [
            T::Ident(Ident::new("cfg_attr", span)),
            T::Group(Group::new(Parenthesis, [
                T::Ident(Ident::new(kind, span)),
                T::Punct(Punct::new(',', Alone)),
                T::Ident(Ident::new("doc", span)),
                T::Group(Group::new(Parenthesis, [
                    T::Ident(Ident::new("cfg", span)),
                    T::Group(Group::new(Parenthesis, attr)),
                ].into_iter().collect())),
            ].into_iter().collect())),
        ].into_iter().collect()))),
        item,
    ].into_iter().collect()
}

/// ```
/// #[docsrs_cfg::__test_docsrs(any(feature = "std", feature = "alloc"))]
/// fn foo() {}
/// ```
#[doc(hidden)]
#[proc_macro_attribute]
pub fn __test_docsrs(attr: TokenStream, item: TokenStream) -> TokenStream {
    let output = docsrs_cfg(attr, item.clone()).to_string()
        .parse::<TokenStream>().unwrap().to_string();
    let expect = r#"
#[cfg(any(docsrs, any(feature = "std", feature = "alloc")))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "std", feature = "alloc"))))]
fn foo() {}
    "#.parse::<TokenStream>().unwrap().to_string();

    if output.trim() == expect.trim() {
        item
    } else {
        let span = Span::call_site();
        [
            T::Ident(Ident::new("compile_error", span)),
            T::Punct(Punct::new('!', Joint)),
            T::Group(Group::new(Brace, T::Literal(
                Literal::string(&format!(
                    "assert failed, expected:\n{}\ncurrent:\n{}",
                    expect.trim(),
                    output.trim(),
                ))
            ).into())),
        ].into_iter().collect()
    }
}

/// ```
/// #[docsrs_cfg::__test_doc(any(feature = "std", feature = "alloc"))]
/// fn foo() {}
/// ```
#[doc(hidden)]
#[proc_macro_attribute]
pub fn __test_doc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let output = doc_cfg(attr, item.clone()).to_string()
        .parse::<TokenStream>().unwrap().to_string();
    let expect = r#"
#[cfg(any(doc, any(feature = "std", feature = "alloc")))]
#[cfg_attr(doc, doc(cfg(any(feature = "std", feature = "alloc"))))]
fn foo() {}
    "#.parse::<TokenStream>().unwrap().to_string();

    if output.trim() == expect.trim() {
        item
    } else {
        let span = Span::call_site();
        [
            T::Ident(Ident::new("compile_error", span)),
            T::Punct(Punct::new('!', Joint)),
            T::Group(Group::new(Brace, T::Literal(
                Literal::string(&format!(
                    "assert failed, expected:\n{}\ncurrent:\n{}",
                    expect.trim(),
                    output.trim(),
                ))
            ).into())),
        ].into_iter().collect()
    }
}
