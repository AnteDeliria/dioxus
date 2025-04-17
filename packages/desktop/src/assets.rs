use dioxus_core::prelude::Callback;
use rustc_hash::FxHashMap;
use std::{rc::Rc, sync::Mutex};
use wry::{http::Request, RequestAsyncResponder};

/// A request for an asset within dioxus-desktop.
pub type AssetRequest = Request<Vec<u8>>;

pub struct AssetHandler {
    f: Callback<(AssetRequest, RequestAsyncResponder)>,
}

#[derive(Clone)]
pub struct AssetHandlerRegistry {
    handlers: Rc<Mutex<FxHashMap<String, AssetHandler>>>,
}

impl AssetHandlerRegistry {
    pub fn new() -> Self {
        AssetHandlerRegistry {
            handlers: Default::default(),
        }
    }

    pub fn has_handler(&self, name: &str) -> bool {
        self.handlers.lock().unwrap().contains_key(name)
    }

    pub fn handle_request(
        &self,
        name: &str,
        request: AssetRequest,
        responder: RequestAsyncResponder,
    ) {
        if let Some(handler) = self.handlers.lock().unwrap().get(name) {
            // And run the handler in the scope of the component that created it
            handler.f.call((request, responder));
        }
    }

    pub fn register_handler(
        &self,
        name: String,
        f: Callback<(AssetRequest, RequestAsyncResponder)>,
    ) {
        self.handlers
            .lock()
            .unwrap()
            .insert(name, AssetHandler { f });
    }

    pub fn remove_handler(&self, name: &str) -> Option<AssetHandler> {
        self.handlers.lock().unwrap().remove(name)
    }
}
