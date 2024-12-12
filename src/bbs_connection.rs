use wasm_bindgen::prelude::*;
use web_sys::WebSocket;

#[wasm_bindgen]
pub struct BBSConnection {
    ws: WebSocket,
    terminal: crate::AmigaTerminal,
}

#[wasm_bindgen]
impl BBSConnection {
    #[wasm_bindgen(constructor)]
    pub fn new(terminal: crate::AmigaTerminal, url: &str) -> Result<BBSConnection, JsValue> {
        let ws = WebSocket::new(url)?;
        
        // Set binary type to arraybuffer for raw data
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
        
        // Handle incoming messages
        let onmessage_callback = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
            if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                let array = js_sys::Uint8Array::new(&abuf);
                let data = array.to_vec();
                // Process incoming BBS data
                // TODO: Implement ANSI/ASCII parsing
            }
        }) as Box<dyn FnMut(_)>);
        
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        Ok(BBSConnection { ws, terminal })
    }

    pub fn send(&self, data: &[u8]) -> Result<(), JsValue> {
        self.ws.send_with_u8_array(data)
    }
} 