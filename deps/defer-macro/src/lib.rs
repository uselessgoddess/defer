use proc_macro::TokenStream;

const DEFER_NAME: &str = "_anonymous_defer_";

#[proc_macro]
pub fn defer_name(_attr: TokenStream) -> TokenStream {
    format!("{}", DEFER_NAME).parse().unwrap()
}

#[proc_macro_attribute]
pub fn use_defer(attr: TokenStream, item: TokenStream) -> TokenStream {
    assert!(attr.is_empty(),
               "macro does not support attributes");

    let defer_decl = format!("let mut {} = Defer::new();", DEFER_NAME);

    let mut str_view = item.to_string();
    let pos = str_view.find("{").unwrap() + 1;
    str_view.insert_str(pos, defer_decl.as_str());
    str_view.parse().unwrap()
}

#[proc_macro]
pub fn defer(item: TokenStream) -> TokenStream {
    format!("{defer_name}.push(Box::new(|| {{ {body} }}));",
        defer_name = DEFER_NAME,
        body = item.to_string()
    ).parse().unwrap()
}
