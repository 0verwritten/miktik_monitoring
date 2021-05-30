extern crate async_std;

use mik_api::mik_api::queries_reader;
use core::str::from_utf8;
use mik_api::mik_api::Connector;
use std::net::SocketAddr;
use std::io;
use std::env;
use std::future;


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
}

#[async_std::main]
async fn main(){

    let addrs = [
            // SocketAddr::new("<in address>".parse().unwrap(), 8729),
            SocketAddr::new("10.13.40.8".parse().unwrap(), 8728),
        ];
    let login = "user1";
    let pass = "123";

    let mut connections: Vec::<Connector> = Connector::new(&addrs, false, true).unwrap();
    let connections_len = connections.len();

    for i in 0..connections.len(){
        &connections[i].login(login, pass, false, true).expect("Login error");
        if i != connections_len - 1{
            connections[i].queries_teller(queries_reader("commands.json"), true);
        }
    }
    let mut  lst_progress = connections[connections_len-1].queries_teller(queries_reader("commands.json"), true);
    
    println!("{:?}", lst_progress.await);

    // connections[0].tell(&["/ip/address/print".to_string()].to_vec(), true, None).unwrap();

    // println!("{:#?}", queries_reader("commands.json"));

    // interactive(connections);

    // println!("Session ended");
}