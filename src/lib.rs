use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

// Add this at the top for logging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct GuruTerminal {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    font_data: Vec<u8>,
    char_width: u32,
    char_height: u32,
}

#[wasm_bindgen]
impl GuruTerminal {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<GuruTerminal, JsValue> {
        console_error_panic_hook::set_once();
        log("Initializing GuruTerminal...");

        let window = window().ok_or_else(|| JsValue::from_str("No window found"))?;
        let document = window.document().ok_or_else(|| JsValue::from_str("No document found"))?;
            
        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or_else(|| JsValue::from_str("Canvas not found"))?
            .dyn_into::<HtmlCanvasElement>()?;

        let ctx = canvas
            .get_context("2d")?
            .ok_or_else(|| JsValue::from_str("Failed to get 2d context"))?
            .dyn_into::<CanvasRenderingContext2d>()?;

        // Set up Amiga font rendering
        ctx.set_font("16px Topaz-8");  // Specify size explicitly
        ctx.set_text_baseline("top");
        log("Canvas context initialized");
        
        // Standard Amiga terminal was 640x256 for PAL
        canvas.set_width(640);
        canvas.set_height(256);

        let terminal = GuruTerminal {
            canvas,
            ctx,
            font_data: Vec::new(),
            char_width: 8,
            char_height: 16,
        };

        // Draw test pattern
        terminal.draw_test_pattern();
        
        Ok(terminal)
    }

    pub fn clear(&self) {
        self.ctx.set_fill_style(&JsValue::from_str("#000000"));
        self.ctx.fill_rect(0.0, 0.0, 640.0, 256.0);
    }

    pub fn write_char(&self, x: u32, y: u32, c: char) {
        self.ctx.set_fill_style(&JsValue::from_str("#00FF00")); // Classic Amiga green
        self.ctx.fill_text(
            &c.to_string(),
            f64::from(x * self.char_width),
            f64::from(y * self.char_height),
        ).unwrap();
    }

    // Add this new method for testing
    pub fn draw_test_pattern(&self) {
        log("Drawing test pattern...");
        
        // Clear screen
        self.clear();

        // Draw border
        self.ctx.set_stroke_style(&JsValue::from_str("#00FF00"));
        self.ctx.stroke_rect(0.0, 0.0, 639.0, 255.0);

        // Draw test text
        self.ctx.set_fill_style(&JsValue::from_str("#00FF00"));
        self.ctx.fill_text("Guru Connect Terminal", 10.0, 10.0).unwrap();
        
        // Draw character test
        for i in 0..32 {
            self.write_char(i, 2, (65 + i) as u8 as char); // Draw ASCII alphabet
        }

        // Draw color test
        let colors = ["#FF0000", "#00FF00", "#0000FF", "#FFFF00"];
        for (i, color) in colors.iter().enumerate() {
            self.ctx.set_fill_style(&JsValue::from_str(color));
            self.ctx.fill_rect(
                (i as f64) * 50.0, 
                100.0, 
                40.0, 
                40.0
            );
        }
    }
} 