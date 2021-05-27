pub mod mik_api{
    extern crate openssl;
    extern crate serde;

    use std::time::Duration;
    use std::collections::HashMap;
    use core::str::from_utf8;
    use std::io::{Read, Write};
    use std::{net};
    use std::net::TcpStream;
    use openssl::ssl;
    use openssl::ssl::{SslMethod, SslConnector, SslStream};

    pub struct Connector{
        stream: Option<TcpStream>,
        ssl_stream: Option<SslStream<TcpStream>>,
        secured: bool,
        username: Option<String>, // saves cridencials to restores session ( in development )
        password: Option<String>,
    }

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct Queries{
        pub command: String,
        pub multiple_objects: bool,
        pub attributes: Vec<String>
    }

    impl Connector{
        pub fn new(addrs: &[std::net::SocketAddr], use_ssl: bool, verbose: bool) -> Result<Vec::<Connector>, String>{
            let mut connections = Vec::new();
            for x in addrs.iter(){
                let stream = net::TcpStream::connect(x).unwrap();
                if !use_ssl{
                    connections.push(
                        Connector{
                            stream: Some(stream),
                            ssl_stream: None,
                            username: None,
                            password: None,
                            secured: false
                        }
                    );
                }else{
                    let mut connector = ssl::SslConnector::builder(SslMethod::tls_client()).unwrap();
                    connector.set_verify(ssl::SslVerifyMode::NONE); // to make ssl verification !!!!! ( temporary solution )
                    let connector = connector.build();
                    connections.push(
                        Connector{
                            stream: None,
                            ssl_stream: Some(connector.connect(&x.ip().to_string(), stream).unwrap()),
                            username: None,
                            password: None,
                            secured: true
                        }
                    );
                }
                if verbose{
                    println!("Connected to {}", x);
                }
            }
            Ok(connections)
        }

        pub fn login(&mut self, username: &str, pwd: &str, overwrite: bool, verbose: bool) -> Result<(), String>{
            if self.username == None || overwrite == true { self.username = Some(String::from(username)); }
            if self.password == None || overwrite == true { self.password = Some(String::from(pwd)); }
            self.tell(&["/login".to_string(), format!("=name={}", self.username.as_ref().unwrap()), format!("=password={}", self.password.as_ref().unwrap())].to_vec(), verbose).unwrap();
            Ok(())
        }
        
        pub fn tell(&mut self, lines: &Vec::<String>, verbose: bool) -> Result<String, String>{//sender: &mut [net::TcpStream]
            let mut text = Vec::<u8>::new();
            for l in lines{
                for x in hexer(l.as_bytes(), false){
                    text.push(x);
                }
            }
            text.push(0);
            if self.secured { (self.ssl_stream.as_mut().unwrap()).write(&text).unwrap(); } 
            else            { self.stream.as_mut().unwrap().write(&text).unwrap(); }
            
            let output = self.reader();
            if verbose == true{
                // println!(">> {}", &output);
                println!("{:?}", responce_decoder(&output[..]));
            }
            Ok(output)


            // for i in 0..sender.len(){
            //     sender[i].write(&text).unwrap();
            //     // std::thread::sleep_ms(1000);
            //     println!(">> {}", reader(&mut self.stream));
            // }
        }

        fn reader(&mut self) -> String{ // net::TcpStream
            // let mut res = String::new();
            let mut res_bytes = Vec::<u8>::new();        
            let mut data = [0 as u8; 10]; // using 50 byte buffer
            if self.secured {
                loop{ match self.ssl_stream.as_mut().unwrap().read(&mut data) {
                    Ok(size) => {
                        // res += &(|| -> String { let mut res = String::new(); for value in 0..size {if data[value] == 0 { res+="\n"; } else if value !=0 && data[value - 1] == 0 { continue; } else { res += from_utf8(&[data[value]]).unwrap(); } } res })();
                        // println!("{:?}", data);

                        (|| { for value in 0..size { res_bytes.push(data[value]); } })();

                        if size < data.len() { break; }
                    },
                    Err(_) => { panic!(format!("An error occurred, terminating connection")); }
            }}} else { 
                loop{ match self.stream.as_mut().unwrap().read(&mut data) {
                    Ok(size) => {
                        // res += &(|| -> String { let mut res = String::new(); for value in 0..size {if data[value] == 0 { res+="\n"; } else if value !=0 && data[value - 1] == 0 { continue; } else { res += from_utf8(&[data[value]]).unwrap(); } } res })();
                        // for value in 0..data.len() {if data[value] == 0 { res+="\n"; } else { res += from_utf8(&[data[value]]).unwrap(); } }
                        // println!("{:?}", data);

                        (|| { for value in 0..size { res_bytes.push(data[value]); } })();

                        if size < data.len() { break; }
                    },
                    Err(_) => { panic!(format!("An error occurred, terminating connection")); }
            }}}
            // println!("{:?}", res_bytes);
            bytes_to_str(&res_bytes)
            // res
        } 
    }

    fn responce_decoder(responce: &str) -> Option::<Vec<HashMap::<String, String>>> { // temporary solution
        let mut res = Vec::<HashMap::<String, String>>::new();

        let mut landfill = responce.split("\n");

        let fst_value = landfill.nth(0).unwrap();

        if  &fst_value.len() >= &5usize && &fst_value[..5] == "!done"{
            return None;
        }if &fst_value.len() >= &5usize && &fst_value[..5] == "!trap" || &fst_value.len() >= &6usize && &fst_value[..6] == "!fatal" {
            panic!(format!("Here is an error during parsing because of invalid responce {:?}", landfill));
        }

        let mut hashpiece = HashMap::new();
        let queries: Vec<Queries> = queries_reader("commands.json");
        for piece in landfill{
            {
                if &piece[..] == "!re"{
                    res.push(hashpiece);
                    hashpiece = HashMap::new();
                    continue;
                }
                if &piece[..3] == ".id"{
                    hashpiece.insert(String::from(&piece[1..3]), String::from(&piece[5..]));
                }else if piece.contains("=") {
                    let mut key = piece.split("=");
                    let (key, value) = (key.nth(0).unwrap(), key.nth(0).unwrap());
                    if queries[0].attributes.contains(&key.to_string()){
                        hashpiece.insert(
                            String::from(key), 
                            String::from(value));
                    }
                }
            }
        }

        Some(res)
    }
    fn queries_reader(file_name: &str) -> Vec::<Queries>{
        let mut file_data = String::new();
        let mut file = std::fs::File::open(file_name).unwrap();
        file.read_to_string(&mut file_data);
        let file_data: Vec<Queries> = serde_json::from_str(&file_data).unwrap();

        file_data
    }
        
    fn bytes_to_str(bytes: &[u8]) -> String {

        let mut l = 0; // every word length
        let mut res = String::new();
        let mut iterator = 0; // iterrator for equal sign // temporary ( may be )

        for i in 0..bytes.len(){
            if i == 0 || l == 0 { if bytes[i] == 0 && i != bytes.len() - 1 { res += "\n"; continue; } else { l = bytes[i]; } }
            else { 
                l -= 1;
                match from_utf8(&[bytes[i]]){
                    Ok(val) => {
                        if val == "=" {
                            if iterator % 2== 0 { res+="\n" }
                            else { res+="="; } 
                            iterator += 1;
                        }
                        else{ res += val }
                    },
                    Err(e) => eprint!("Error during responce decoding: {}", e) 
                }
            }
        }

        res
    }

    fn dec_to_hec(mut value: usize) -> Vec::<u8>{
        let mut res = Vec::new();
        let too_high = value >= 268435456;
        while value <= 16 {
            res.push((((value / 16) % 16) * value % 16) as u8);
            value /= 256;
        }
        if too_high {
            res.reverse();
            res.push(240 as u8);
            res.reverse();
        }
        res
    }
    
    fn hexer(value: &[u8], add_last: bool) -> Vec::<u8>{
        let len = value.len();
        let mut res = Vec::<u8>::new();
        // println!("{}", len);
        
        if len < 128{
            res.push(len as u8);
        }else {
            res = dec_to_hec(len);
        }
        
        for val in value{
            res.push(*val);
        }
        
        if add_last{
            res.push(0);
        }
        res
    }   
}