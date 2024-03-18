use std::{self, thread};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use dns_parser::{self, Packet, Question, Builder};

struct RootSevers<'a>{
    root_servers: Vec<&'a str>
}

impl <'a> RootSevers<'a>{
    pub fn new() -> Self{
        let mut root_servers = Vec::<&'a str>::new();
        root_servers.push("198.41.0.4");
        root_servers.push("199.9.14.201");
        root_servers.push("192.33.4.12");
        root_servers.push("199.7.91.13");
        root_servers.push("192.203.230.10");
        root_servers.push("192.5.5.241");
        root_servers.push("192.112.36.4");
        root_servers.push("198.97.190.53");
        root_servers.push("192.36.148.17");
        root_servers.push("192.58.128.30");
        root_servers.push("193.0.14.129");
        root_servers.push("199.7.83.42");
        root_servers.push("202.12.27.33");
        RootSevers { root_servers }
    }
}

fn main(){

    let socket = UdpSocket::bind("localhost:6969").unwrap();

    loop {
        let socket = socket.try_clone().unwrap();
        let mut buf = [0; 512];
        let (n, address) = socket.recv_from(&mut buf).unwrap();
        thread::spawn(move  || {
            let read_bytes = &buf[..n];
            resolve_dns(socket, &read_bytes, address);
        });
    }
    
}

fn resolve_dns(conn: UdpSocket, buffer: &[u8], sock_addr : SocketAddr){

    let root_servers = RootSevers::new();

    let packet = Packet::parse(&buffer).unwrap();
    let header = packet.header;
    let questions = packet.questions;

    dns_query(&questions[0], root_servers.root_servers);
}

fn dns_query(question: &Question, root_servers: Vec<&str>){
    let mut message = Builder::new_query(1, true);
    message.add_question(question.qname.to_string().as_str(), question.prefer_unicast, question.qtype, question.qclass);
    let bytes = message.build().unwrap();

    for server in root_servers{
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let mut buf = [0; 512];
        let _ = socket.connect(server.to_string()+":53").unwrap();
        let _ = socket.send(&bytes).unwrap();
        let nbytes = socket.recv(&mut buf).unwrap();

        // Redirects me to visit the name servers
        let packet = Packet::parse(&buf[..nbytes]).unwrap();

        println!("Answers: {:?}", packet.answers);
        println!("NameServer(s) {:?}", packet.nameservers);
        println!("Opt: {:?}", packet.opt);

    }
}
