/*
 * Created on Tue Oct 27 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

extern crate hidapi;
extern crate enigo;

pub mod shared_data;

pub use shared_data::SharedData;
use tungstenite::{HandshakeError, Message, WebSocket, server};

use std::{io, net::TcpListener, net::TcpStream, sync::Arc, sync::RwLock, thread::JoinHandle, net::SocketAddr, thread, time::Duration};
use crate::{config::ConfigFile, command::ReqCommand, command::ReqCommands, command::ResCommand, command::ResCommands, device, tablet_handler::TabletHandler};

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

    pub fn new(port: u16, device: device::Device, config_file: ConfigFile) -> Result<Self, StoryTabletError> {
        let shared_data = Arc::new(RwLock::new(SharedData::new(device, config_file)));

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
                        match server::accept(stream) {
                            Err(HandshakeError::Interrupted(_)) => {
                            }

                            Err(_) => {
                                println!("Error while making connection from {}", addr);
                            }

                            Ok(socket) => {
                                connection.push((addr, socket));
                                println!("Connected from {}", addr);
                            }
                        }
                        
                    }
                }
            }

            connection.retain(move |(addr, socket)| {
                if !socket.can_read() {
                    println!("{} disconnected", addr);
                    return false;
                }

                true
            });

            for (_, socket) in connection.iter_mut() {
                match socket.read_message() {
                    Err(tungstenite::Error::Io(err)) if err.kind() == io::ErrorKind::WouldBlock => {

                    }

                    Err(_) => {
                        continue;
                    }       

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
        match command.data {
            ReqCommands::GetConfig { } => {
                Self::send_response(socket, ResCommand { id: command.id, data: ResCommands::GetConfig { config: self.shared.read().unwrap().config().clone() } });
            }

            ReqCommands::UpdateConfig { config } => {
                let mut shared = self.shared.write().unwrap();
                let config_file = shared.get_config_file_mut();

                config_file.set_config(config);
                println!("Config updated");

                Self::send_response(socket, ResCommand { id: command.id, data: ResCommands::UpdateConfig { updated: true } });
            }

            ReqCommands::SaveConfig { force_write } => {
                let mut shared = self.shared.write().unwrap();
                let config_file = shared.get_config_file_mut();

                let mut saved = true;
                let mut file_changed = false;
                if force_write || config_file.changed() {
                    let write_res = config_file.save_to_file(true);

                    if write_res.is_err() {
                        saved = false;
                        println!("Error while writing config: {:?}", write_res.err().unwrap());
                    } else {
                        file_changed = true;
                        println!("Config saved");
                    }
                }

                Self::send_response(socket, ResCommand { id: command.id, data: ResCommands::SaveConfig { saved, file_changed } });
            }

            ReqCommands::GetStatus { } => {
                Self::send_response(socket, ResCommand { id: command.id, data: ResCommands::GetStatus { status: self.tablet_handler.get_status() } });
            }

            ReqCommands::GetDevice { } => {
                Self::send_response(socket, ResCommand { id: command.id, data: ResCommands::GetDevice { device: self.shared.read().unwrap().device().clone() } });
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
