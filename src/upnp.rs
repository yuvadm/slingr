extern crate libc;

use self::libc::{uint32_t, uint16_t, uint8_t, c_char, c_uchar, c_int, c_void};

pub type GSSDPClient = *mut c_void;

#[link(name = "gssdp-1.0")]
extern {
    pub fn gssdp_client_new(main_context: *mut c_void, iface: *const c_char, error: *mut c_void) -> GSSDPClient;
}

pub fn discover() {
    println!("Discovering...");
}
