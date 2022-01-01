use std::{net::{UdpSocket, SocketAddr}, sync::mpsc::Sender};

pub struct Client {
    client_address: SocketAddr,
    server_address: SocketAddr,
    socket: UdpSocket,
    sender: Sender<(SocketAddr, Vec<u8>)>
}

impl Client {
    pub fn new(client_address: SocketAddr, server_address: SocketAddr, sender: Sender<(SocketAddr, Vec<u8>)>) -> Self { 
        let mut socket = UdpSocket::bind("0.0.0.0:10000");
        let mut i: u16 = 10000;
        while socket.is_err() && i < 65535 {
            i += 1;
            socket = UdpSocket::bind(format!("0.0.0.0:{}", i));
        }
        let socket: UdpSocket = socket.unwrap();
        socket.set_nonblocking(true).unwrap();
        Self {
            client_address,
            server_address,
            socket,
            sender
        }  
    }

    pub fn listen(&self) {
        let mut v: [u8; 1500] = [0u8; 1500];
        let recv = self.socket.recv_from(&mut v);
        if recv.is_err() {
            return;
        }
        let (size, address) = recv.unwrap();
        let buffer = &v[..size];
        self.handle_packet(address, buffer.to_vec());
    }

    pub fn handle_packet(&self, address: SocketAddr, buffer: Vec<u8>) {
        if address.eq(&self.server_address) {
            self.sender.send((self.client_address, buffer)).unwrap();
            return;
        }
        self.socket.send_to(&buffer, self.server_address).unwrap();
    }
}