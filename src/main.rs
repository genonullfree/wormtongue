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

    if *uc >= 0x20 && *uc < 0x7f {
        print!("{}", c);
    } else if *uc < 0x20 {
        if *uc == 0x01 {
            println!();
        } else {
            print!("<{}>", ASCIICODES[*uc as usize]);
        }
    } else if *uc == 0x7f {
        print!("<DEL>");
    } else if *uc > 0x7f && *uc <= 0xff {
        print!("<{:02x}>", uc);
    }

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
