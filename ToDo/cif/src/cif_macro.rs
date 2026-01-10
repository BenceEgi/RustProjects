use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Token};

//GRAMMAR
//{condition} ? expr : expr

pub fn process(input: TokenStream) -> TokenStream{

    let syntax = parse_macro_input!(input as Composition);
    let condition = syntax.condition;
    let function_t = syntax.true_expr;
    let function_f = syntax.false_expr;
    quote!{
        if (#condition) {
            #function_t;
        }
        else{
            #function_f;
        }
    }.into()
}

/// Stores the parts of the full expression
struct Composition{
    condition: Condition,
    true_expr: TExpr,
    false_expr: FExpr,
}

impl Parse for Composition{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let condition = input.parse()?;
        let function_t = input.parse()?;
        let function_f = input.parse()?;
        Ok(Self{
            condition,
            true_expr: function_t,
            false_expr: function_f
        })
    }
}

/// Stores the expression for the true branch
struct TExpr(syn::Expr);

impl Parse for TExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![?] = input.parse()?;
        input.parse().map(Self)
    }
}

impl ToTokens for TExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

/// Stores the expression for the false branch
struct FExpr(syn::Expr);

impl Parse for FExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![:] = input.parse()?;
        input.parse().map(Self)
    }
}

impl ToTokens for FExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

/// Stores the expression block for the condition
struct Condition(syn::ExprBlock);

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

impl ToTokens for Condition{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}