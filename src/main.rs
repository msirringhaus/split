use clap::Parser;
use std::io::{self, BufRead};

#[derive(Parser)]
#[command(about = "Split input strings and extract certain columns", long_about = None)]
#[command(version = "1.0")]
struct Cli {
    /// Keep empty split-elements, if delimiter is repeated. Ignored, if no Delimiter is given.
    #[arg(short, long)]
    keep_empty: bool,

    /// Use the complement of the selected columns, meaning all columns except the specified ones.
    #[arg(long)]
    complement: bool,

    /// With which new delimiter the resulted split should be joined (if multiple indices are picked).
    #[arg(short, long, default_value = " ")]
    join_delimiter: String,

    /// Extract given columns. Separate by commas. Starts counting with 1. Negative values count from the back. For negative values at the beginning, use the notation equal-sign: -c=-1,1,-2
    #[arg(short, long, value_parser=validate_columns, value_delimiter=',')]
    column: Vec<i64>,

    /// Which Delimiter should be used to split. If no delimiter is given, all whitespaces are used.
    #[arg(value_name = "DELIMITER")]
    delimiter: Option<String>,
}

impl Cli {
    fn split_line<'b>(&self, line: &'b str) -> Vec<&'b str> {
        let res: Vec<_> = match &self.delimiter {
            Some(delim) => line.split(delim).collect(),
            None => line.split_whitespace().collect(),
        };

        if !self.keep_empty {
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
        if self.column.is_empty() {
            if self.complement {
                // Complement of "all" is "nothing"
                res = Vec::new();
            } else {
                res = all_splits.to_owned();
            }
        } else {
            let mut filtered_columns: Vec<_> = self
                .column
                .iter()
                .map(|i| {
                    if *i > 0 {
                        i - 1
                    } else {
                        all_splits.len() as i64 + i
                    }
                }) // clap verified != 0
                .filter(|i| i < &(all_splits.len() as i64) && i >= &0)
                .collect();
            if self.complement {
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
        }
        res
    }
}

fn validate_columns(v: &str) -> Result<i64, String> {
    match v.parse::<i64>() {
        Ok(val) if val != 0 => Ok(val),
        _ => Err(format!("The value \"{}\" is not allowed", v)),
    }
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    for line in io::stdin().lock().lines() {
        let line = &line?;
        let splits = cli.split_line(line);
        let filtered = cli.pick_columns(&splits);
        println!("{}", filtered.join(&cli.join_delimiter));
    }
    Ok(())
}
