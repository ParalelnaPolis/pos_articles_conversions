extern crate htmlstream;
extern crate csv;
#[macro_use]
extern crate configure_me;

include_config!();

use std::{io, fmt};
use std::path::PathBuf;

macro_rules! mapping_try {
    ($result:expr, $context:expr, $fun:expr) => {
        match $result {
            Ok(item) => item,
            Err(error) => return Err($fun(error, $context)),
        }
    }
}

enum Error {
    File { file: PathBuf, error: io::Error, operation: &'static str },
    Csv { file: PathBuf, error: csv::Error, },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::File { file, error, operation } => write!(f, "failed to {} file {}: {}", operation, file.display(), error),
            Error::Csv { file, error, } => write!(f, "failed to write CSV file {}: {}", file.display(), error),
        }
    }
}


fn drop_first<A, B>((_, b): (A, B)) -> B {
    b
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum State {
    Initial,
    Html,
    Body,
    Div,
    Table,
    Tr,
    Td,
}

fn run() -> Result<(), Error> {
    use htmlstream::{tag_iter, attr_iter};
    use htmlstream::HTMLTagState::{Opening, Closing, Text};
    use State::*;

    let (config, _) = Config::including_optional_config_files(std::iter::empty::<&str>()).unwrap_or_exit();

    let html = mapping_try!(std::fs::read_to_string(&config.input), config.input, |error, file| Error::File { file, error, operation: "read from", });

    let mut state = State::Initial;
    let mut row = Vec::new();
    let mut writer = mapping_try!(csv::Writer::from_path(&config.output), config.output, |error, file| Error::Csv { file, error });

    for tag in tag_iter(&html).map(drop_first) {
        match (state, &*tag.name, tag.state) {
            (Initial, "html", Opening) => state = Html,
            (Html, "html", Closing) => state = Initial,
            (Html, "body", Opening) => state = Body,
            (Body, "body", Closing) => state = Html,
            (Body, "div", Opening) => if attr_iter(&tag.attributes).map(drop_first).any(|attr| attr.name == "class" && attr.value == "table-scrollable") { state = Div },
            (Div, "div", Closing) => state = Body,
            (Div, "table", Opening) => state = Table,
            (Table, "tr", Opening) => state = Tr,
            (Tr, "tr", Closing) => {
                //println!("{:?}", row);
                if row.len() > 0 {
                    mapping_try!(writer.write_record(&row), config.output, |error, file| Error::Csv { file, error });
                }
                row.clear();
                state = Table;
            },
            (Tr, "td", Opening) => state = Td,
            (Td, "td", Closing) => state = Tr,
            (Td, _, Text) => {
                let text = if config.strip_currency && row.len() == 4 {
                    tag.html.trim_end_matches(|c| c != '.' && (c < '0' || c > '9')).to_owned()
                } else {
                    tag.html
                };
                row.push(text);
            },
            _ => (),
        }
        //println!("{:?}", tag);
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
