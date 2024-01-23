use axum::extract::ws::{Message as WsMessage, WebSocket};
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
    TryStreamExt,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct Manager<'a> {
    socket_sender: SplitSink<WebSocket, WsMessage>,
    channels: Arc<Mutex<HashMap<&'a str, Sender<Message<'a>>>>>,
}

impl Manager<'_> {
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
                    let msg = if let Ok(msg) = msg {
                        msg
                    } else {
                        // Client disconnect
                        stop = true;
                        break;
                    };

                    // deserialize msg and send through proper channel
                }

                if stop {
                    break;
                }
            }
        });
    }

    pub async fn send(&mut self, channel: &str, data: String) -> Result<(), Error> {
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

    pub fn recv(&mut self, channel: &str) -> Receiver<String> {
        todo!()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Message<'a> {
    channel: &'a str,
    data: String,
}

pub enum Error {
    Serde(String),
    Send(String),
    Receive(String),
}
