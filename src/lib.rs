use std::{
    net::{Ipv4Addr, TcpListener, TcpStream, UdpSocket},
    str::FromStr,
};

pub fn udp_is_available(host: &str, port: u16) -> bool {
    UdpSocket::bind((Ipv4Addr::from_str(host).unwrap(), port)).is_ok()
}

pub fn tcp_is_available(host: &str, port: u16) -> bool {
    TcpListener::bind((Ipv4Addr::from_str(host).unwrap(), port)).is_ok()
}

pub fn tcp_can_connect(host: &str, port: u16) -> bool {
    TcpStream::connect((host, port)).is_ok()
}

pub fn port_is_available(host: &str, port: u16) -> bool {
    udp_is_available(host, port) && tcp_is_available(host, port) && !tcp_can_connect(host, port)
}

pub fn find_port(host: &str, port: u16) -> Option<u16> {
    for i in port..u16::MAX {
        if port_is_available(host, i) {
            return Some(i);
        }
    }
    return None;
}
