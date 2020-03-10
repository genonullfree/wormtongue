use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    loop {

    let mut sock = UdpSocket::bind("0.0.0.0:1337").expect("Couldn't bind to address");

    let mut buf = [0;32];
    let (size, src) = sock.recv_from(&mut buf)
                          .expect("Didn't rx anything...");

    let buf = &mut buf[..size];

    for i in 0..size-1 {
        print!("{:02x} ", buf[i]);
    }

    if buf[0] == 0 {
        break;
    }
    };
    Ok(())
}
