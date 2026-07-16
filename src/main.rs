use std::{
    collections::HashMap,
    env::{ self },
    io::{ BufRead, BufReader, Write },
    net::{ TcpListener, TcpStream },
    process::exit,
    sync::{ Arc, Mutex },
    thread,
};
use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    from: String,
    text: String,
}

type Peers = Arc<Mutex<HashMap<String, TcpStream>>>;

fn broadcast(msg: &Message, peer: &Peers, skip: Option<&str>) {
    let json = serde_json::to_string(msg).unwrap() + "\n";
    let mut map = peer.lock().unwrap();

    map.retain(|addr, stream| {
        if skip.map_or(false, |s| s == addr) {
            return true;
        }
        stream.write_all(json.as_bytes()).is_ok()
    });
}

fn handle_connection(my_addr: String, peer: Peers, stream: TcpStream) {
    let peer_addr: String = stream.peer_addr().unwrap().to_string();
    println!("[+] peer connected: {peer_addr}");
    peer.lock().unwrap().insert(peer_addr.clone(), stream.try_clone().unwrap());

    let reader = BufReader::new(stream);
    for line in reader.lines() {
        match line {
            Ok(json) => {
                match serde_json::from_str::<Message>(&json) {
                    Ok(msg) => {
                        println!("{}: {}", msg.from, msg.text);
                        broadcast(&msg, &peer, Some(&peer_addr));
                    }

                    Err(e) => eprint!("Error: {}", e),
                }
            }

            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    peer.lock().unwrap().remove(&peer_addr);
    println!("[-] disconnected: {peer_addr}");

    let _ = my_addr;
}

fn connected_to(addr: &str, peer: Peers, my_addr: String) {
    match TcpStream::connect(addr) {
        Ok(stream) => {
            println!("[+] outbond -> {addr}");
            thread::spawn(move || handle_connection(my_addr, peer.clone(), stream));
        }

        Err(e) => eprintln!("[!] could not connect to address: {e}"),
    }
}

fn main() {
    let arg: Vec<String> = env::args().collect();
    if arg.len() < 2 {
        eprintln!("Usage: {} <port> [peer_addr ...]", arg[0]);
        exit(1);
    }

    let port = &arg[1];
    let my_add = format!("127.0.0.1:{port}");
    let peer: Peers = Arc::new(Mutex::new(HashMap::new()));

    let listner = TcpListener::bind(&my_add).expect("Bind failed.");
    println!("Listning on port: {my_add}");

    {
        let peers = peer.clone();
        let my = my_add.clone();
        thread::spawn(move || {
            for stream in listner.incoming().flatten() {
                let peers = peers.clone();
                let my = my.clone();
                thread::spawn(move || handle_connection(my, peers, stream));
            }
        });
    }

    for peer_addr in &arg[2..] {
        connected_to(peer_addr, peer.clone(), my_add.clone());
    }

    println!("Type message and press Enter to send. \n");
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let text = match line {
            Ok(t) if !t.trim().is_empty() => t,
            _ => {
                continue;
            }
        };

        let msg = Message { from: my_add.clone(), text };
        broadcast(&msg, &peer, None);
    }
}
