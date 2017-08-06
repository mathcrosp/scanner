use std::env;
use std::net;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;

fn handle(tx: mpsc::Sender<u16>, host: net::IpAddr, port: u16) {
    match net::TcpStream::connect((host, port)) {
        Ok(_) => {
            tx.send(port).unwrap();
        }
        Err(_) => {}
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

    let (tx, rx) = mpsc::channel();
    for port in 1..15000 {
        let tx = tx.clone();

        thread::spawn(move || {
            handle(tx, host, port as u16)
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
