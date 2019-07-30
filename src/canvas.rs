use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use crate::base::*;

pub fn get_context() -> web_sys::CanvasRenderingContext2d {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

pub fn draw_pixel(context: &web_sys::CanvasRenderingContext2d, pixel: Pixel) {
    context.set_fill_style(
        &JsValue::from_str(
            &format!(
                "rgb({}, {}, {})",
                pixel.color.x * 255.99,
                pixel.color.y * 255.99,
                pixel.color.z * 255.99
            )));
    context.fill_rect(pixel.position.x, pixel.position.y, 1.0, 1.0);
}
