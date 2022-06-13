use crate::{token::Token, util::get_value_from_ident};
use logos::Lexer;
use std::collections::HashMap;

pub fn parse_expression<'a>(
    lex: &Lexer<Token>,
    store: HashMap<&'a str, String>,
) -> (String, HashMap<&'a str, String>) {
    let eval_string = get_value_from_ident(&lex.remainder().replace(";", ""), store.clone());

    let r = meval::eval_str(eval_string).unwrap();

    (r.to_string(), store)
}

