extern crate async_std;

use mik_api::mik_api::queries_reader;
use std::{ io, env, future, task };
use mik_api::mik_api::Connector;
use std::net::SocketAddr;
use core::str::from_utf8;

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
                connections[i].tell(&lines, true, None).expect("Tell error");
            }

            lines.clear();
        }else{
            lines.push(line.to_string());
        }
    }
    println!("Session ended");
}

#[async_std::main]
async fn main(){

    let addrs = [
            SocketAddr::new("<in address>".parse().unwrap(), 8729),
        ];
    let login = "user1";
    let pass = "123";

    let mut connections: Vec::<Connector> = Connector::new(&addrs, false, true).unwrap();
    let connections_len = connections.len();

    for i in 0..connections.len(){
        connections[i].login(login, pass, false, true).expect("Login error");
        if i != connections_len - 1{
            connections[i].queries_teller(queries_reader("commands.json"), false);
        }
    }

    let key = "INTERACT";
    match env::var_os(key) {
        Some(_) => interactive(connections),
        None => { connections[connections_len-1].queries_teller(queries_reader("commands.json"), false).await; }
    }
}