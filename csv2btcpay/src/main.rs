extern crate csv;
#[macro_use]
extern crate configure_me;
extern crate serde_yaml;

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
    MissingColumn { description: &'static str, id: usize, },
    Yaml { file: PathBuf, error: serde_yaml::Error, },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::File { file, error, operation } => write!(f, "failed to {} file {}: {}", operation, file.display(), error),
            Error::Csv { file, error } => write!(f, "failed to read CSV file {}: {}", file.display(), error),
            Error::MissingColumn { description, id, } => write!(f, "the column for {} (position: {}) is missing", description, id),
            Error::Yaml { file, error } => write!(f, "Failed to serialize Yaml file {}: {}", file.display(), error),
        }
    }
}


#[derive(Debug, Serialize)]
struct PosItem {
    title: String,
    price: String,
}

fn run() -> Result<(), Error> {
    let (config, _) = Config::including_optional_config_files(std::iter::empty::<&str>()).unwrap_or_exit();

    let mut reader = mapping_try!(csv::Reader::from_path(&config.input), config.input, |error, file| Error::Csv { file, error, });
    let writer = mapping_try!(std::fs::File::create(&config.output), config.output, |error, file| Error::File { file, error, operation: "write to"});
    let writer = io::BufWriter::new(writer);

    let mut template = std::collections::HashMap::new();

    for item in reader.records() {
        let item = mapping_try!(item, config.input, |error, file| Error::Csv { file, error, });
        let title = item.get(config.title_field).ok_or(Error::MissingColumn { description: "title", id: config.title_field })?;
        let price = item.get(config.price_field).ok_or(Error::MissingColumn { description: "price", id: config.price_field })?;

        let prefix = if let Some(prefix) = config.prefix_field {
            Some(item.get(prefix).ok_or(Error::MissingColumn { description: "title prefix", id: prefix })?)
        } else {
            None
        };

        let key = if let Some(prefix) = prefix {
            format!("{} - {}", prefix.to_lowercase(), title.to_lowercase())
        } else {
            format!("{}", title.to_lowercase())
        };

        let title = if let Some(prefix) = prefix {
            format!("{} - {}", prefix, title)
        } else {
            format!("{}", title)
        };

        let item = PosItem {
            title,
            price: price.to_owned(),
        };

        template.insert(key, item);
    }

    serde_yaml::to_writer(writer, &template).map_err(|error| Error::Yaml { file: config.output, error, })
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
