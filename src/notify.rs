const SOAP_ACTION_PREFIX: &'static str = "urn:schemas-upnp-org:service:AVTransport:1#";
const ACTION_SET_URI: &'static str = "SetAVTransportURI";
const ACTION_PLAY: &'static str = "Play";
const ACTION_PAUSE: &'static str = "Pause";
const ACTION_STOP: &'static str = "Stop";

pub const BODY_SET_URI: &'static str = r#"<?xml version='1.0' encoding='utf-8'?>
<s:Envelope s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/" xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
    <s:Body>
        <u:SetAVTransportURI xmlns:u="urn:schemas-upnp-org:service:AVTransport:1">
            <InstanceID>0</InstanceID>
            <CurrentURI>http://10.5.1.243:51497/0</CurrentURI>
            <CurrentURIMetaData>&lt;DIDL-Lite xmlns="urn:schemas-upnp-org:metadata-1-0/DIDL-Lite/" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:sec="http://www.sec.co.kr/" xmlns:upnp="urn:schemas-upnp-org:metadata-1-0/upnp/"&gt;&lt;item id="0" parentID="-1" restricted="false"&gt;&lt;upnp:class&gt;object.item.videoItem.movie&lt;/upnp:class&gt;&lt;dc:title&gt;dlnanow&lt;/dc:title&gt;&lt;res protocolInfo="http-get:*:video/mp4:*"&gt;http://10.5.1.243:51497/0&lt;/res&gt;&lt;/item&gt;&lt;/DIDL-Lite&gt;</CurrentURIMetaData>
        </u:SetAVTransportURI>
    </s:Body>
</s:Envelope>"#;

pub const BODY_PLAY: &'static str = r#"<?xml version='1.0' encoding='utf-8'?>
<s:Envelope s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/" xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
    <s:Body>
        <u:Play xmlns:u="urn:schemas-upnp-org:service:AVTransport:1">
            <InstanceID>0</InstanceID>
            <Speed>1</Speed>
        </u:Play>
    </s:Body>
</s:Envelope>"#;

pub const BODY_STOP: &'static str = r#"<?xml version='1.0' encoding='utf-8'?>
<s:Envelope s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/" xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
    <s:Body>
        <u:Stop xmlns:u="urn:schemas-upnp-org:service:AVTransport:1">
            <InstanceID>0</InstanceID>
        </u:Stop>
    </s:Body>
</s:Envelope>"#;


pub const A_SET: &'static str = "urn:schemas-upnp-org:service:AVTransport:1#SetAVTransportURI";
pub const A_PLAY: &'static str = "urn:schemas-upnp-org:service:AVTransport:1#Play";
pub const A_STOP: &'static str = "urn:schemas-upnp-org:service:AVTransport:1#Stop";
