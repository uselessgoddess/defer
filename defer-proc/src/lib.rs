use proc_macro::TokenStream;

const DEFER_NAME: &str = "_anonymous_defer_";

#[proc_macro_attribute]
pub fn use_defer(attr: TokenStream, item: TokenStream) -> TokenStream {
    assert!(attr.is_empty(), "macro does not support attributes");

    let defer_decl = format!("let mut {DEFER_NAME} = defer::Defer::new();");

    let mut str_view = item.to_string();
    let pos = str_view.find('{').unwrap() + 1;
    str_view.insert_str(pos, defer_decl.as_str());
    str_view.parse().unwrap()
}

#[proc_macro]
pub fn defer(item: TokenStream) -> TokenStream {
    format!("{DEFER_NAME}.push(move || {{ {item} }});",)
        .parse()
        .unwrap()
}
