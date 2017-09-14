Scanner
=======
A primitive portscanner initially written to try programming in Rust.

How does it work
------------------
Scanner tries to connect with `std::net::TcpStream::connect()` to every port
from the range specified by user. On success it sends current port number
to the channel. When the scanning is done, all ports from the channel are
printed to output and marked as open.

How to build
--------------
The Rust toolchain is required to build this program. Since you've
got rustc and Cargo installed, just run:
```
$ git clone https://github.com/mathcrosp/scanner
$ cd scanner
$ cargo build
```

Usage
-------
The only mandatory command line argument is host's IP address.

Using optional command line arguments you can specify range of ports to scan or
count of threads to run.

License
---------
[BSD 2-Clause License](https://opensource.org/licenses/BSD-2-Clause) © 2017 Dmitry Zubarev
