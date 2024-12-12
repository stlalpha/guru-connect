use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn setup_keyboard(terminal: &AmigaTerminal) {
    let onkeydown = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        // Prevent default browser behavior
        event.prevent_default();
        
        // Convert modern keyboard input to Amiga scancodes
        let key = event.key();
        match key.as_str() {
            "Enter" => {
                // Send CR/LF
                // terminal.send(&[13, 10]);
            }
            // Add more key mappings
            _ => {
                if key.len() == 1 {
                    // Send ASCII character
                    // terminal.send(&[key.bytes().next().unwrap()]);
                }
            }
        }
    }) as Box<dyn FnMut(_)>);

    web_sys::window()
        .unwrap()
        .add_event_listener_with_callback(
            "keydown",
            onkeydown.as_ref().unchecked_ref(),
        )
        .unwrap();
    
    onkeydown.forget();
} 