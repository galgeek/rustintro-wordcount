extern crate getopts;
use getopts::Options;
use std::env;
use std::io;
use std::io::Read;
use std::fs::File;

fn read_file(fname: &str) -> Result<String, io::Error> {
    let mut f = try!(File::open(fname));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

fn count_chars(fname: &str) -> io::Result<usize> {
    println!("counting chars...");
    let chars = try!(read_file(fname));
    Ok(chars.len())
}

fn count_lines(s: &str) {
    println!("counting lines...");
}

fn count_words(s: &str) {
    println!("counting words...");
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("c", "", "count characters");
    opts.optflag("l", "", "count lines");
    opts.optflag("w", "", "count words");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let output = matches.opt_str("o");
    // } else {
        // io::stdout()
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
        // io::stdin()
    };

    if matches.opt_present("w") {
        count_words(&input)
    }
    if matches.opt_present("c") {
        match count_chars(&input) {
            Ok(c) => println!("{:?} characters", c),
            Err(e) => println!("error counting characters: {:?}", e),
        }
    }
    if matches.opt_present("l") {
        count_lines(&input)
    }
}
