use std::{io::{self, Read}, net::{TcpListener, TcpStream}, ops::Index, sync::{atomic::AtomicBool, Arc}, thread, time::{Duration, SystemTime, UNIX_EPOCH}};

mod client;
use chess_networking::Ack;
use client::ServerClient;
use viktoe_chess::{board::Turn, prelude::BoardPosition, ChessGame};

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
    turn: Turn,

    clients: Vec<ServerClient>
}

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
            turn: Turn::White,

            clients: vec![]
        }, running))
    }

    pub fn check_for_client(&mut self) -> Option<ServerClient> {
        if let Ok((stream, addr)) = self.listener.accept() {
            stream.set_nonblocking(true).unwrap();
            Some(ServerClient::new(stream, addr))
        } else {
            None
        }
    }

    fn initiation(&mut self) {
        if let (Some(white), Some(black)) = (&mut self.white, &mut self.black) {
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
        if let Some(move_packet) = self.get_current_player().read_move() {
            let from = BoardPosition::try_from(move_packet.from).unwrap();
            let to = BoardPosition::try_from(move_packet.to).unwrap();

            match self.chess.move_piece(&from, &to) {
                Ok(state) => {
                    self.get_current_player().send_ack(Ack { ok: true, end_state: None });
                    self.update_turn();
                    self.get_current_player().send_move(&move_packet);
                    self.get_current_player().read_ack();
                },
                Err(_) => {
                    self.get_current_player().send_ack(Ack { ok: false, end_state: None });
                }
            }

        }
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

    fn get_current_player(&mut self) -> &mut ServerClient {
        match self.turn {
            Turn::White => self.white.as_mut().expect("White player should exist"),
            Turn::Black => self.black.as_mut().expect("Black player should exist")
        }
    }

    fn update_turn(&mut self) {
        self.turn = match self.chess.get_player_turn() {
            Turn::White => Turn::White,
            Turn::Black => Turn::Black,
        }
    }
}
