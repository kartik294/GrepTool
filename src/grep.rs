use regex::Regex;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, StandardStreamLock, WriteColor};

pub struct Grep {}

impl Grep {
    fn highlight(&self, line: &str, regex: &Regex, stdout: &mut StandardStreamLock) {
        let mut last_end = 0;

        for mat in regex.find_iter(line) {
            let (start, end) = (mat.start(), mat.end());

            write!(stdout, "{}", &line[last_end..start]).unwrap();

            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))
                .unwrap();

            write!(stdout, "{}", &line[start..end]).unwrap();

            stdout.reset().unwrap();

            last_end = end;
        }

        write!(stdout, "{}", &line[last_end..]).unwrap();
    }

    fn process(&self, regex: Regex, path: String) {
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);

        let stdout = StandardStream::stdout(ColorChoice::Always);
        let mut stdout = stdout.lock();

        for line in reader.lines() {
            let line = line.unwrap();
            if regex.is_match(&line) {
                self.highlight(&line, &regex, &mut stdout);
                writeln!(stdout).unwrap();
            }
        }
    }

    pub fn run(&self) {
        let stdin = stdin();
        let mut stdout = stdout();
        let mut input = String::new();

        loop {
            print!("CMD> ");
            stdout.flush().unwrap();
            input.clear();
            stdin.read_line(&mut input).unwrap();

            let args: Vec<&str> = input.trim().split_ascii_whitespace().collect();

            if args.is_empty() {
                continue;
            }

            if args[0] == "exit" {
                break;
            }

            if args.len() < 3 {
                eprintln!("Usage: mygrep <pattern> <file.extension>");
                continue;
            }

            let pattern = args[1];
            let regex = match Regex::new(pattern) {
                Ok(re) => re,
                Err(e) => {
                    eprintln!("Invalid pattern: {}", e);
                    continue;
                }
            };

            self.process(regex, args[2].to_string());
        }
    }
}
