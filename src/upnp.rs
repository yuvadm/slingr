extern crate libc;

use self::libc::{uint32_t, uint16_t, uint8_t, c_char, c_uchar, c_int, c_void};
use std::ptr;

pub type GSSDPClient = *mut c_void;

#[link(name = "gssdp-1.0")]
extern {
    pub fn gssdp_client_new(main_context: *mut c_void, iface: *const c_char, error: *mut c_void) -> GSSDPClient;
    pub fn gssdp_client_get_interface(client: *mut GSSDPClient) -> *const c_char;
}

pub fn discover() {
    println!("Discovering...");
    unsafe {
        let mut client = gssdp_client_new(ptr::null_mut(), ptr::null(), ptr::null_mut());
        let iface = gssdp_client_get_interface(&client);
    }
    println!("Using interface: {}", iface);
}
