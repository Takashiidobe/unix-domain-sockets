use std::os::unix::net::UnixDatagram;

fn main() -> std::io::Result<()> {
    let socket = UnixDatagram::bind("/tmp/socket.udp")?;
    loop {
        let mut buf = [0; 1024];
        let (count, address) = socket.recv_from(&mut buf)?;
        let msg: Vec<_> = buf.into_iter().take_while(|x| *x != 0).collect();
        let str_msg = String::from_utf8_lossy(&msg);
        println!(
            "socket {:?} sent {} with a length of {}",
            address, str_msg, count
        );
    }
}
