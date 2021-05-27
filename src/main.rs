use core::str::from_utf8;
use mik_api::mik_api::Connector;
use std::net::SocketAddr;
use std::io;
use std::env;

fn interactive(mut connections: std::vec::Vec<mik_api::mik_api::Connector>){
    let mut lines = Vec::new();
    loop{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        if  line == "."{
            break;
        }else if line == ""{
            lines.push(line.to_string());
            for i in 0..connections.len(){
                connections[i].tell(&lines, true).expect("Tell error");
            }

            lines.clear();
        }else{
            lines.push(line.to_string());
        }
    }
}

fn main(){

    let addrs = [
            SocketAddr::new("<in address>".parse().unwrap(), 8729),

        ];
    let login = "user1";
    let pass = "123";

    let mut connections: Vec::<Connector> = Connector::new(&addrs, false, true).unwrap();

    for i in 0..connections.len(){
        connections[i].login(login, pass, false, true).expect("Login error");
    }
    
    // connections[0].tell(&["/ip/address/print".to_string()].to_vec(), true);
    interactive(connections);

    println!("Session ended");
}