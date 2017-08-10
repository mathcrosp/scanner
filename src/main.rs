extern crate getopts;

use getopts::Options;
use std::env;
use std::net;
use std::process;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;

const NTHREADS: u16 = 4;
const MAX_PORT: u16 = 65535;

fn handle(tx: mpsc::Sender<u16>, host: net::IpAddr,
          start_port: u16, end_port: u16, nthreads: u16) {
    let mut port = start_port + 1;

    loop {
        match net::TcpStream::connect((host, port)) {
            Ok(_) => {
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if end_port - nthreads <= port {
            break;
        }

        port += nthreads;
    }
}

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

    println!("scanning {} from {} to {}...", &host, start_port, end_port);

    let addr: net::IpAddr;
    match net::IpAddr::from_str(&host) {
        Ok(value) => {
            addr = value;
        }
        Err(_) => {
            addr = net::IpAddr::V4(
                net::Ipv4Addr::new(127, 0, 0, 1)
            );
        }
    };

    let (tx, rx) = mpsc::channel();
    for i in 0..nthreads {
        let tx = tx.clone();

        thread::spawn(move || {
            handle(tx, addr, (start_port + i) as u16, end_port, nthreads);
        });
    }

    let mut output = vec![];
    drop(tx);
    for port in rx {
        output.push(port);
    }
    output.sort();

    println!("");

    for port in output {
        println!("{} open", port);
    }
}
