use std::net::Ipv4Addr;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct InetAddress {
    address: [u8; 4],
    port: u16,
}

impl InetAddress {
    pub const fn new(address: [u8; 4], port: u16) -> InetAddress {
        InetAddress {address, port}
    }

    pub fn get_string_address(&self) -> String {
        format!("{}.{}.{}.{}", self.address[0], self.address[1], self.address[2], self.address[3])
    }

    pub fn from_bytes(bytes: [u8; 4]) -> InetAddress {
        InetAddress { address: bytes, port: 0 }
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn as_bytes(&self) -> &[u8; 4] {
        &self.address
    }

    pub const LOCALHOST: Self = InetAddress::new([127, 0, 0, 1], 0);
    
    pub const BROADCAST: Self = InetAddress::new([255, 255, 255, 255], 0);

    pub fn equals(&self, other: &InetAddress) -> bool {
        self.get_string_address() == other.get_string_address() && self.port == other.port
    }

    pub fn get_as_ipv4addr(&self) -> Ipv4Addr {
        Ipv4Addr::new(self.address[0], self.address[1], self.address[2], self.address[3])
    }
}

fn main() {
    let inet_address = InetAddress::LOCALHOST;

    println!("{:?}", inet_address);
    println!("{}", InetAddress::new([127, 0, 0, 1], 0).get_string_address());
}
