use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote_spanned,
    visit_mut::{self, VisitMut},
    Expr, Lit,
};

/// Converts `&str` literals to `&[char]`.
///
/// ```rust
/// use utf32_lit::utf32;
/// let s = utf32!("æbc");
/// assert_eq!(s, &['æ', 'b', 'c']);
///
/// let s_array = utf32!(&["foo", "bubble", "baz"]);
/// let expected: &[&[char]] = &[&['f', 'o', 'o'], &['b', 'u', 'b', 'b', 'l', 'e'], &['b', 'a', 'z']];
/// assert_eq!(s_array, expected);
/// ```
#[proc_macro]
pub fn utf32(input: TokenStream) -> TokenStream {
    let mut expr = parse_macro_input!(input as Expr);
    Utf32Replace.visit_expr_mut(&mut expr);
    quote!(#expr).into()
}

struct Utf32Replace;

impl VisitMut for Utf32Replace {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Lit(expr) = &node {
            if let Lit::Str(s) = &expr.lit {
                let string = s.value();
                let chars = string.chars();
                *node = parse_quote_spanned!(s.span()=> { let out: &[char] = &[#(#chars),*]; out });
            }
        }
        visit_mut::visit_expr_mut(self, node);
    }
}

/// Converts all `&str` literals to `&[char]`.
///
/// ```rust
/// use utf32_lit::utf32_all_strings;
/// #[utf32_all_strings]
/// mod strings {
///     pub const NAME: &[char] = "hello";
/// }
///
/// assert_eq!(strings::NAME, &['h', 'e', 'l', 'l', 'o']);
/// ```
#[proc_macro_attribute]
pub fn utf32_all_strings(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input);
    Utf32Replace.visit_item_mut(&mut item);
    quote!(#item).into()
}
