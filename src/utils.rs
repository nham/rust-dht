//! Various utilities

use std::net::{self, IpAddr};


/// Convert socket address to bytes in network order.
pub fn netaddr_to_netbytes(addr: &net::SocketAddr) -> Vec<u8> {
    match addr.ip {
        IpAddr::V4(a) => {
            let o = a.octets();
            let x = (addr.port >> 8) as u8;
            let y = (addr.port & 0xFF) as u8;
            vec![o[0], o[1], o[2], o[3], x, y]
        },
        // TODO(divius): implement
        IpAddr::V6(_) => panic!("IPv6 not implemented")
    }
}

/// Get socket address from netbytes.
pub fn netaddr_from_netbytes(bytes: &[u8]) -> net::SocketAddr {
    assert_eq!(6, bytes.len());
    net::SocketAddr::V4(
        net::SocketAddrV4::new(
            net::Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]),
            ((bytes[4] as u16) << 8) + bytes[5] as u16
        )
    )
}


#[cfg(test)]
pub mod test {
    use std::net;
    use std::num::FromPrimitive;

    use num;

    use super::super::Node;


    pub static ADDR: &'static str = "127.0.0.1:8008";

    pub fn new_node(id: usize) -> Node {
        new_node_with_port(id, 8008)
    }

    pub fn new_node_with_port(id: usize, port: u16) -> Node {
        Node {
            id: FromPrimitive::from_uint(id).unwrap(),
            address: net::SocketAddr {
                ip: net::Ipv4Addr(127, 0, 0, 1),
                port: port
            }
        }
    }

    pub fn usize_to_id(id: usize) -> num::BigUint {
        FromPrimitive::from_uint(id).unwrap()
    }
}
