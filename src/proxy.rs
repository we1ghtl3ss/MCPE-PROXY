use std::{net::{SocketAddr, UdpSocket}, collections::HashMap, sync::{mpsc::{Sender, Receiver, self}, Arc, Mutex, MutexGuard}};

use crate::client::Client;

pub struct Proxy {
    server_address: SocketAddr,
    socket: UdpSocket,
    clients: HashMap<SocketAddr, Arc<Mutex<Client>>>,
    sender: Sender<(SocketAddr, Vec<u8>)>,
    receiver: Receiver<(SocketAddr, Vec<u8>)>
}

impl Proxy {
    pub fn new(server_address: SocketAddr) -> Self {
        let (sender, receiver) : (Sender<(SocketAddr, Vec<u8>)>, Receiver<(SocketAddr, Vec<u8>)>) = mpsc::channel();
        let socket = UdpSocket::bind("0.0.0.0:19132").unwrap();
        socket.set_nonblocking(true).unwrap();
        Self {
            server_address,
            socket,
            clients: HashMap::new(),
            sender,
            receiver
        }
    }

    pub fn listen(&mut self) {
        loop {
            for client in self.clients.values() {
                {

                    let client: Arc<Mutex<Client>> = Arc::clone(client);
                    std::thread::spawn(move || {
                        let client: MutexGuard<Client> = client.lock().unwrap();
                        client.listen();
                    });
                }
            }
            let recv = self.receiver.try_recv();
            if !recv.is_err() {
                let (address, buffer) : (SocketAddr, Vec<u8>) = recv.unwrap();
                self.socket.send_to(&buffer, address).unwrap();
            }
            let mut v: [u8; 1500] = [0u8; 1500];
            let recv = self.socket.recv_from(&mut v);
            if recv.is_err() {
                continue;
            }
            let (size, address) = recv.unwrap();
            let buffer: &[u8] = &v[..size];
            if !self.clients.contains_key(&address) {
                self.clients.insert(address, Arc::new(Mutex::new(Client::new(address, self.server_address, self.sender.clone()))));
            }
            self.clients.get(&address).unwrap().lock().unwrap().handle_packet(address, buffer.to_vec());
        }
    }
}