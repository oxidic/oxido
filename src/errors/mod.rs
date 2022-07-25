use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

pub mod error_keys;

#[derive(Debug, Clone, Copy)]
pub struct Error<'a> {
    pub file: &'a str,
    pub line: &'a str,
    pub code: i32,
    pub message: &'a str,
    pub label: bool,
}

impl<'a> Error<'a> {
    pub fn new(file: &'a str, line: &'a str, code: i32, message: &'a str, label: bool) -> Self {
        Self {
            file,
            line,
            code,
            message,
            label,
        }
    }

    pub fn build(self, error: Error) -> (Diagnostic<usize>, SimpleFiles<&str, &str>) {
        let mut files = SimpleFiles::new();

        let file_id = files.add(error.file, error.line);

        let mut diagnostic: Diagnostic<usize> = Diagnostic::error()
            .with_message(error.message)
            .with_code("E".to_owned() + &error.code.to_string());

        if error.label {
            diagnostic = diagnostic.with_labels(vec![Label::primary(file_id, 0..error.line.len())]);
        }

        (diagnostic, files)
    }

    pub fn throw(file: &'a str, line: &'a str, code: i32, message: &'a str, label: bool) {
        let error = Error::new(file, line, code, message, label);
        let (diagnostic, files) = error.build(error);

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        term::emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();
    }
}
