use std::net;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread;

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

pub fn portscan(host: String, start_port: u16, end_port: u16, nthreads: u16) {
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
