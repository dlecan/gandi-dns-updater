use error::IpError;
use ip::IpAddr;
use std::io;
use std::io::prelude::*;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub trait MyIPAddressProvider<'a> {
    fn get_my_ip_addr(&self) -> Result<IpAddr, IpError>;
}

pub struct StdinIpProvider;

impl<'a> MyIPAddressProvider<'a> for StdinIpProvider {
    fn get_my_ip_addr(&self) -> Result<IpAddr, IpError> {
        let mut input = String::new();

        try!(io::stdin().read_line(&mut input));

        trace!("Read stdin: {}", input);

        let ip_addr = try!(IpAddr::from_str(input.trim()));
        Ok(ip_addr)
    }
}
