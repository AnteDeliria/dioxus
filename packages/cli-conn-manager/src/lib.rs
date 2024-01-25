use axum::extract::ws::{Message as WsMessage, WebSocket};
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::{self, Receiver, Sender};

pub struct Manager {
    socket_sender: SplitSink<WebSocket, WsMessage>,
    channels: Arc<Mutex<HashMap<String, Sender<Message>>>>,
}

impl Manager {
    pub fn new(socket: WebSocket) -> Self {
        let (socket_sender, socket_receiver) = socket.split();

        let channels = Arc::new(Mutex::new(HashMap::new()));
        let mgr = Self {
            socket_sender,
            channels,
        };

        mgr.listen(socket_receiver);
        mgr
    }

    pub fn listen(&self, mut receiver: SplitStream<WebSocket>) {
        let channels = self.channels.clone();

        tokio::spawn(async move {
            let mut stop = false;
            loop {
                while let Some(msg) = receiver.next().await {
                    let msg = match msg {
                        Ok(m) => m,
                        Err(_) => {
                            // Client disconnected.
                            stop = true;
                            break;
                        }
                    };

                    let msg = msg.into_text().unwrap();
                    let data = serde_json::from_str::<Message>(&msg).unwrap();

                    let chs = channels.lock().unwrap();

                    if let Some(sender) = chs.get(&data.channel) {
                        _ = sender.try_send(data);
                    }
                }

                if stop {
                    break;
                }
            }
        });
    }

    pub async fn send(&mut self, channel: String, data: String) -> Result<(), Error> {
        let msg = Message { channel, data };

        let data = match serde_json::to_string(&msg) {
            Ok(d) => d,
            Err(e) => return Err(Error::Serde(e.to_string())),
        };

        let result = self.socket_sender.send(WsMessage::Text(data)).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Send(e.to_string())),
        }
    }

    pub fn recv(&mut self, channel: String) -> Receiver<Message> {
        let (sender, receiver) = mpsc::channel(10);

        let mut chs = self.channels.lock().unwrap();
        chs.insert(channel, sender);
        receiver
    }
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    channel: String,
    data: String,
}

pub enum Error {
    Serde(String),
    Send(String),
    Receive(String),
}
