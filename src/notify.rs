extern crate hyper;
extern crate futures;

use self::hyper::{Method, Request, Uri};
use self::hyper::header::HeaderValue;
use self::hyper::body::Body;

const SOAP_ACTION_PREFIX: &'static str = "urn:schemas-upnp-org:service:AVTransport:1#";
const ACTION_SET_URI: &'static str = "SetAVTransportURI";
const ACTION_PLAY: &'static str = "Play";
const ACTION_PAUSE: &'static str = "Pause";
const ACTION_STOP: &'static str = "Stop";

const BODY_SET_URI: &'static str = r#"<?xml version='1.0' encoding='utf-8'?>
<s:Envelope s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/" xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
    <s:Body>
        <u:SetAVTransportURI xmlns:u="urn:schemas-upnp-org:service:AVTransport:1">
            <InstanceID>0</InstanceID>
            <CurrentURI>http://10.5.1.243:51497/0</CurrentURI>
            <CurrentURIMetaData>&lt;DIDL-Lite xmlns="urn:schemas-upnp-org:metadata-1-0/DIDL-Lite/" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:sec="http://www.sec.co.kr/" xmlns:upnp="urn:schemas-upnp-org:metadata-1-0/upnp/"&gt;&lt;item id="0" parentID="-1" restricted="false"&gt;&lt;upnp:class&gt;object.item.videoItem.movie&lt;/upnp:class&gt;&lt;dc:title&gt;dlnanow&lt;/dc:title&gt;&lt;res protocolInfo="http-get:*:video/mp4:*"&gt;http://10.5.1.243:51497/0&lt;/res&gt;&lt;/item&gt;&lt;/DIDL-Lite&gt;</CurrentURIMetaData>
        </u:SetAVTransportURI>
    </s:Body>
</s:Envelope>"#;

const BODY_PLAY: &'static str = r#"
<?xml version='1.0' encoding='utf-8'?>
<s:Envelope s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/" xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
    <s:Body>
        <u:Play xmlns:u="urn:schemas-upnp-org:service:AVTransport:1">
            <InstanceID>0</InstanceID>
            <Speed>1</Speed>
        </u:Play>
    </s:Body>
</s:Envelope>"#;

const BODY_STOP: &'static str = r#"
<?xml version='1.0' encoding='utf-8'?>
<s:Envelope s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/" xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
    <s:Body>
        <u:Stop xmlns:u="urn:schemas-upnp-org:service:AVTransport:1">
            <InstanceID>0</InstanceID>
        </u:Stop>
    </s:Body>
</s:Envelope>"#;


pub fn build_request(body: &'static str, action: &str) {
    let uri: Uri = "http://10.5.1.201:38400/serviceControl/AVTransport".parse().unwrap();
    let mut req = Request::new(Body::from(body));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = uri.clone();
    req.headers_mut().insert("content-type", HeaderValue::from_str("text/xml").unwrap());
    req.headers_mut().insert("SOAPACTION", HeaderValue::from_str(action).unwrap());
}

pub fn set_uri() {
    build_request(BODY_SET_URI, "urn:schemas-upnp-org:service:AVTransport:1#SetAVTransportURI")
}

pub fn play() {
    build_request(BODY_PLAY, "urn:schemas-upnp-org:service:AVTransport:1#Play")
}
