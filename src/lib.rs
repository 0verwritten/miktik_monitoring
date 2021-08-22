pub mod miktik_api{
    extern crate rustls;
    // extern crate openssl;
    extern crate serde;
    extern crate chrono;
    extern crate tiny_http;
    extern crate termion;

    use std::thread;
    use std::sync::{Arc, Mutex};

    use std::{
        time::{ Duration },
        collections::HashMap,
        fmt, fmt::{ Display, Formatter },
        io, io::{Read, Write, ErrorKind, BufReader},
        net, net::{ TcpStream, ToSocketAddrs }};
    use core::str::from_utf8;
    use termion::{ color, style };
    use webpki::{DNSNameRef};
    use rustls::{ RootCertStore, TLSError, Certificate, ServerCertVerified, ServerCertVerifier, ClientConfig, Stream, ClientSession };
    // use openssl::{ ssl, ssl::{ SslMethod, SslStream } };

    /// Responce Error type
    pub enum ConnectionError{
        ResponceError(String),
        IoError(io::Error)
    }

    /// Mikrotik connector main struct
    // #[derive(Debug)]
    pub struct Connector{
        stream: Option<TcpStream>,
        // ssl_stream: Option<SslStream<TcpStream>>,
        ssl_stream: Option<ClientSession>,
        address: String,
        username: Option<String>, // saves cridencials to restores session ( in development )
        password: Option<String>,
        cert: Option<String>, // cerification location of there is one
        ca_cert: Option<String>,
        secured: bool,
        connected: bool,
        instance_name: String
    }

    /// Commans config deserealization parental struct
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct Commands{
        // prefix: String, TODO
        commands: Vec<Queries>
    }

    /// Commands config deserealization structure
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct Queries{
        split_targets: Option<Vec<String>>,
        split_character: Option<String>,
        graph_targets: Option<Vec<String>>,
        attributes: Option<Vec<String>>,
        query: Option<Mutex<Vec<String>>>,
        separator: Option<String>,
        command: String,
        name: String,
    }

    /// Struct that contains user credentials
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct Identity{
        name: String,
        uri: String,
        use_ssl: bool,
        username: String,
        password: String,
        cert: Option<String>, // cerification location of there is one
        ca_cert: Option<String>
    }

    impl Connector{

        /// Initialization of connection object
        pub fn new(addr: String, instance_name: String, use_ssl: bool, cert_file: Option<&String>, ca_cert_file: Option<&String>) -> Connector{
            let connection;
            let mut stream = Err(io::Error::from(ErrorKind::NotFound));
            if let Ok(val) = addr.to_socket_addrs(){
                for addres in &val.collect::<Vec<_>>(){
                    // stream = match net::TcpStream::connect(&addr){
                    stream = match net::TcpStream::connect_timeout(&addres, Duration::new(2, 2)){
                        Ok(con) => Ok(con),
                        Err(err) => Err(err),
                    };
                    if let Ok(_) = stream{
                        break;
                    }
                }
            }
            if let Err(_) = stream{
                return Connector{ 
                    stream: None, 
                    ssl_stream: None, 
                    username: None, 
                    password: None, 
                    secured: use_ssl, 
                    address: addr, 
                    ca_cert: match ca_cert_file { Some(val) => Some(String::from(val)), None => None }, 
                    cert: match cert_file { Some(val) => Some(String::from(val)), None => None }, 
                    connected: false,
                    instance_name: instance_name
                }
            }
            let stream = stream.unwrap();

            stream.set_read_timeout(    Some(Duration::new(2,0))   ).unwrap();
            stream.set_write_timeout(   Some(Duration::new(2,0))   ).unwrap();

            if !use_ssl{
                connection = Connector{
                        stream: Some(stream),
                        ssl_stream: None,
                        username: None,
                        password: None,
                        secured: false,
                        address: addr,
                        ca_cert: None,
                        cert: None,
                        connected: true,
                        instance_name: instance_name
                    };
            }else{
                // let mut connector = ssl::SslConnector::builder(SslMethod::tls_client()).unwrap();
                let mut connector = ClientConfig::new();
                if let Some(ca_file) = ca_cert_file {    
                    connector.root_store.add_pem_file(&mut BufReader::new(std::fs::File::open(ca_file).unwrap())).unwrap();
                    // connector.set_ca_file( std::path::Path::new(ca_file)).unwrap();
                    // connector.set_verify(ssl::SslVerifyMode::PEER);
                    println!("\nca");
                } if let Some(_cert) = cert_file{
                    println!("cert\n");
                    // connector.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
                    // connector.set_certificate_file( std::path::Path::new(cert), ssl::SslFiletype::PEM ).unwrap();
                    // connector.set_verify(ssl::SslVerifyMode::PEER); 
                } else { 
                    eprintln!("{}Warning!{} No certificate verification used in {}", color::Fg(color::Yellow), color::Fg(color::Reset), addr);
                    connector.dangerous().set_certificate_verifier(Arc::new(NoVerify {}));
                    // connector.set_verify(ssl::SslVerifyMode::NONE);
                }
                println!("{}",addr);
                let dns_name = webpki::DNSNameRef::try_from_ascii_str(&addr.split(":").nth(0).unwrap()).unwrap();


                // let connector = connector.build();

                connection = Connector{
                        // stream: None,
                        stream: Some(stream),
                        // ssl_stream: Some(connector.connect(&addr, stream).unwrap()),
                        // ssl_stream: Some(Stream::new(&mut ClientSession::new(&Arc::new(connector), dns_name), &mut stream)),
                        ssl_stream: Some(ClientSession::new(&Arc::new(connector), dns_name)),
                        username: None,
                        password: None,
                        secured: true,
                        address: addr,
                        ca_cert: match ca_cert_file { Some(val) => Some(String::from(val)), None => None },
                        cert: match cert_file { Some(val) => Some(String::from(val)), None => None },
                        connected: true,
                        instance_name: instance_name
                    };
            }
            println!("{}Connected{} to {}", color::Fg(color::LightGreen), color::Fg(color::Reset), connection.address);
            connection
        }


        /// Creates connactions using `credentials.json` file and at the same time logins them
        pub fn initial(file: String, verbose: bool) -> Result<Vec::<Connector>, String>{
            let mut connections = Vec::new();
            let data: Vec::<Identity> = type_reader(&file);

            for item in &data{
                let mut connection = Connector::new( match item.uri.parse() { Ok(val) => val, Err(msg) => return Err(msg.to_string()) }, item.name.to_string(), item.use_ssl, item.cert.as_ref(), item.ca_cert.as_ref() );
                if !connection.connected { 
                    eprintln!("{}Error{} connecting to {}. Skipping", color::Fg(color::LightRed), color::Fg(color::Reset), item.uri); 
                    connection.password = Some(item.password.to_string());
                    connection.username = Some(item.username.to_string());
                    connections.push(connection);
                }else{
                    match connection.login(&item.username, &item.password, true, verbose){
                        Ok(_) => connections.push(connection),
                        Err(err) => { eprintln!("{}Error{} on logining {} ({}):\n\"{}{}{}\"", color::Fg(color::LightRed), color::Fg(color::Reset), item.name, item.uri, style::Bold, err, style::Reset) }
                    }
                }
            }

            return  if connections.len() == 0 { Err(String::from("Connected to 0 instances!"))} 
                    else if connections.len() < data.len() { eprintln!("{}Connected to {} instences out of {}{}", color::Fg(color::LightYellow), connections.len(), data.len(), color::Fg(color::Reset)); Ok(connections) }
                    else { Ok(connections) };
        }

        fn reconnect(&mut self, verbose: bool) -> Result<(), ConnectionError>{
            let credentials = (self.username.as_ref().unwrap().to_string(), self.password.as_ref().unwrap().to_string());
            *self =  Connector::new(self.address.to_string(), self.instance_name.to_string(), self.secured, self.cert.as_ref(), self.ca_cert.as_ref());
            match self.login(&credentials.0, &credentials.1, true, verbose){
                Ok(_) => Ok(()),
                Err(err) => {
                    self.username = Some(credentials.0);
                    self.password = Some(credentials.1);
                    Err(ConnectionError::ResponceError(err))
                }
            }
        }

        /// Logins into the routerboatd
        pub fn login(&mut self, username: &str, pwd: &str, overwrite: bool, verbose: bool) -> Result<(), String>{
            if self.username == None || overwrite == true { self.username = Some(String::from(username)); }
            if self.password == None || overwrite == true { self.password = Some(String::from(pwd)); }
            match self.tell(&["/login".to_string(), format!("=name={}", self.username.as_ref().unwrap()), format!("=password={}", self.password.as_ref().unwrap())].to_vec(), false, None){
                Ok(responce) => { 
                    if verbose == true { println!("login responce: {}", responce); }
                    if responce.contains( "!done" ) { return Ok(()); }
                    else { return Err( responce.to_string() ); }
                },
                Err(msg) => return Err( msg.to_string() )
            };
        }

        /// Reads responce from the network stream after [Teller] send the request
        /// [Teller]: tell
        fn reader(&mut self) -> Result<String, io::Error>{
            let mut res_bytes = Vec::<u8>::new();        
            let mut data = [0 as u8; 50]; // using 50 byte buffer
            
            if self.secured {
                let mut tls_stream = Stream::new(self.ssl_stream.as_mut().unwrap(), self.stream.as_mut().unwrap());
                loop{
                    match tls_stream.read(&mut data) {
                        Ok(size) => {
                            for value in 0..size { res_bytes.push(data[value]); };
                            if size <= 7 || data[ size - 7..size] == [ 5, 33, 100, 111, 110, 101, 0 ] { break; } // '!done ' sign means end of sentence
                        },
                        Err(err) => { return Err(err); }
                    }

                }
            } else { 
                loop{
                    match self.stream.as_mut().unwrap().read(&mut data) {
                        Ok(size) => {
                            for value in 0..size { res_bytes.push(data[value]); };
                            if size <= 7 || data[ size - 7..size] == [ 5, 33, 100, 111, 110, 101, 0 ] { break; } // '!done ' sign means end of sentence
                        },
                        Err(err) => { return Err(err); }
                    }
                }
            }
            Ok(bytes_to_str(&res_bytes))
        }
        
        /// Responce formater
        fn response_decoder(&mut self, responce: &str, query: &Queries, verbose: bool) -> Result::< HashMap::<String, Vec<Vec<(String, bool, String)>>>, String> {
            let mut res         = HashMap::<String, Vec<Vec<(String, bool, String)>>>::new();
            let mut landfill    = responce.split("\n");
            let     fst_value   = landfill.nth(0).unwrap();

            if  &fst_value.len() >= &5usize && &fst_value[..5] == "!done" && verbose{
                return Err(format!("{}Empty message recieved{}: '!done' from {}{}{} in command {}", color::Fg(color::LightYellow), color::Fg(color::Reset), color::Fg(color::LightCyan),self.address, color::Fg(color::Reset), query.command));
            }if &fst_value.len() >= &5usize && &fst_value[..5] == "!trap" && verbose || &fst_value.len() >= &6usize && &fst_value[..6] == "!fatal" && verbose {
                return Err(format!("{}{}{} responce from {}{}{} in command {}", color::Fg(color::LightRed), fst_value, color::Fg(color::Reset), color::Fg(color::LightCyan),self.address, color::Fg(color::Reset), query.command));
            }
            let mut res_values   = Vec::<(String, bool, String)>::new();  
            for piece in landfill{
                {
                    if &piece[..] == "!re"{
                        
                        res_values.push((String::from("routerboard_name"), false, self.instance_name.to_string()));
                        res.entry(query.name.to_string()).or_insert(Vec::new()).push(res_values);
                        res_values = Vec::new();

                        continue;
                    }else if piece.contains("=") {
                        let mut key = piece.split("=");
                        let (key, mut value) = (key.nth(0).unwrap(), key.nth(0).unwrap());
                        let key_formated = key.replace("-", "_");
                        if value == "true" { value = "1"; }
                        else if value == "false" { value = "0"; }

                        if query.split_character != None && query.split_targets.as_ref().unwrap_or(&Vec::new()).contains(&key.to_string()) && value.contains(query.split_character.as_ref().unwrap()){
                            let value = value.split(query.split_character.as_ref().unwrap()).collect::<Vec<&str>>();
                            if let Some(val) = query.graph_targets.as_ref(){
                                if val.contains(&key.to_string()){
                                    for i in 0..value.len(){ if value[i] != "" {
                                        res_values.push((format!("{}_{}", key_formated, i), true, value[i].to_string()));
                                    }}
                                }
                            }
                            if let Some(val) = &query.attributes{
                                if val.contains(&key.to_string()){
                                    for i in 0..value.len(){ if value[i] != "" {
                                        res_values.push((format!("{}_{}", key_formated, i), false, value[i].to_string()));
                                    }}
                                }
                            }
                            else if &None == &query.attributes { for i in 0..value.len(){ if value[i] != "" { res_values.push((format!("{}_{}", key_formated, i), false, String::from(value[i]))); } } }
                        }else{
                            if let Some(val) = query.graph_targets.as_ref(){
                                if val.contains(&key.to_string()){
                                    res_values.push((String::from(&key_formated), true, value.to_string()));
                                }
                            }
                            if let Some(val) = &query.attributes {
                                if val.contains(&key.to_string()){
                                    res_values.push((key_formated, false, String::from(value)));
                                }
                            }else if &query.attributes == &None{ res_values.push((key_formated, false, String::from(value))); }
                        }
                    }
                }
            }

            res_values.push((String::from("routerboard_name"), false, self.instance_name.to_string()));
            res.entry(query.name.to_string()).or_insert(Vec::new()).push(res_values);
            
            Ok(res)
        }

        /// Converts keys and values from `response_decoder` into a result string vector
        fn web_responce_formater(data: &mut HashMap<String, Vec<Vec<(String, bool, String)>>>, output: &mut String) {
            for (query_name, attributes) in data{
                for attribute in attributes{
                    let mut attributes_slice = Vec::new();
                    let mut to_display = HashMap::<String, isize>::new();
                    for (attribute_name, displayshion, value) in attribute {
                        if *displayshion { if let Ok(val) = value.parse::<isize>(){ to_display.insert(attribute_name.to_string(), val); } }
                        else{ 
                            if attribute_name != "routerboard_address" && attribute_name != "routerboard_name" {
                                attributes_slice.push(format!( "{}_{}=\"{}\"", query_name, attribute_name.replace("-", "_"), value ));
                            } else {
                                attributes_slice.push(format!( "{}=\"{}\"", attribute_name.replace("-", "_"), value ));
                            }
                        }
                    }
                    if to_display.len() == 0 { output.push_str( &format!("miktik_{}{{{}}} 0\n", query_name, attributes_slice.join(", ")) ); }
                    for (attribute_name, value) in to_display {
                        output.push_str( &format!("miktik_{}_{}{{{}}} {}\n", query_name, attribute_name, attributes_slice.join(", "), value) );
                    }
                }
            }
        }
        
        /// Sends commands to routerboard after [Login] has been perforned
        /// 
        /// [Login]: login
        pub fn tell(&mut self, lines: &Vec::<String>, verbose: bool, _attributes: Option<&Vec<String>>) -> Result<String, io::Error>{//sender: &mut [net::TcpStream]
            if !self.connected { return Err(io::Error::from(ErrorKind::NotConnected)); }
            let mut text = Vec::<u8>::new();
            for l in lines{
                for x in hexer(l.as_bytes(), false){
                    text.push(x);
                }
            }
            text.push(0);
            if self.secured { 
                let mut tls_stream = Stream::new(self.ssl_stream.as_mut().unwrap(), self.stream.as_mut().unwrap());
                // (self.ssl_stream.as_mut().unwrap()).write(&text).unwrap();
                tls_stream.write(&text).unwrap(); 
            } 
            else            { self.stream.as_mut().unwrap().write(&text).unwrap(); }
            
            let output = match self.reader(){
                Ok(val) => val,
                Err(err) => return Err(err)
            };
            if verbose == true{
                println!(">> {}", &output);
            }
            Ok(output)
        }

        /// Sends commands from list to routerboard after [Login] has been perforned
        /// 
        /// [Login]: login
        pub fn tell_get(&mut self, lines: &Vec::<String>, verbose: bool, query: &Queries) -> Result<HashMap::<String, Vec<Vec<(String, bool, String)>>>, ConnectionError>{//sender: &mut [net::TcpStream]
            let mut text = Vec::<u8>::new();
            for l in lines{
                for x in hexer(l.as_bytes(), false){
                    text.push(x);
                }
            }
            match &query.query{
                Some(val) => { for l in &*val.lock().unwrap() { for x in hexer(l.as_bytes(), false) { text.push(x); } } }
                None => ()
            }
            text.push(0);
            if self.secured { 
                let mut tls_stream = Stream::new(self.ssl_stream.as_mut().unwrap(), self.stream.as_mut().unwrap());
                // (self.ssl_stream.as_mut().unwrap()).write(&text).unwrap();
                tls_stream.write(&text).unwrap(); 
            } 
            else            { self.stream.as_mut().unwrap().write(&text).unwrap(); }
            
            let output = match self.reader(){
                Ok(val) => val,
                Err(err) => return Err(ConnectionError::IoError(err))
            };
            let hash_res = self.response_decoder(&output[..], query, verbose);
            if verbose == true{
                match &hash_res{
                    Ok(_val) => () /*println!(">> {:#?}", _val)*/,
                    Err(msg) => eprintln!("{}Error{}: {}", color::Fg(color::LightRed), color::Fg(color::Reset), msg)
                }
            }

            match hash_res{
                Ok(value) => Ok(value),
                Err(msg) => Err(ConnectionError::ResponceError(msg))
            }
        }
        
        /// Executes commands from list runs web server with metrics to be reserved by prometheus
        pub async fn queries_teller(connections: Arc<Vec<Mutex<Connector>>>, queries_file: String, verbosibility: bool, uri: String, port: u32, ) -> bool /*Vec::<String>*/ {
            let metrics = Arc::new(Mutex::new(HashMap::<String, Vec<Vec<(String, bool, String)>>>::new()));
            let server = tiny_http::Server::http(format!("{}:{}", uri, port)).unwrap();
            let mut queries: Arc<Commands> = Arc::new(type_reader(&queries_file));
            let reconnect_candidates = Arc::new(Mutex::new(Vec::new()));
            let connections_len = connections.len();

            for i in 0..connections_len{
                if !connections[i].lock().unwrap().connected { reconnect_candidates.lock().unwrap().push(i); }
            }

            println!("{}Starting listening{} on: {}http://{}:{}{}", color::Fg(color::LightGreen), color::Fg(color::Reset), color::Fg(color::LightCyan), uri, port, color::Fg(color::Reset));

            loop{

                let request = match server.recv() {
                    Ok(rq) => { rq },
                    Err(e) => { eprintln!("error: {}", e); break }
                };

                match request.url() {
                    "/metrics" => {
                        println!("{}{:?}{}:\twent to metrics page", style::Bold, request, style::Reset);

                        let mut tasks = Vec::new();
                        for i in 0..connections_len {
                            if !reconnect_candidates.lock().unwrap().contains(&i){
                                let queries = Arc::clone(&queries);
                                let connections = Arc::clone(&connections);
                                let reconnect_candidates = Arc::clone(&reconnect_candidates);
                                let metrics = Arc::clone(&metrics);
                                tasks.push(thread::spawn( move || {
                                    for command in &queries.commands{
                                        let mut prev = Vec::new();
                                        if let Some(query) = &command.query {
                                            let query_len = query.lock().unwrap().len();
                                            for item in 0..query_len {
                                                if query.lock().unwrap()[item].contains("${"){
                                                    let mut  opened = false;
                                                    let value = query.lock().unwrap()[item].chars().map( |x| { if x == '}' { opened = false; } if opened { return x } if x == '{' { opened = true; } ' '  } ).collect::<String>();
                                                    let mut value = value.trim().split('.');
                                                    if  let (Some(key), Some(value)) = (value.next(), value.next()){
                                                        if let Some(entry) = metrics.lock().unwrap().get(key) {
                                                            let mut opened = false;
                                                            let res = (|| {let mut res = String::new(); for q in query.lock().unwrap()[item].chars() {if q == '}' { opened = false; } else if q == '{' { opened = true; res+="◊"; } else if !opened { res += &q.to_string(); }  } res } )();
                                                            let mut res_val = String::new();
                                                            for ent in entry.iter() { if ent.iter().any( |x| x.2 == connections[i].lock().unwrap().instance_name.to_string() ) { for en in ent { if en.0 == value { res_val += &format!("{},", en.2); } } } }

                                                            prev.push((item, (*query.lock().unwrap())[item].to_string()));
                                                            (*query.lock().unwrap())[item] = res.replace("$◊", &format!("{}", res_val) );
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        match connections[i].lock().unwrap().tell_get( &vec![ command.command.to_string() ], verbosibility, command){
                                            Ok( val ) => { for (key,  mut value) in val { metrics.lock().unwrap().entry(key).or_insert(Vec::new()).append(&mut value); } },
                                            Err(err) => {
                                                match err{
                                                    ConnectionError::ResponceError(msg) => eprintln!("{}", msg),
                                                    ConnectionError::IoError(_err_) => { reconnect_candidates.lock().unwrap().push(i); eprintln!("Conneciton error on {}: {:?}", connections[i].lock().unwrap().address, _err_); break; }
                                                }
                                            }
                                        }
                                        for (item, val) in prev{
                                            (*command.query.as_ref().unwrap().lock().unwrap())[item] = val;
                                        }
                                        if verbosibility { println!("Command {} done on {}",  command.command, connections[i].lock().unwrap().address); }
                                    }
                                    if verbosibility { println!("{} finished\n", connections[i].lock().unwrap().address); }
                                }));
                            }
                        }
                        for task in tasks{
                            task.join().unwrap();
                        }

                        let mut res = "".to_owned();
                        for i in 0..connections_len{
                            // res += "miktik__connection__status__{routerboard_address=\""; res += &connections[i].lock().unwrap().address[..];
                            res += "miktik__connection__status__{";
                            res += "\", routerboard_name=\""; res += &connections[i].lock().unwrap().instance_name[..]; res += "\"} ";
                            if  (&*reconnect_candidates.lock().unwrap()).contains(&i){ res += "0\n"; } else { res += "1\n"; }
                        }
        
                        Connector::web_responce_formater(&mut metrics.lock().unwrap(), &mut res);
        
                        let response = tiny_http::Response::from_string(res);
                        let _ = request.respond(response);
                        metrics.lock().unwrap().clear();
                    },
                    "/" => { 
                        println!("{}{:?}{}: console home is here", style::Bold, request, style::Reset);
                        match request.respond(tiny_http::Response::from_file(std::fs::File::open("./templates/index.html").unwrap())){
                            Ok(_) => (),
                            Err(e) => eprintln!("{}Error{} happened: {}", color::Fg(color::LightRed), color::Fg(color::Reset), e)
                        }
                    },
                    "/imgs/gears.gif" => {
                        println!("{}{:?}{}:\tgear gif file is here", style::Bold, request, style::Reset);
                        match request.respond(tiny_http::Response::from_file(std::fs::File::open("./templates/images/gears.gif").unwrap())){
                            Ok(_) => (),
                            Err(e) => eprintln!("{}Error{} happened: {}", color::Fg(color::LightRed), color::Fg(color::Reset), e)
                        }
                    },
                    "/config/update" => {
                        println!("{}{:?}{}:\tupdating config", style::Bold, request, style::Reset);
                        queries = Arc::new(type_reader(&queries_file));
                        match request.respond(tiny_http::Response::from_file(std::fs::File::open("./templates/reload_config.html").unwrap())){
                            Ok(_) => (),
                            Err(e) => eprintln!("{}Error{} happened: {}", color::Fg(color::LightRed), color::Fg(color::Reset), e)
                        }
                    }
                    _   => {
                        println!("{}{:?}{}", style::Bold, request, style::Reset);
                    }
                }

                let mut removed = 0;
                let mut recon_len = reconnect_candidates.lock().unwrap().len();
                for i in 0..recon_len{
                    if verbosibility{
                        println!("Starting reconnection to {}", connections[reconnect_candidates.lock().unwrap()[i]].lock().unwrap().address.to_string());
                    }
                    let reconnect_res = connections[reconnect_candidates.lock().unwrap()[i-removed]].lock().unwrap().reconnect(false);
                    match reconnect_res{
                        Ok(()) => { 
                            println!("{}Reconnected{} to {}", color::Fg(color::LightGreen), color::Fg(color::Reset), connections[reconnect_candidates.lock().unwrap()[i-removed]].lock().unwrap().address);
                            if recon_len != 0{
                                reconnect_candidates.lock().unwrap().remove(i - removed);
                                recon_len -= 1;
                                removed += 1;
                            }
                        },
                        Err(err) => { 
                            match err {
                                ConnectionError::IoError(io_err) => eprintln!("{}Connection error{} during reconnection: {}", color::Fg(color::LightRed), color::Fg(color::Reset), io_err),
                                ConnectionError::ResponceError(msg) => eprintln!("{}Resonce error{} during reconnection: {}", color::Fg(color::LightRed), color::Fg(color::Reset), msg)
                            }
                        }
                    };
                }
                // std::thread::sleep(date_array_to_duration(queries.interval)); // not used because prometheus will do it itself
            }
            true
        }

        /// Returns connection state
        /// 
        pub fn is_connected(&self) -> bool { self.connected }
    }
    impl Display for Connector {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{} ({})", self.instance_name, self.address)
        }
    }

    impl Drop for Connector{
        fn drop(&mut self) {
            if self.connected{
                match self.secured{
                    true => {
                        // if let Err(msg) = self.ssl_stream.as_mut().unwrap().shutdown(){
                        //     eprintln!("Error during closing session {}\n{}", self.address.to_string(), msg);
                        // }
                    },
                    false => {
                        if let Err(msg) = self.stream.as_mut().unwrap().shutdown(std::net::Shutdown::Both){
                            eprintln!("Error during closing session {}\n{}", self.address.to_string(), msg);
                        } 
                    }
                }
            }
            println!("Disconnected from {}", self.address.to_string());
        }
    }

    /// Disables versification in ssl connection if no certificate is present and cert is invalid
    struct NoVerify {}
    impl ServerCertVerifier for NoVerify{
        fn verify_server_cert(
            &self, 
            _roots: &RootCertStore, 
            _presented_certs: &[Certificate], 
            _dns_name: DNSNameRef<'_>, 
            _ocsp_response: &[u8]
        ) -> Result<ServerCertVerified, TLSError>{
            Ok(ServerCertVerified::assertion())
        }

    }

    /// Reads data file and returns result
    pub fn type_reader<T>(file_name: &str) -> T where T: for<'de> serde::Deserialize<'de> {
        let file        = std::fs::File::open(file_name).unwrap();
        let file_: T    = serde_json::from_reader(file).unwrap();
        return file_;
    }

    /// Decodes responce to string
    fn bytes_to_str(bytes: &[u8]) -> String {

        let mut l = 0; // every word length
        let mut res = String::new();
        let mut i = 0;

        while i < bytes.len() {
            if i == 0 || l == 0 {
                if bytes[i] == 0 { if i != bytes.len() - 1 { res += "\n"; } i+=1; continue; } 
                else {
                    if       bytes[i]         < 128          { l = bytes[i] as u32; }
                    else if (bytes[i] as u32) < 16384u32     { l = (bytes[i] as u32 - 128) * u32::pow(16, 2) + bytes[i + 1] as u32; i+=1; }
                    else if (bytes[i] as u32) < 2097152u32   { l = (bytes[i] as u32 - 192) * u32::pow(16, 4) + bytes[i + 1] as u32 * u32::pow(16, 2) + bytes[i + 2] as u32; i+=2; }
                    else if (bytes[i] as u32) < 268435456u32 { l = (bytes[i] as u32 - 224) * u32::pow(16, 6) + bytes[i + 1] as u32 * u32::pow(16, 4) + bytes[i + 2] as u32 * u32::pow(16, 2) + bytes[i + 3] as u32; i+=3; }
                    else                                     { l =  bytes[i + 1] as u32    * u32::pow(16, 6) + bytes[i + 2] as u32 * u32::pow(16, 4) + bytes[i + 3] as u32 * u32::pow(16, 2) + bytes[i + 4] as u32; i+=4;}
                    if bytes[i+1] == 61 { res+="\n"; l-=1; i+=1; }  // skips first '=' character if there is any
                    else if bytes[i+1] >= 248 { panic!("Control bytes recieved"); } // in case of control byte
                }
                i+=1;
            }
            else {
                match from_utf8(&bytes[ i..i + (l as usize) ]){
                    Ok(val) => {
                        res += val
                    },
                    Err(e) => eprintln!("Error during responce decoding byte {}: {}", bytes[i], e) 
                }
                i += l as usize;
                l = 0;
            }
        }
        res
    }

    /// Converts dec base to hex and adds length in the beginning as mikrotik api want
    fn dec_to_hex(mut value: usize) -> Vec::<u8>{
        let val = value;
        let mut res = Vec::new();

        while value >= 16 {
            res.push( ( value % 16 ) as u8 );
            value /= 16;
            if value >= 16 {
                *res.last_mut().unwrap() += 16 * ( value % 16 ) as u8;
                value /= 16;
            }else{
                *res.last_mut().unwrap() += 16 * value as u8;
                value = 0;
            }
        }
        if value != 0{
            res.push( value as u8 );
        }

        // adding prefixed according to protocol
        if      val < 16384usize     { if res.len() == 1 { res.push(0u8);} res[1] += 128; }
        else if val < 2097152usize   { if res.len() == 2 { res.push(0u8);} res[2] += 192; }
        else if val < 268435456usize { if res.len() == 3 { res.push(0u8);} res[3] += 224; }
        else                         { res.push( 240 as u8 ); }

        res.reverse();
        res
    }

    /// Converts dec array to hex array
    fn hexer(value: &[u8], add_last: bool) -> Vec::<u8>{
        let len = value.len();
        let mut res = Vec::<u8>::new();
        
        if len < 128{
            res.push(len as u8);
        }else {
            res = dec_to_hex(len);
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