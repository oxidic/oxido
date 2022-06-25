use crate::{
    store::{Function, Store},
    token::Token,
    util::check_syntax,
};
use logos::Lexer;

pub fn parse_function(mut lex: Lexer<Token>, mut store: Store) -> Store {
    check_syntax(lex.next(), Token::Function, &store);
    check_syntax(lex.next(), Token::Ident, &store);
    let name = lex.slice();
    check_syntax(lex.next(), Token::SquareBraceOpen, &store);
    let mut arguments = vec![];

    loop {
        if lex.next().unwrap() == Token::SquareBraceClose {
            break;
        }
        let arg = lex.slice().to_string();
        if arg == "," {
            continue;
        }
        arguments.push(arg);
    }

    store.functions.insert(
        name.to_string(),
        Function {
            arguments,
            lines: vec![],
        },
    );
    store.states.functions.capturing = true;
    store.states.functions.current = name.to_string();
    store.states.stack.push(String::from("function"));

    store
}
