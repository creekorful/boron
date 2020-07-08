use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::str;
use std::str::FromStr;
use std::time::Duration;

pub fn grab_banner(
    address: &str,
    connect_timeout: Duration,
    read_timeout: Duration,
    write_timeout: Duration,
) -> anyhow::Result<String> {
    let address = SocketAddr::from_str(address)?;

    let mut stream = TcpStream::connect_timeout(&address, connect_timeout)?;
    stream.set_read_timeout(Option::from(read_timeout))?;
    stream.set_write_timeout(Option::from(write_timeout))?;

    let mut buffer = [0; 512];

    // Try to read banner right after connecting
    let result = stream.read(&mut buffer);
    if result.is_ok() {
        return Ok(String::from(str::from_utf8(&buffer)?));
    }

    // If timeout related error happens, do not fails
    // because we may need to talk first
    let error = result.err().unwrap();
    if error.kind() != ErrorKind::WouldBlock {
        return Err(anyhow::anyhow!(error));
    }

    // If nothing was returned, send a dummy request
    stream.write("HEAD / HTTP/1.1\n\n".as_ref())?;

    // Try to read again
    stream.read(&mut buffer)?;
    return Ok(String::from(str::from_utf8(&buffer)?));
}
