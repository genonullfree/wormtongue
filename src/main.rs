use std::net::UdpSocket;
use std::io::{self, Write};

static ASCIICODES: [&str;32] = [
    "NUL",
    "SOH",
    "STX",
    "ETX",
    "EOT",
    "ENQ",
    "ACK",
    "BEL",
    "BS",
    "HT",
    "LF",
    "VT",
    "FF",
    "CR",
    "SO",
    "SI",
    "DLE",
    "DC1",
    "DC2",
    "DC3",
    "DC4",
    "NAK",
    "SYN",
    "ETB",
    "CAN",
    "EM",
    "SUB",
    "ESC",
    "FS",
    "GS",
    "RS",
    "US"
];

fn print_char(uc: &u8) {

        let c: char = *uc as char;

        print!("{}", c);
        io::stdout().flush();
}

fn main() {
    loop {

        let sock = UdpSocket::bind("0.0.0.0:1337").expect("Couldn't bind to address");

        let mut buf = [0];
        let size = sock.recv(&mut buf)
                              .expect("Didn't rx anything...");

        if size != 1 {
            println!("Error, received more than 1 byte!");
            continue;
        }

        print_char(&buf[0]);

    };
}
