mod utils;
use wasm_bindgen::JsCast;
use std::f64;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Flake {
    x: i32,
    y: i32,
}

impl Flake {
    pub fn new(x: i32, y: i32) -> Flake {
        Flake { x, y }
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        context.arc(self.x as f64, self.y as f64, 3.0, 0.0, 2.0 * f64::consts::PI).unwrap();
        context.set_fill_style(&JsValue::from("white"));
        context.fill();
    }

    pub fn tick(&self, context: &web_sys::CanvasRenderingContext2d) {
        self.draw(&context);
        // self.y += 1;
    }
}

#[wasm_bindgen]
pub struct Snow {
    flakes: Vec<Flake>,
}

impl Snow {
    pub fn new() -> Snow {
        let mut flakes: Vec<Flake> = vec![];
        flakes.push(Flake::new(100, 100));

        Snow {
            flakes,
        }
    }

    pub fn tick(&self, context: &web_sys::CanvasRenderingContext2d) {
        for flake in &self.flakes {
            flake.tick(&context);
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    context: web_sys::CanvasRenderingContext2d,
    snow: Snow,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(id: &str) -> Universe {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(id).unwrap();

        let width = canvas.get_attribute("width").unwrap();
        let height = canvas.get_attribute("height").unwrap();

        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Universe {
            width: width.parse::<u32>().unwrap(),
            height: height.parse::<u32>().unwrap(),
            context,
            snow: Snow::new(),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn clear_world(&self) {
        self.context.move_to(0.0, 0.0);
        self.context.set_fill_style(&JsValue::from("black"));
        self.context.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
    }

    pub fn tick(&self) {
        self.context.begin_path();
        self.clear_world();

        self.snow.tick(&self.context);

        self.context.stroke();
    }
}
