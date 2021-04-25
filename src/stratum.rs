#![allow(dead_code, unused_variables)]

//
// Imports
//

use super::protocol::ValidatedMinerConf;
use json::{self, JsonValue};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

//
// Type aliases
//

type MinerId = String;

//
// Enumerations
//

#[derive(Clone)]
pub enum StratumResponse {
    Login(MinerId, StratumJob),
    Invalid,
}

//
// Structures
//

struct JsonRpcResponse {
    id: JsonValue,
    error: JsonValue,
    result: JsonValue,
}

#[derive(Clone)]
pub struct StratumJob {
    pub blob: String,
    pub job_id: String,
    pub target: String,
}

/// Stratum+TCP client.
pub struct StratumClient {
    current_id: u64,
    username: String,
    password: String,
    endpoint: String,
    handlers: Vec<Sender<StratumResponse>>,
    send_thread: Option<thread::JoinHandle<()>>,
    recv_thread: Option<thread::JoinHandle<()>>,
    sender: Option<Sender<String>>,
    connected: bool,
}

//
// Implementations
//

impl From<JsonValue> for JsonRpcResponse {
    fn from(val: JsonValue) -> Self {
        Self {
            id: val["id"].clone(),
            error: val["error"].clone(),
            result: val["result"].clone(),
        }
    }
}

impl<'a> From<&'a JsonRpcResponse> for StratumJob {
    fn from(val: &JsonRpcResponse) -> Self {
        Self {
            blob: val.result["job"]["blob"].as_str().unwrap().to_owned(),
            job_id: val.result["job"]["job_id"].as_str().unwrap().to_owned(),
            target: val.result["job"]["target"].as_str().unwrap().to_owned(),
        }
    }
}

impl StratumClient {
    /// Constructs a new `StratumClient`.
    pub fn new(conf: ValidatedMinerConf, handlers: Vec<Sender<StratumResponse>>) -> StratumClient {
        StratumClient {
            current_id: 1,
            username: conf.user,
            password: conf.pass,
            endpoint: conf.pool,
            handlers: handlers,
            send_thread: None,
            recv_thread: None,
            connected: false,
            sender: None,
        }
    }

    /// Connects to the pool.
    pub fn connect(&mut self) {
        println!("Starting Stratum on stratum+tcp://{}", self.endpoint);
        if let Ok(stream) = TcpStream::connect(&self.endpoint) {
            // Create buffered reader and writer
            let reader = BufReader::new(stream.try_clone().unwrap());
            let writer = BufWriter::new(stream);

            // Create send-receive channel
            let (tx, rx) = channel();
            self.sender = Some(tx);

            // Create sender-thread
            self.send_thread = Some(thread::spawn(move || {
                handle_send(rx, writer);
            }));

            // Create receiver-thread
            let handlers = self.handlers.clone();
            self.recv_thread = Some(thread::spawn(move || handle_recv(reader, &handlers)));

            // Signal that we are connected
            self.connected = true;
        } else {
            println!("Stratum connection failed!");
        }
    }

    /// Authenticates with the pool.
    pub fn login(&mut self) {
        let body = object! {
            "jsonrpc" => "2.0",
            "method" => "login",
            "params" => object![
                "login" => self.username.clone(),
                "pass" => self.password.clone(),
            ],
            "id" => "login",
        };
        self.send(body);
    }

    pub fn share(&mut self) -> u64 {
        let id = self.get_id();
        let miner_id = 0;
        let job_id = 0;
        let nonce = "foo";
        let hash = "foo";
        let body = object! {
            "jsonrpc" => "2.0",
            "method" => "submit",
            "params" => object![
                "id" => miner_id,
                "job_id" => job_id,
                "nonce" => nonce,
                "result" => hash,
            ],
            "id" => id,
        };
        self.send(body);
        id
    }

    /// Blocks while the connection is alive.
    pub fn join(self) {
        self.send_thread.unwrap().join().unwrap();
    }

    /// Sends a JSON-RPC 2.0 object
    fn send(&mut self, json: JsonValue) {
        self.sender
            .clone()
            .unwrap()
            .send(json::stringify(json))
            .unwrap();
    }

    /// Gets a fresh JSON-RPC 2.0 id
    fn get_id(&mut self) -> u64 {
        self.current_id += 1;
        self.current_id - 1
    }
}

//
// Private Functions
//

fn handle_send(rx: Receiver<String>, mut writer: BufWriter<TcpStream>) {
    loop {
        let command = rx.recv().unwrap();
        write!(writer, "{}\n", command).unwrap();
        writer.flush().unwrap();
    }
}

fn handle_recv(mut reader: BufReader<TcpStream>, handlers: &Vec<Sender<StratumResponse>>) {
    loop {
        let mut buf = String::new();
        if reader.read_line(&mut buf).is_ok() {
            if !buf.is_empty() {
                let data = json::parse(&buf).unwrap();
                let rpc_resp = JsonRpcResponse::from(data);
                if !rpc_resp.error.is_null() {
                    println!("Error: {}", rpc_resp.error["message"]);
                    continue;
                }
                let stratum_resp = if !rpc_resp.result["job"].is_null() {
                    let job = StratumJob::from(&rpc_resp);
                    let miner_id = match rpc_resp.result["id"].as_str() {
                        Some(val) => val.to_string(),
                        None => {
                            println!("Invalid miner id!");
                            continue;
                        }
                    };
                    StratumResponse::Login(miner_id, job)
                } else {
                    println!("Invalid Stratum response!");
                    continue;
                };
                for handler in handlers {
                    handler.send(stratum_resp.clone()).unwrap();
                }
            }
        } else {
            println!("Unable to parse Stratum response!");
        }
    }
}
