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
        let mut res;
        match self.values_of("column") {
            Some(all_columns) => {
                let mut filtered_columns : Vec<_> = all_columns
                                                    .into_iter()
                                                    .map(|i| i.parse::<i64>().unwrap()) // ok to unwrap, has been verified by clap
                                                    .map(|i| if i > 0 { i - 1 } else { all_splits.len() as i64 + i }) // clap verified != 0
                                                    .filter(|i| i < &(all_splits.len() as i64) && i >= &0)
                                                    .collect();
                if self.is_present("complement") {
                    res = all_splits.to_owned();
                    filtered_columns.sort();
                    filtered_columns.dedup();
                    for i in filtered_columns.into_iter().rev() {
                        res.remove(i as usize);
                    }
                } else {
                    res = filtered_columns
                                .into_iter()
                                .filter_map(|i| all_splits.get(i as usize))
                                .map(ToOwned::to_owned)
                                .collect();
                }
            },

            None => {
                if self.is_present("complement") {
                    // Complement of "all" is "nothing"
                    res = Vec::new();
                } else {
                    res = all_splits.to_owned();
                }
            }
        }
        res
    }
}

fn validate_columns(v: String) -> Result<(), String> {
    match v.parse::<i64>() {
       Ok(val) if val != 0 => Ok(()),
       _ => Err(String::from(format!("The value \"{}\" is not allowed", v)))
    }
}


fn main() -> Result<(), std::io::Error> {
    let opt = App::new("split")
                 .version("1.0")
                 .about("Split input strings and extract certain columns")
                 .arg(Arg::with_name("keep-empty")
                     .long("keep-empty")
                     .short("k")
                     .help("Keep empty split-elements, if delimiter is repeated. Ignored, if no Delimiter is given.")
                     .takes_value(false))
                 .arg(Arg::with_name("complement")
                     .long("complement")
                     .help("Use the complement of the selected columns, meaning all columns except the specified ones.")
                     .takes_value(false))
                 .arg(Arg::with_name("join-delimiter")
                     .short("j")
                     .help("With which new delimiter the resulted split should be joined (if multiple indices are picked")
                     .takes_value(true)
                     .default_value(" "))
                 .arg(Arg::with_name("column")
                     .short("c")
                     .takes_value(true)
                     .value_delimiter(",")
                     .validator(validate_columns)
                     .help("Extract given columns. Separate by commas. Starts counting with 1. Negative values count from the back. For negative values, use the notation -c=\"-1\""))
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
