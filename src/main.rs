extern crate getopts;

mod portscan;

use getopts::Options;
use std::env;
use std::net;
use std::str::FromStr;

const NTHREADS: u16 = 4;
const MAX_PORT: u16 = 65535;

fn show_brief(opts: Options, program: &str) {
    let brief = format!("Usage: {} HOST [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this message");
    opts.optopt("p", "ports", "Range of ports to scan", "<start>:<end>");
    opts.optopt("j", "threads", "Number of threads to run", "<number of threads>");

    let args: Vec<String> = env::args().collect();
    let ref program = args[0];
    if args.len() < 2 {
        show_brief(opts, program);
        return;
    }
    let ref host = args[1];

    let matches = match opts.parse(&args[2..]) {
        Ok(m) => m,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    if matches.opt_present("h") {
        show_brief(opts, program);
        return;
    }

    let addr: net::IpAddr;
    match net::IpAddr::from_str(&host) {
        Ok(value) => {
            addr = value;
        }
        Err(_) => {
            println!("{} is not a valid IP address.", host);
            show_brief(opts, program);
            return;
        }
    };

    let ports = match matches.opt_str("ports") {
        Some(value) => value,
        None => "1".to_owned() + &(":".to_owned()) + &(MAX_PORT.to_string()),
    };

    let mut split = ports.split(':');
    let start_port = split.next().unwrap().parse().unwrap();
    let end_port = split.next().unwrap().parse().unwrap();

    if start_port > end_port {
        println!("Error: start port should be less than end port");
        return;
    }

    let nthreads = match matches.opt_str("threads") {
        Some(value) => value.parse().expect("number of threads must be an \
                                            integer"),
        None => NTHREADS,
    };

    portscan::portscan(addr, start_port, end_port, nthreads);
}
