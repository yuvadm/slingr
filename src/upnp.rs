extern crate libc;

use self::libc::{uint32_t, uint16_t, uint8_t, c_char, c_uchar, c_int, c_void};
use std::ptr;

#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
enum EventType {
    ControlActionRequest,
    ControlActionComplete,
    ControlGetVarRequest,
    ControlGetVarComplete,
    DiscoveryAdvertisementAlive,
    DiscoveryAdvertisementByebye,
    DiscoverySearchResult,
    DiscoverySearchTimeout,
    SubscriptionRequest,
    Received,
    RenewalComplete,
    SubscribeComplete,
    UnsubscribeComplete,
    AutorenewalFailed,
    SubscriptionExpired,
}

const LINE_SIZE: usize = 180;

#[repr(C)]
struct Discovery {
    err_code: libc::c_int,
    expires: libc::c_int,
    device_id: [libc::c_char; LINE_SIZE],
    device_type: [libc::c_char; LINE_SIZE],
    service_type: [libc::c_char; LINE_SIZE],
    service_ver: [libc::c_char; LINE_SIZE],
    location: [libc::c_char; LINE_SIZE],
    os: [libc::c_char; LINE_SIZE],
    date: [libc::c_char; LINE_SIZE],
    ext: [libc::c_char; LINE_SIZE],
    dest_addr: *mut libc::sockaddr_in,
}

type ClientHandle = libc::c_int;

type ClientCallbackPtr = extern "C" fn(event_type: EventType,
                                       event: *const libc::c_void,
                                       cookie: *mut libc::c_void);

#[link(name = "upnp")]
extern "C" {
    fn UpnpInit(hostIp: *const libc::c_char, destPort: libc::c_ushort) -> libc::c_int;
    fn UpnpRegisterClient(callback: ClientCallbackPtr,
                          cookie: *mut libc::c_void,
                          handle: *mut ClientHandle)
                          -> libc::c_int;
    fn UpnpUnRegisterClient(handle: ClientHandle) -> libc::c_int;
    fn UpnpSearchAsync(handle: ClientHandle,
                       maxAttempts: libc::c_int,
                       target: *const libc::c_char,
                       cookie: *const libc::c_void)
                       -> libc::c_int;
}

pub fn discover() {
    println!("Discovering...");
    unsafe {
        let err = UpnpInit(ptr::null(), 0);
        let target = CString::new("ssdp:all");
        UpnpSearchAsync(self.handle.client, 1, target.as_ptr(), cookie);
    }
}
