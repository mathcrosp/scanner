use std::env;
use std::net;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;

const NTHREADS: u16 = 4;
const MAX_PORT: u16 = 65535;

fn handle(tx: mpsc::Sender<u16>, host: net::IpAddr, start_port: u16, end_port: u16) {
    let mut port = start_port + 1;

    loop {
        match net::TcpStream::connect((host, port)) {
            Ok(_) => {
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if end_port - NTHREADS <= port {
            break;
        }

        port += NTHREADS;
    }
}

fn main() {
    let mut args = env::args();
    let filename = args.next();

    let input = args.next()
        .unwrap_or("127.0.0.1".to_owned());

    println!("scanning {}...", input);

    let mut host: net::IpAddr = net::IpAddr::V4(
        net::Ipv4Addr::new(127, 0, 0, 1)
    );

    match net::IpAddr::from_str(&input) {
        Ok(value) => {
            host = value;
        }
        Err(_) => {}
    };

    let start_port: u16 = 1;
    let end_port: u16 = MAX_PORT;

    let (tx, rx) = mpsc::channel();
    for i in 0..NTHREADS {
        let tx = tx.clone();

        thread::spawn(move || {
            handle(tx, host, (start_port + i) as u16, end_port);
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
