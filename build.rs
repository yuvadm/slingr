extern crate pkg_config;

fn main() {
    pkg_config::Config::new().probe("gssdp-1.0").unwrap();
}
