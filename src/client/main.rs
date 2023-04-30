use std::os::unix::net::UnixDatagram;

fn main() -> std::io::Result<()> {
    let socket = UnixDatagram::bind("/tmp/socket-client.udp")?;
    socket.send_to(b"hello world", "/tmp/socket.udp")?;
    Ok(())
}
