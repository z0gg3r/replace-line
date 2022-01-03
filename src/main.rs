// SPDX-FileCopyrightText: 2021 zocker <zockerfreunde03.info@gmx.de>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#[macro_use]
extern crate clap;
use clap::{ App, Arg, ArgGroup };

fn main() {
        let matches = App::new(crate_name!())
                .version(crate_version!())
                .author(crate_authors!())
                .about(crate_description!())
                .arg(Arg::new("PATTERN")
                        .index(1)
                        .takes_value(true)
                        .help("The pattern that the line that should be replaced must contain")
                )
                .arg(Arg::new("pattern")
                        .takes_value(true)
                        .short('p')
                        .long("-pattern")
                        .help("The pattern that the line that should be replaced must contain")
                )
                .arg(Arg::new("REPLACEMENT")
                        .takes_value(true)
                        .index(2)
                        .help("The line that any matched line should be replaced with")
                )
                .arg(Arg::new("replacement")
                        .takes_value(true)
                        .short('r')
                        .long("-replacement")
                        .help("The line that any matched line shoud be replaced with")
                )
                .arg(Arg::new("file")
                        .takes_value(true)
                        .short('f')
                        .long("-file")
                        .help("Read from this file. If not given, read from stdin instead.")
                )
                .group(ArgGroup::new("patterns")
                        .args(&["PATTERN", "pattern"])
                        .required(true))
                .group(ArgGroup::new("replacements")
                        .args(&["REPLACEMENT", "replacement"]))
                .get_matches();
    
        let file = matches.value_of("file").unwrap_or("").to_string();
        let pattern = matches.value_of("patterns").unwrap_or("").to_string();
        let replacement = matches.value_of("replacemnts").unwrap_or("").to_string();
        
        if !pattern.is_empty() && file.is_empty() {
                let mut v: Vec<String> = vec![];

                loop {
                        let mut input = String::new();

                        let bytes = std::io::stdin()
                                .read_line(&mut input)
                                .expect("ERROR 4: Could not read from stdin!");
                        input = input.trim().to_string();

                        if bytes == 0 {
                                break;
                        }

                        if !replacement.is_empty() {
                                if input.contains(&pattern) {
                                        v.push(replacement.clone());
                                } else {
                                        v.push(input);
                                }
                        } else if !input.contains(&pattern) {
                                v.push(input);
                        }
                }

                for line in v {
                        println!("{}", line);
                }
        } else {
                let input = read_file(file.clone());
                let input = if input.is_ok() {
                        input.unwrap()
                } else {
                        String::new()
                };
                if !input.is_empty() {
                        let v = replace(
                                split(input),
                                pattern.clone(),
                                replacement.clone(),
                        );

                        for line in v {
                                println!("{}", line);
                        }
                }
        }
}

fn split(s: String) -> Vec<String> {
        let lines: Vec<&str> = s.split('\n').collect();

        let mut splits: Vec<String> = Vec::with_capacity(lines.len());

        for line in lines {
                splits.push(String::from(line));
        }

        splits
}

fn replace(src: Vec<String>, pattern: String, replacement: String) -> Vec<String> {
        let mut v: Vec<String> = vec![];

        for line in src {
                if !replacement.is_empty() {
                        if line.contains(&pattern) {
                                v.push(replacement.clone());
                        } else {
                                v.push(line);
                        }
                } else if !line.contains(&pattern) {
                        v.push(line);
                }
        }

        v
}

fn read_file(
        _path: String,
) -> Result<String, Box<dyn std::error::Error + 'static>> {
        let path = std::path::Path::new(&_path);

        if !path.exists() {
                eprintln!("ERROR 1: {} is not a valid path!", _path);
        }

        if !path.is_file() {
                eprintln!("ERROR 2: {} is valid, but not a file!", _path);
        }

        let file = std::fs::read_to_string(path)?;

        Ok(file)
}
