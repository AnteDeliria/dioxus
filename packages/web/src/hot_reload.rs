use dioxus_std::conn_manager::ConnectionChannel;
use futures_channel::mpsc::UnboundedReceiver;

use dioxus_core::{prelude::spawn_forever, Template};

pub(crate) fn init(mut channel: ConnectionChannel) -> UnboundedReceiver<Template> {
    use serde::Deserialize;

    let (tx, rx) = futures_channel::mpsc::unbounded();

    spawn_forever(async move {
        loop {
            while let Some(data) = channel.recv().await {
                let val = serde_json::from_str::<serde_json::Value>(&data.data).unwrap();
                let val: &'static serde_json::Value = Box::leak(Box::new(val));
                let template = Template::deserialize(val).unwrap();
                tx.unbounded_send(template).unwrap()
            }
        }
    });

    // change the rsx when new data is received
    /*let cl = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
            let string: String = text.into();
            let val = serde_json::from_str::<serde_json::Value>(&string).unwrap();
            // leak the value
            let val: &'static serde_json::Value = Box::leak(Box::new(val));
            let template: Template = Template::deserialize(val).unwrap();
            tx.unbounded_send(template).unwrap();
        }
    }) as Box<dyn FnMut(MessageEvent)>);*/

    rx
}
