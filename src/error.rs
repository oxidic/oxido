use std::ops::Range;
use std::process::exit;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

pub fn error(
    name: &str,
    source: &str,
    code: &str,
    message: &str,
    note: &str,
    range: Range<usize>,
) -> ! {
    let mut files = SimpleFiles::new();

    let file_id = files.add(name, source);

    let diagnostic = Diagnostic::error()
        .with_message(message)
        .with_code("E".to_owned() + code)
        .with_labels(vec![Label::primary(file_id, range).with_message(note)])
        .with_notes(vec!["note: ".to_owned() + note]);

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    term::emit(&mut writer.lock(), &config, &files, &diagnostic).unwrap();

    exit(1)
}
