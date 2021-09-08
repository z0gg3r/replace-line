// SPDX-FileCopyrightText: 2021 zocker <zockerfreunde03.info@gmx.de>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use getopt::Opt;

fn main() {
        let args: Vec<String> = std::env::args().collect();

        let mut opts = getopt::Parser::new(&args, "f:r:p:h");

        let mut file = String::new();
        let mut pattern = String::new();
        let mut sub = String::new();
        let mut help = false;

        loop {
                match opts
                        .next()
                        .transpose()
                        .expect("ERROR 3: Could not parse arguments!")
                {
                        None => break,
                        Some(opt) => match opt {
                                Opt('f', Some(arg)) => file = arg.clone(),
                                Opt('p', Some(arg)) => pattern = arg.clone(),
                                Opt('r', Some(arg)) => sub = arg.clone(),
                                Opt('h', None) => help = true,
                                _ => unreachable!(),
                        },
                }
        }

        if help || pattern.is_empty() {
                print_usage(&args[0]);
        } else if file.is_empty() {
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

                        if !sub.is_empty() {
                                if input.contains(&pattern) {
                                        v.push(sub.clone());
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
                                sub.clone(),
                        );

                        for line in v {
                                println!("{}", line);
                        }
                }
        }
}

fn print_usage(s: &str) {
        println!("{} -p PATTERN [-r REPLACER] [-f FILE]", s);
}

fn split(s: String) -> Vec<String> {
        let lines: Vec<&str> = s.split('\n').collect();

        let mut splits: Vec<String> = Vec::with_capacity(lines.len());

        for line in lines {
                splits.push(String::from(line));
        }

        splits
}

fn replace(src: Vec<String>, pattern: String, sub: String) -> Vec<String> {
        let mut v: Vec<String> = vec![];

        for line in src {
                if !sub.is_empty() {
                        if line.contains(&pattern) {
                                v.push(sub.clone());
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
