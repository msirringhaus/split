use std::io::{self, BufRead};

use clap::{Arg, App, ArgMatches};

trait SplitFunctions {
    fn split_line<'b>(&self, line: &'b str) -> Vec<&'b str>;
    fn pick_columns<'b>(&self, all_splits: &'b [&str]) -> Vec<&'b str>;
}

impl<'a> SplitFunctions for ArgMatches<'a> {
    fn split_line<'b>(&self, line: &'b str) -> Vec<&'b str> {
        let res : Vec<_>;
        match self.value_of("DELIMITER") {
            Some(delim) => {res = line.split(delim).collect();},
            None        => {res = line.split_whitespace().collect();},
        }

        if !self.is_present("keep-empty") {
            res.iter()
                  .filter(|x| !x.is_empty())
                  .map(ToOwned::to_owned)
                  .collect()
        } else {
            res
        }
    }

    fn pick_columns<'b>(&self, all_splits: &'b [&str]) -> Vec<&'b str> {
        let res;
        match self.values_of("column") {
            Some(all_columns) => {
                res = all_columns
                            .into_iter()
                            .map(|i| i.parse::<usize>().unwrap()) // ok to unwrap, has been verified by clap
                            .filter(|i| i < &all_splits.len())
                            .filter_map(|i| all_splits.get(i))
                            .map(ToOwned::to_owned)
                            .collect();
            },

            None => {res = all_splits.to_owned();},
        }
        res
    }
}

fn validate_columns(v: String) -> Result<(), String> {
    if v.parse::<usize>().is_ok() { return Ok(()); }
    Err(String::from(format!("The value \"{}\" is not an integer", v)))
}


fn main() -> Result<(), std::io::Error> {
    let opt = App::new("split")
                 .version("1.0")
                 .about("Split input strings and extract certain columns")
                 .arg(Arg::with_name("keep-empty")
                     .short("k")
                     .help("Keep empty split-elements, if delimiter is repeated. Ignored, if no Delimiter is given.")
                     .takes_value(false))
                 .arg(Arg::with_name("join-delimiter")
                     .short("j")
                     .help("With which new delimiter the resulted split should be joined (if multiple indeces are picked")
                     .takes_value(true)
                     .default_value(" "))
                 .arg(Arg::with_name("column")
                     .short("c")
                     .takes_value(true)
                     .value_delimiter(",")
                     .validator(validate_columns)
                     .help("Extract given columns"))
                 .arg(Arg::with_name("DELIMITER")
                     .required(false)
                     .help("Which Delimiter should be used to split. If no delimiter is given, all whitespaces are used"))
                 .get_matches();

    for line in io::stdin().lock().lines() {
        let line = &line?;
        let splits = opt.split_line(line);
        let filtered = opt.pick_columns(&splits);
        println!("{}", filtered.join(opt.value_of("join-delimiter").unwrap()));
    }
    Ok(())
}
