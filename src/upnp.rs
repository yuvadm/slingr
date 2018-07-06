use std::net::UdpSocket;

const SEARCH_REQUEST: &'static str = "M-SEARCH * HTTP/1.1\r\n
Host: 239.255.255.250:1900\r\n
Man: \"ssdp:discover\"\r\n
ST: ssdp:all\r\n
MX: 3\r\n
User-Agent: rustcast/0.1.0\r\n\r\n";

pub fn discover() {
    println!("Discovering...");
    let socket = UdpSocket::bind("[::]:0").unwrap();
    socket.connect("239.255.255.250:1900").unwrap();
    socket.send_to(SEARCH_REQUEST.as_bytes(), "239.255.255.250:1900").expect("couldn't send data");
}
