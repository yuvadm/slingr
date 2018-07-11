use std::net::UdpSocket;
use std::str;

const MULTICAST_ADDRESS: &'static str = "239.255.255.250:1900";
const SEARCH_REQUEST: &'static str = "M-SEARCH * HTTP/1.1\r\n
Host: 239.255.255.250:1900\r\n
Man: \"ssdp:discover\"\r\n
ST: ssdp:all\r\n
MX: 3\r\n
User-Agent: slingr/0.1.0\r\n\r\n";

pub fn discover() {
    println!("Discovering...");
    let socket = UdpSocket::bind("[::]:0").unwrap();
    socket.send_to(SEARCH_REQUEST.as_bytes(), MULTICAST_ADDRESS).expect("couldn't send data");

    let mut buf = [0; 1024];
    loop {
        let (amt, _src) = socket.recv_from(&mut buf).expect("Didn't receive data...");
        let filled_buf = &mut buf[..amt];
        let s = str::from_utf8(filled_buf).unwrap();
        println!("Got: {}\n\n", s);
    }
}
