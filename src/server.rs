use std::{io::{self, Read}, net::{TcpListener, TcpStream}, ops::Index, sync::{atomic::AtomicBool, Arc}, thread, time::{Duration, SystemTime, UNIX_EPOCH}};

mod client;
use client::ServerClient;
use viktoe_chess::ChessGame;

pub enum ServerState {
    GameInitiation,
    Playing,
    Ended,
}

pub struct Server {
    running: Arc<AtomicBool>,
    listener: TcpListener,
    port: u16,
    state: ServerState,

    chess: ChessGame,
    white: Option<ServerClient>,
    black: Option<ServerClient>,

    clients: Vec<ServerClient>
}

const BUF : [u8; 1024 ] = [0; 1024];

impl Server {
    pub fn init(port: u16) -> std::io::Result<(Self, Arc<AtomicBool>)> {
        let running = Arc::new(AtomicBool::new(true));
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
        listener.set_nonblocking(true).unwrap();

        Ok((Server {
            running: running.clone(),
            listener,
            port,
            state: ServerState::GameInitiation,

            chess: ChessGame::default(),
            white: None,
            black: None,

            clients: vec![]
        }, running))
    }

    pub fn check_for_client(&mut self) -> Option<ServerClient> {
        if let Ok((stream, addr)) = self.listener.accept() {
            Some(ServerClient::new(stream, addr))
        } else {
            None
        }
    }

    fn initiation(&mut self) {
        if let (Some(white), Some(black)) = (&mut self.white, &mut self.black) {
            println!("Switch");
            self.clients.clear();
            self.state = ServerState::Playing;
            white.send_start();
            black.send_start();
            return;
        }

        if let Some(client) = self.check_for_client() {
            self.clients.push(client);
        }
        
        for client in &mut self.clients {
            if client.is_established() {
                continue;
            }

            if let Some(mut start) = client.read_start() {
                let white_occupied = self.white.is_some();
                let black_occupied  = self.black.is_some();
                start.fen = None;
                start.time = None;
                start.inc = None;

                if (start.is_white && !white_occupied) || (!start.is_white && black_occupied) {
                    start.is_white = true;
                    self.white = Some(client.fill(start));
                } else  {
                    start.is_white = false;
                    self.black = Some(client.fill(start));
                }
            }
        }
    }

    fn play(&mut self) {

    }

    fn end(&mut self) {

    }

    pub fn start(&mut self) {
        // let mut last = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        while self.running.load(std::sync::atomic::Ordering::SeqCst) {
            // let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            // println!("Time: {}us", (now - last).as_micros());
            // last = now;
            match self.state {
                ServerState::GameInitiation => self.initiation(),
                ServerState::Playing => self.play(),
                ServerState::Ended => self.end()
            }
            
            thread::sleep(Duration::from_millis(1));
        }

        println!("Server stopped");
    }
}
