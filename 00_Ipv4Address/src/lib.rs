
///! # Ipv4 Address
///!
///! A simple lib to calculate information about Ipv4 Addresses.
///!
#[derive(Default, Debug)]
pub struct Ipv4Addr {
    addr: u32,
    mask: u32,
}


fn parse_ipv4_address(input: String) -> u32 {
    let mut ip : u32 = 0;
    let mut current : u32 = 0;
    for c in input.chars() {
        if c.is_digit(10) {
            current *= 10;
            current += c.to_digit(10).unwrap();
        } else if c == '.' {
            ip *= 256;
            ip += current;
            current = 0;
        }
    }

    ip *= 256;
    ip += current;

    return ip;
}

fn stringify_ipv4_address(input: u32) -> String {
    let a = (input & (255 << 24)) >> 24;
    let b = (input & (255 << 16)) >> 16;
    let c = (input & (255 << 8)) >> 8;
    let d = input & 255;

    return format!("{}.{}.{}.{}", a, b, c, d);
}

impl Ipv4Addr {
    /// # Example
    /// ```
    /// use Ipv4Address::Ipv4Addr;
    /// let a = Ipv4Addr::new(
    ///     Ipv4Addr::to_u32("127.0.0.1".into()),
    ///     Ipv4Addr::to_u32("255.255.255.0".into())
    /// );
    /// ```
    pub fn new(addr: u32, mask: u32) -> Self {
        Self {
            addr: addr,
            mask: mask,
        }
    }

    /// Convert a string dot notation ip address to u32.
    pub fn to_u32(input: String) -> u32 {
        parse_ipv4_address(input)
    }

    /// Convert u32 ip address to dot notation string.
    pub fn to_string(input: u32) -> String {
        stringify_ipv4_address(input)
    }

    /// Get the current network ip adress.
    pub fn network(&self) -> u32 {
        self.addr & self.mask
    }

    /// Get the broadcast address for the current network.
    pub fn broadcast(&self) -> u32 {
        self.network() | !self.mask
    }

    /// Get the first available ip address in current network.
    pub fn first(&self) -> u32 {
        self.network() + 1
    }

    /// Get the last available ip address in current network.
    pub fn last(&self) -> u32 {
        self.network() | (!self.mask) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ipv4_address() {
        assert_eq!(parse_ipv4_address("127.0.0.1".into()), 2130706433);
        assert_eq!(parse_ipv4_address("192.0.2.235".into()), 3221226219);
        assert_eq!(parse_ipv4_address("159.12.35.156".into()), 2668372892);
    }

    #[test]
    fn test_stringify_ipv4_address() {
        assert_eq!(stringify_ipv4_address(2130706433), "127.0.0.1");
        assert_eq!(stringify_ipv4_address(3221226219), "192.0.2.235");
        assert_eq!(stringify_ipv4_address(2668372892), "159.12.35.156");
    }

    #[test]
    fn test_to_u32() {
        assert_eq!(Ipv4Addr::to_u32("127.0.0.1".into()), 2130706433);
        assert_eq!(Ipv4Addr::to_u32("192.0.2.235".into()), 3221226219);
        assert_eq!(Ipv4Addr::to_u32("159.12.35.156".into()), 2668372892);
    }


    #[test]
    fn test_struct() {
        let _ = Ipv4Addr { addr: 0, mask: 0, };
    }

    #[test]
    fn test_new() {
        let _ = Ipv4Addr::new(0, 0);
    }

    #[test]
    fn test_network_1() {
        let a = Ipv4Addr::new(
            Ipv4Addr::to_u32("127.0.0.1".into()),
            Ipv4Addr::to_u32("255.255.255.0".into()),
        );

        assert_eq!("127.0.0.0", Ipv4Addr::to_string(a.network()));
    }

    #[test]
    fn test_network_2() {
        let a = Ipv4Addr::new(
            Ipv4Addr::to_u32("192.168.56.74".into()),
            Ipv4Addr::to_u32("255.255.255.0".into()),
        );

        assert_eq!("192.168.56.0", Ipv4Addr::to_string(a.network()));
    }

    #[test]
    fn test_boardcast_1() {
        let a = Ipv4Addr::new(
            Ipv4Addr::to_u32("127.0.0.1".into()),
            Ipv4Addr::to_u32("255.0.0.0".into()),
        );

        assert_eq!("127.255.255.255", Ipv4Addr::to_string(a.broadcast()));
    }

    #[test]
    fn test_boardcast_2() {
        let a = Ipv4Addr::new(
            Ipv4Addr::to_u32("192.168.56.76".into()),
            Ipv4Addr::to_u32("255.255.0.0".into()),
        );

        assert_eq!("192.168.255.255", Ipv4Addr::to_string(a.broadcast()));
    }

    #[test]
    fn test_first_1() {
        let a = Ipv4Addr::new(
            Ipv4Addr::to_u32("127.0.0.12".into()),
            Ipv4Addr::to_u32("255.0.0.0".into()),
        );

        assert_eq!("127.0.0.1", Ipv4Addr::to_string(a.first()));
    }

    #[test]
    fn test_last_1() {
        let a = Ipv4Addr::new(
            Ipv4Addr::to_u32("127.0.0.12".into()),
            Ipv4Addr::to_u32("255.0.0.0".into()),
        );

        assert_eq!("127.255.255.254", Ipv4Addr::to_string(a.last()));
    }

    #[test]
    fn test_from_strings() {
        let a = Ipv4Addr::new(
            Ipv4Addr::to_u32("127.0.0.1".into()),
            Ipv4Addr::to_u32("255.255.255.0".into()),
        );

        assert_eq!(a.addr, 2130706433);
        println!("{:?}", a);
        println!("addr {:?}", Ipv4Addr::to_string(a.addr));
        println!("mask {:?}", Ipv4Addr::to_string(a.mask));
        println!("network {:?}", Ipv4Addr::to_string(a.network()));
    }
}
