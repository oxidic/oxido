use crate::{store::Store, token::Token};
use colored::Colorize;

pub fn error(i: Option<Token>, v: Token, store: &Store) -> String {
    let syntax_error = "SyntaxError:".red().bold();
    format!(
        "--> {}
|{}. {}
|{} Expected {} found {} instead!
|Exiting due to previous error
       ",
        store.file_name,
        store.lines.at.to_string().blue(),
        store.lines.text.underline(),
        syntax_error,
        v.to_string().cyan().bold(),
        i.unwrap().to_string().cyan().bold(),
    )
}

pub fn check_syntax(i: Option<Token>, v: Token, store: &Store) {
    if i.unwrap() != v {
        println!("{}", error(i, v, store));
        std::process::exit(1);
    }
}

pub fn parse_ident(x: &String, store: &Store) -> String {
    if x.chars()
        .map(|f| f.is_alphabetic())
        .collect::<Vec<bool>>()
        .contains(&true)
    {
        let mut flag = false;
        x.trim()
            .split(" ")
            .map(|f| {
                if f.starts_with("\"") {
                    flag = !flag;
                    f.to_string()
                } else if !f
                    .chars()
                    .map(|f| f.is_alphabetic())
                    .collect::<Vec<bool>>()
                    .contains(&false)
                    && flag == false
                    && f != "true"
                    && f != "false"
                {
                    store.variables.get(f).unwrap().to_string()
                } else {
                    f.to_string()
                }
            })
            .collect::<String>()
    } else {
        x.to_string()
    }
}
