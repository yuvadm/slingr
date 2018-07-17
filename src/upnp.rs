extern crate hyper;
extern crate regex;

use self::regex::Regex;
use self::hyper::Client;
use self::hyper::rt::{self, Future, Stream};
use std::collections::HashMap;
use std::net::UdpSocket;
use std::str;
use std::io::{self, Write};



const MULTICAST_ADDRESS: &'static str = "239.255.255.250:1900";
const SEARCH_REQUEST: &'static str = "M-SEARCH * HTTP/1.1\r\n
Host: 239.255.255.250:1900\r\n
Man: \"ssdp:discover\"\r\n
ST: ssdp:all\r\n
MX: 3\r\n
User-Agent: slingr/0.1.0\r\n\r\n";

// enum Capabilities {
//     RENDERING_CONTROL,
//     AV_TRANSPORT,
//     CONNECTION_MANAGER
// }

#[derive(Debug)]
pub struct Device {
    usn: String,
    xml: String,
}

const USN_REGEX: &'static str = r"uuid:([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})";
const XML_REGEX: &'static str = r"Location: (http(s?)://[\w\.:/]*)";

fn parse_udp_message(msg: &str) -> Device {
    let usn_re = Regex::new(USN_REGEX).unwrap();
    let xml_re = Regex::new(XML_REGEX).unwrap();
    let usn = usn_re.captures(msg).unwrap().get(1).unwrap().as_str().to_string();
    let xml = xml_re.captures(msg).unwrap().get(1).unwrap().as_str().to_string();
    Device {
        usn,
        xml
    }
}

fn fetch_url(url: hyper::Uri) -> impl Future<Item=(), Error=()> {
    let client = Client::new();

    client
        .get(url)
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: {:#?}", res.headers());
            res.into_body().for_each(|chunk| {
                io::stdout().write_all(&chunk)
                    .map_err(|e| panic!("example expects stdout is open, error={}", e))
            })
        })
        .map(|_| {
            println!("\n\nDone.");
        })
        .map_err(|err| {
            eprintln!("Error {}", err);
        })
}

pub fn discover() {
    println!("Discovering...");
    let socket = UdpSocket::bind("[::]:0").unwrap();
    socket.send_to(SEARCH_REQUEST.as_bytes(), MULTICAST_ADDRESS).expect("couldn't send data");

    let mut devices = HashMap::new();
    let d = Device {usn: "a".to_string(), xml: "c".to_string()};
    devices.insert("foo", d);

    let mut buf = [0; 1024];
    loop {
        let (amt, _src) = socket.recv_from(&mut buf).expect("Didn't receive data...");
        let filled_buf = &mut buf[..amt];
        let s = str::from_utf8(filled_buf).unwrap();
        let d = parse_udp_message(s);
        println!("Got: {:?}\n\n", d);
        rt::run(fetch_url(d.xml.parse::<hyper::Uri>().unwrap()));
        break;
    }
}

#[cfg(test)]
mod tests {
    use upnp::*;

    #[test]
    fn test_parsing() {
        let msg = indoc!(r#"
            HTTP/1.1 200 OK
            Content-Type: text/xml; charset="utf-8"
            SERVER: Linux/3.10.0 eHomeMediaCenter/1.0
            Content-Length: 0
            Cache-Control: max-age=1810
            EXT: 
            Date: Fri, 02 Jan 1970 22:25:35 GMT
            ST: urn:schemas-upnp-org:service:ConnectionManager:1
            USN: uuid:9acb38e1-09cc-dba0-ffff-ffffe156cab7::urn:schemas-upnp-org:service:ConnectionManager:1
            Location: http://192.168.33.44:38400/deviceDescription/MediaRenderer
            Foo: foo"#);
        let d = parse_udp_message(msg);
        assert_eq!(d.usn, "9acb38e1-09cc-dba0-ffff-ffffe156cab7");
        assert_eq!(d.xml, "http://192.168.33.44:38400/deviceDescription/MediaRenderer");
    }
}
