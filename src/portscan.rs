use std::net;
use std::sync::mpsc;
use std::thread;

fn handle(tx: mpsc::Sender<u16>, addr: net::IpAddr,
          start_port: u16, end_port: u16, nthreads: u16) {
    let mut port = start_port + 1;

    loop {
        match net::TcpStream::connect((addr, port)) {
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

pub fn portscan(addr: net::IpAddr, start_port: u16, end_port: u16, nthreads: u16) {
    println!("scanning {} from {} to {}...", &addr, start_port, end_port);

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
        println!("{}: open", port);
    }
}
