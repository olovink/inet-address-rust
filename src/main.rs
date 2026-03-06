#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct InetAddress {
    address: [u8; 4],
    port: u16,
    version: u8
}

impl InetAddress {
    pub const fn new(address: [u8; 4], port: u16, version: u8) -> InetAddress {
        InetAddress {address, port, version}
    }

    pub fn get_string_address(&self) -> String {
        format!("{}.{}.{}.{}", self.address[0], self.address[1], self.address[2], self.address[3])
    }

    pub const LOCALHOST: Self = InetAddress::new([127, 0, 0, 1], 0, 4);

    pub fn equals(&self, other: &InetAddress) -> bool {
        self.get_string_address() == other.get_string_address() && self.port == other.port && self.version == other.version
    }

    pub fn get_as_ipv4addr(&self) -> Ipv4Addr {
        Ipv4Addr::new(self.address[0], self.address[1], self.address[2], self.address[3])
    }
}

fn main() {
    let inet_address = InetAddress::LOCALHOST;

    println!("{:?}", inet_address);
    println!("{}", InetAddress::new([127, 0, 0, 1], 0, 4).get_string_address());
}
