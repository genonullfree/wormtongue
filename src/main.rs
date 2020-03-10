use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    loop {

        let sock = UdpSocket::bind("0.0.0.0:1337").expect("Couldn't bind to address");

        let mut buf = [0];
        let (size, _src) = sock.recv_from(&mut buf)
                              .expect("Didn't rx anything...");

        let c: char = buf[0] as char;

        print!("{} ", c);

        if c == '\0' {
            break;
        }
    };
    Ok(())
}
