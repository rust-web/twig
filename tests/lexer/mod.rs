use twig::api::lexer::{ Lexer, Tokens, Options };
use twig::api::tokens::TokenRef;

#[test]
fn name_label_for_tag() {
    let template = "{% § %}";
    let lexer = Lexer::new(Options::default(), Vec::new());
    let mut s = lexer.tokens(&template);

    expect(&mut s, TokenRef::BlockStart);
    expect(&mut s, TokenRef::Name("§"));
}

fn expect<'i, 'c>(stream: &mut Tokens<'i, 'c>, token_value: TokenRef<'c>) {
    if let Err(e) = stream.expect(token_value) {
        panic!("Received error {:?}", e);
    }
}
