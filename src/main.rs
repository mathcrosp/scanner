extern crate getopts;

mod portscan;

use getopts::Options;
use std::env;
use std::process;

const NTHREADS: u16 = 4;
const MAX_PORT: u16 = 65535;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ref program = args[0];

    let mut opts = Options::new();
    opts.optflag("h", "help", "Program description");
    opts.optopt("a", "host", "Host to scan ports on", "HOST");
    opts.optopt("e", "eport", "Port to end scanning at", "EPORT");
    opts.optopt("j", "threads", "Number of threads to run", "THREADS");
    opts.optopt("s", "sport", "Port to start scanning from", "SPORT");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} OPTIONS", program);
        print!("{}", opts.usage(&brief));
        return;
    }

    let host = match matches.opt_str("host") {
        Some(value) => value.to_string(),
        None => "127.0.0.1".to_owned(),
    };

    let start_port = match matches.opt_str("sport") {
        Some(value) => value.parse().expect("start port must be an integer"),
        None => 1,
    };

    let end_port = match matches.opt_str("eport") {
        Some(value) => value.parse().expect("end port must be an integer"),
        None => MAX_PORT,
    };

    let nthreads = match matches.opt_str("threads") {
        Some(value) => value.parse().expect("number of threads must be an \
                                            integer"),
        None => NTHREADS,
    };

    if start_port > end_port {
        println!("error: start port should be less than end port");
        process::exit(1);
    }

    portscan::portscan(host, start_port, end_port, nthreads);
}
