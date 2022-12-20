use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn ume(input: TokenStream) -> TokenStream {
    let mut ctx = Ctx { idents: vec![] };
    let string = ctx
        .transform(input)
        .to_string()
        .replace('{', "{{")
        .replace('}', "}}")
        .replace("___ume_ume", "{}");
    let mut inner: Vec<_> = ctx
        .idents
        .into_iter()
        .flat_map(|ident| {
            [
                TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                TokenTree::Ident(ident),
            ]
        })
        .collect();
    inner.insert(0, TokenTree::Literal(Literal::string(&string)));
    TokenStream::from_iter(vec![
        TokenTree::Ident(Ident::new("format_args", Span::call_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter(inner),
        )),
    ])
}

struct Ctx {
    pub idents: Vec<Ident>,
}

impl Ctx {
    fn transform(&mut self, input: TokenStream) -> TokenStream {
        let mut tokens = vec![];
        let mut iter = input.into_iter();
        while let Some(token) = iter.next() {
            match &token {
                TokenTree::Group(group) => tokens
                    .push(Group::new(group.delimiter(), self.transform(group.stream())).into()),
                TokenTree::Punct(punct) => {
                    if punct.as_char() == '#' {
                        if let Some(next_token) = iter.next() {
                            if let TokenTree::Ident(ident) = next_token {
                                self.idents.push(ident.clone());
                                tokens.push(Ident::new("___ume_ume", Span::call_site()).into());
                            } else {
                                tokens.push(token);
                                tokens.push(next_token);
                            }
                        }
                    } else {
                        tokens.push(token)
                    }
                }
                _ => tokens.push(token),
            }
        }
        TokenStream::from_iter(tokens)
    }
}
