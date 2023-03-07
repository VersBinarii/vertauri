#![feature(async_closure)]
use serde_wasm_bindgen::to_value;
use vertigo::{bind, component, dom, start_app, transaction, DomElement, Value};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
fn App() -> DomElement {
    let message = Value::<String>::default();
    let message_input = Value::<String>::default();
    {
        let message = message.clone();
        let message_inner = message.clone();
        transaction(move |ctx| {
            let msg = message.get(ctx);
            let json_msg = to_value(&msg).unwrap();
            spawn_local(async move {
                let greeting: String = invoke("command", json_msg)
                    .await
                    .as_string()
                    .expect("Failed to invoke command from backend");
                message_inner.set(greeting);
            });
        });
    }
    let greet = bind!(
        message,
        message_input || {
            transaction(|ctx| {
                let msg = message_input.get(ctx);
                message.set(msg);
            })
        }
    );
    let read_input = bind!(message_input, |input_val| {
        message_input.set(input_val);
    });
    dom! {
        <html>
            <head />
            <body>
                <div>
                    <input id="greet-input" value={message.to_computed()} on_input={read_input} placeholder="Enter a name..." />
                    <button on_click={greet}>{"Greet"}</button>
                    <p>{message}</p>
                </div>
            </body>
        </html>
    }
}

#[no_mangle]
pub fn start_application() {
    let count = Value::new(0);

    let view = dom! {
        <App />
    };

    start_app(count, view);
}
