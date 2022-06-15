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
        store.file_name(),
        store.line_number().to_string().blue(),
        store.line_text().underline(),
        syntax_error,
        v.to_string().cyan().bold(),
        i.unwrap().to_string().cyan().bold(),
    )
}

pub fn check_data_type(i: Option<Token>, v: Token, store: &Store) {
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
        x.chars()
            .map(|f| {
                if f == '"' {
                    flag = !flag;
                    f.to_string()
                }
                else if f.is_alphabetic() && flag == false {
                    store.get_variable(&*f.to_string()).unwrap().to_string()
                } else {
                    f.to_string()
                }
            })
            .collect::<String>()
    } else {
        x.to_string()
    }
}
