use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
#[allow(unused_imports)]
use std::io::{self, BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("lee <1076774025@qq.com>")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input files")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .conflicts_with("lines")
                .help("print the first NUM bytes of each file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("print the first NUM lines instead of the first 10")
                .takes_value(true)
                .default_value("10"),
        )
        .get_matches();

    let files = matches.values_of_lossy("files").unwrap();
    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()?
        .unwrap();
    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()?;
    Ok(Config {
        files,
        lines,
        bytes,
    })
}

// pub fn run(config: Config) -> MyResult<()> {
//     for filename in config.files {
//         match open(&filename) {
//             Err(err) => eprintln!("{}: {}", filename, err),
//             Ok(mut file) => {
//                 println!("==> {} <==", filename);
//                 if let Some(num_bytes) = config.bytes {
//                     let mut handle = file.take(num_bytes as u64);
//                     let mut buffer = vec![0; num_bytes];
//                     let bytes_read = handle.read(&mut buffer)?;
//                     print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
//                 } else {
//                     let mut line = String::new();
//                     for _ in 0..config.lines {
//                         let bytes = file.read_line(&mut line)?;
//                         if bytes == 0 {
//                             break;
//                         }
//                         print!("{}", line);
//                         line.clear();
//                     }
//                 }
//                 println!();
//             }
//         }
//     }
//     Ok(())
// }
pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes as u64);
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
