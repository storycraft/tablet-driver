/*
 * Created on Tue Oct 27 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

extern crate hidapi;
extern crate enigo;

pub mod shared_data;

pub use shared_data::SharedData;
use tungstenite::{Message, WebSocket, server};

use std::{io, net::TcpListener, net::TcpStream, sync::Arc, sync::RwLock, thread::JoinHandle, net::SocketAddr, thread, time::Duration};
use crate::{command::ReqCommand, command::ResCommand, config::Config, device, tablet_handler::TabletHandler};

#[derive(Debug)]
pub enum StoryTabletError {

    NotStarted, AlreadyStarted

}
pub struct StoryTablet {

    server: TcpListener,

    started: bool,
    shared: Arc<RwLock<SharedData>>,

    tablet_handler: Arc<TabletHandler>

}

impl StoryTablet {

    pub fn new(port: u16, device: device::Device, config: Config) -> Result<Self, StoryTabletError> {
        let shared_data = Arc::new(RwLock::new(SharedData::new(device, config)));

        Ok(Self {
            server: TcpListener::bind(("127.0.0.1", port)).unwrap(),

            started: false,
            shared: Arc::clone(&shared_data),

            tablet_handler: Arc::new(TabletHandler::new(shared_data.clone()))
        })
    }
    
    fn create_handle<F>(&self, func: F) -> JoinHandle<()>
    where F: Fn(), F: Send + 'static {
        thread::spawn(move || func())
    }

    pub fn start(mut self) -> Result<(), StoryTabletError> {
        if self.started {
            return Err(StoryTabletError::AlreadyStarted);
        }
        self.started = true;

        let inner_handler = self.tablet_handler.clone();
        let input_handle = self.create_handle(move || {
            inner_handler.start();
        });
        println!("Input thread started. Id: {:?}", input_handle.thread().id());

        println!("Driver started");

        self.server.set_nonblocking(true).expect("Cannot set non-blocking");
        self.listen_connection();

        let tablet_handler = self.tablet_handler.clone();
        if tablet_handler.running() {
            tablet_handler.stop();
        }
        input_handle.join().expect("Input thread already killed");

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), StoryTabletError> {
        if !self.started {
            return Err(StoryTabletError::NotStarted);
        }
        self.started = false;
        
        Ok(())
    }

    pub fn started(&self) -> bool {
        self.started
    }

    fn listen_connection(&mut self) {
        let mut connection: Vec<(SocketAddr, WebSocket<TcpStream>)> = Vec::with_capacity(1);
        while self.started {
            match self.server.accept() {
                Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                    
                }

                Err(err) => {
                    panic!("Cannot receive incoming connection. Error: {}", err);
                }

                Ok((stream, addr)) => {
                    // Only accepts local connection
                    if stream.local_addr().unwrap().ip() == addr.ip() {
                        connection.push((addr, server::accept(stream).unwrap()));
                        println!("Connected from {}", addr);
                    }
                }
            }

            for (_, socket) in connection.iter_mut() {
                match socket.read_message() {
                    Err(_) => { continue; }
        
                    Ok(message) => {
                        self.handle_socket(socket, message);
                    }
                }
            }

            thread::sleep(Duration::from_millis(1));
        }

        for (addr, mut socket) in connection {
            let closing = socket.close(None);
            if closing.is_err() {
                println!("Error while closing socket: {}", closing.err().unwrap());
            }

            println!("{} disconnected", addr);
        }
        
    }

    fn handle_socket(&mut self, socket: &mut WebSocket<TcpStream>, message: Message) {
        if !message.is_text() {
            return;
        }
        
        match serde_json::from_str::<ReqCommand>(message.to_text().unwrap()) {
            Err(err) => {
                println!("Unknown message received by client: {}", err);
            }

            Ok(req) => {
                self.handle_command(socket, req);
            }
        }
    }

    fn handle_command(&mut self, socket: &mut WebSocket<TcpStream>, command: ReqCommand) {
        match command {

            ReqCommand::Stop { } => {
                let stop_res = self.stop();

                if stop_res.is_err() {
                    println!("Cannot stop driver: {:?}", stop_res.err().unwrap());

                    Self::send_response(socket, ResCommand::Stop { stopping: false });
                } else {
                    Self::send_response(socket, ResCommand::Stop { stopping: true });
                }
            }

            ReqCommand::GetConfig { } => {
                Self::send_response(socket, ResCommand::GetConfig { config: self.shared.read().unwrap().get_config().clone() });
            }

            ReqCommand::UpdateConfig { config } => {
                self.shared.write().unwrap().set_config(config);
                println!("Config updated");
                Self::send_response(socket, ResCommand::UpdateConfig { updated: true });
            }

            ReqCommand::GetStatus { } => {
                Self::send_response(socket, ResCommand::GetStatus { status: self.tablet_handler.get_status() });
            }

            _ => {
                
            }
        }
        
    }

    fn send_response(socket: &mut WebSocket<TcpStream>, res: ResCommand) {
        let written = socket.write_message(Message::Text(serde_json::to_string(&res).unwrap()));

        if written.is_err() {
            println!("Cannot write response: {}", written.err().unwrap());
        }
    }
}
