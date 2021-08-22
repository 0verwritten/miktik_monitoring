extern crate async_std;

use termion::input::TermRead;
use miktik_api::miktik_api::Connector;
use std::{ io, io::{Write, stdout, stdin}, env, sync::{ Arc, Mutex } };

/// brings interactiveness to the application
// fn interactive(mut connections: std::vec::Vec<Connector>){
fn interactive(mut connections: Option<std::vec::Vec<Connector>>){
    let mut lines = Vec::new();
    if connections.is_none() {
        let (mut addr, mut port, mut use_ssl, mut username) = ( String::new(), String::new(), String::new(), String::new() );

        print!("Enter router's address: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut addr).expect("Could not read address");

        print!("Enter router's port: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut port).expect("Could not read port");
        let port: usize = port.trim().parse().expect(&format!("Could not convert {:?} into string", port));

        print!("Do you want to use ssl? (Y/n) ");
        stdout().flush().unwrap();
        stdin().read_line(&mut use_ssl).expect("Could not read answer");

        connections = match Connector::new(format!("{}:{}", addr.trim(), port), String::from("chosen_one"), if use_ssl.trim().to_lowercase() == "y" { true } else { false }, None, None){
            val if val.is_connected() => Some(vec![val]),
            _ => panic!("Could not establish connection with router !!")
        };

        print!("Enter username: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut username).expect("Could not read username");

        print!("Enter password: ");
        stdout().flush().unwrap();
        let passwd = stdin().read_passwd(&mut stdout().lock())
                            .expect("Could not read password").expect("Could not read password");
        println!();

        connections.as_mut().unwrap()[0].login(&username.trim(), &passwd.trim(), true, true).expect("Could not connect to router !!");
    }

    let connections = connections.as_mut().unwrap();

    loop{
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        if  line == "."{
            break;
        }else if line == "?"{
            println!("Help:\n? - to get help again\n. - to stop\n<command> - to write command\nall queries that you want to put into the function must be on the next line\nTo send command press Enter ( new line ) again");
        }else if line == ""{
            lines.push(line.to_string());
            for i in 0..connections.len(){
                if let Err(msg) =  connections[i].tell(&lines, true, None){
                    eprintln!("{} returned error: {}", connections[i], msg);
                }
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

    if env::args().filter( |x| {x == "-h" || x == "--help"} ).count() > 0 {
        println!("Help:\n-i\t\tget interactive console\n-ia\t\tget interactive console with all the routers from credentials.json file\n-h\t\tget some help\n-q <file name>\texecute commands from file ( not developed yet )\n-c <file name>\tcridentials file ( not even close to be done )\n");
        println!("Enviroment variables example:\nmiktik_server_address=0.0.0.0\nmiktik_server_port=7878");
        println!("\nHelp for interactive console:\n? - to get help again\n. - to stop\n<command> - to write command\nall queries that you want to put into the function must be on the next line\nTo send command press Enter ( new line ) again");
        return;
    }

    let uri: String = match env::vars_os().find(|x| { x.0  == "miktik_server_address"}){
        Some(val) => val.1.into_string().unwrap(),
        None => String::from("0.0.0.0")
    };
    let port: u32 = match env::vars_os().find(|x| { x.0 == "miktik_server_port"}){
        Some(val) => val.1.into_string().unwrap().parse().unwrap(),
        None => 7878
    };

    if env::args().filter( |x| {x == "-i"} ).count() > 0 {
        interactive(None);
    }else if env::args().filter( |x| {x == "-ia"} ).count() > 0 {
        interactive(Some(Connector::initial( String::from("./config/credentials.json"), true ).unwrap()));
    }else if env::args().filter( |x| {x == "-v"} ).count() > 0 {
        Connector::queries_teller(Arc::new(Connector::initial( String::from("./config/credentials.json"), true ).unwrap().into_iter().map(|x| Mutex::from(x)).collect()), "./config/commands.json".to_string(), true, uri, port).await;
    }
    else{
        Connector::queries_teller(Arc::new(Connector::initial( String::from("./config/credentials.json"), false ).unwrap().into_iter().map(|x| Mutex::from(x)).collect()), "./config/commands.json".to_string(), false, uri, port).await;
    }
}