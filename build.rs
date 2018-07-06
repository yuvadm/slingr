extern crate pkg_config;

fn main() {
    pkg_config::Config::new().atleast_version("1.0.0").probe("gssdp-1.0").unwrap();
}
