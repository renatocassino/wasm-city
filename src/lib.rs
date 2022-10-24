mod utils;
use wasm_bindgen::JsCast;
use std::f64;
use rand::Rng;

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
    x: f64,
    y: f64,
    size: i32,
    x_velocity: f64,
    y_velocity: f64,
}

impl Flake {
    pub fn new(width: u32, height: u32) -> Flake {
        let mut rng = rand::thread_rng();
        let size: i32 = rng.gen_range(3..6);
        let x: f64 = rng.gen_range(0.0..width as f64);
        let y: f64 = 0.0 - rng.gen_range(10.0..300.0);

        let x_velocity: f64 = rng.gen_range(-0.5..0.5);
        let y_velocity: f64 = rng.gen_range(0.2..1.0);

        Flake { x, y, size, x_velocity, y_velocity }
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        context.begin_path();
        context.arc(self.x, self.y, self.size as f64, 0.0, 2.0 * f64::consts::PI).unwrap();
        context.set_fill_style(&JsValue::from("white"));
        context.fill();
        context.stroke();
    }

    pub fn tick(&mut self, context: &web_sys::CanvasRenderingContext2d) {
        self.draw(&context);
        let mut rng = rand::thread_rng();
        let x_force = rng.gen_range(-0.05..0.05);
        self.x_velocity += x_force;

        self.y += self.y_velocity;
        self.x += self.x_velocity;
    }
}

#[wasm_bindgen]
pub struct Snow {
    flakes: Vec<Flake>,
    width: u32,
    height: u32,
}

impl Snow {
    pub fn new(width: u32, height: u32) -> Snow {
        let mut flakes: Vec<Flake> = vec![];

        for _ in 0..100 {
            flakes.push(Flake::new(width, height));
        }

        Snow {
            flakes,
            width,
            height,
        }
    }

    pub fn tick(&mut self, context: &web_sys::CanvasRenderingContext2d) {
        for flake in &mut self.flakes {
            flake.tick(&context);

            let mut respawn = false;
            if flake.y > self.height as f64 + 10.0 {
                respawn = true;
            }

            if flake.x > self.width as f64 + 10.0 || flake.x < -10.0 {
                respawn = true;
            }

            if respawn {
                *flake = Flake::new(self.width, self.height);
            }
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
            snow: Snow::new(width.parse::<u32>().unwrap(), height.parse::<u32>().unwrap()),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn clear_world(&self) {
        self.context.begin_path();
        self.context.set_fill_style(&JsValue::from("#111"));
        self.context.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
        self.context.stroke();
    }

    pub fn tick(&mut self) {
        self.clear_world();

        self.snow.tick(&self.context);

    }
}
