use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, Document, Event, HtmlButtonElement, HtmlCanvasElement,
    HtmlParagraphElement, MouseEvent, window,
};

fn draw(document: &Document) {
    let canvas = document
        .get_element_by_id("canvas")
        .expect("Should find a #canvas element")
        .dyn_into::<HtmlCanvasElement>()
        .expect("Should be able to convert to HtmlCanvasElement");

    let reset_button = document
        .get_element_by_id("reset_button")
        .expect("Should find a #reset_button element")
        .dyn_into::<HtmlButtonElement>()
        .expect("Should be able to convert to HtmlButtonElement");

    let save_and_next_button = document
        .get_element_by_id("save_and_next_button")
        .expect("Should find a #save_and_next_button element")
        .dyn_into::<HtmlButtonElement>()
        .expect("Should be able to convert to HtmlButtonElement");

    let federate_button = document
        .get_element_by_id("federate_button")
        .expect("Should find a #federate_button element")
        .dyn_into::<HtmlButtonElement>()
        .expect("Should be able to convert to HtmlButtonElement");

    let data_dump = document
        .get_element_by_id("data_dump")
        .expect("Should find a #data_dump element")
        .dyn_into::<HtmlParagraphElement>()
        .expect("Should be able to convert to HtmlParagraphElement");

    let ctx = canvas
        .get_context("2d")
        .expect("Should be able to get 2d context")
        .expect("Should be able to get 2d context")
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("Should be able to convert to canvas rendering context");

    ctx.set_fill_style_str("white");
    ctx.fill_rect(0f64, 0., canvas.width() as f64, canvas.height() as f64);
    ctx.set_stroke_style_str("black");
    ctx.set_line_width(2f64);
    ctx.set_line_cap("value");

    let ctx = Rc::new(ctx);
    let drawing = Rc::new(RefCell::new(false));

    // Mouse Down
    {
        let ctx = Rc::clone(&ctx);
        let drawing = Rc::clone(&drawing);

        let on_mouse_down = Closure::wrap(Box::new(move |event: Event| {
            let event = event
                .dyn_ref::<MouseEvent>()
                .expect("Should be able to convert to MouseEvent");

            *drawing.borrow_mut() = true;
            ctx.begin_path();
            ctx.move_to(event.offset_x() as f64, event.offset_y() as f64);
        }) as Box<dyn FnMut(Event)>);

        canvas
            .add_event_listener_with_callback("mousedown", on_mouse_down.as_ref().unchecked_ref())
            .unwrap();

        on_mouse_down.forget();
    }

    // Mouse Move
    {
        let ctx = Rc::clone(&ctx);
        let drawing = Rc::clone(&drawing);

        let on_mouse_move = Closure::wrap(Box::new(move |event: Event| {
            let event = event
                .dyn_ref::<MouseEvent>()
                .expect("Should be able to convert to MouseEvent");

            if *drawing.borrow() {
                ctx.line_to(event.offset_x() as f64, event.offset_y() as f64);
                ctx.stroke();
            }
        }) as Box<dyn FnMut(Event)>);

        canvas
            .add_event_listener_with_callback("mousemove", on_mouse_move.as_ref().unchecked_ref())
            .unwrap();

        on_mouse_move.forget();
    }

    // Mouse Up
    {
        let drawing = Rc::clone(&drawing);

        let on_mouse_up =
            Closure::wrap(Box::new(move |_event: Event| *drawing.borrow_mut() = false)
                as Box<dyn FnMut(Event)>);

        canvas
            .add_event_listener_with_callback("mouseup", on_mouse_up.as_ref().unchecked_ref())
            .unwrap();

        on_mouse_up.forget();
    }

    // Save & Next Button
    {
        let ctx = Rc::clone(&ctx);

        let on_save_and_next = Closure::wrap(Box::new(move |_event: Event| {
            let image_data = ctx
                .get_image_data(0f64, 0f64, canvas.width() as f64, canvas.height() as f64)
                .expect("Should be able to get image data")
                .data();

            let data_slice = image_data.as_slice();

            let greyscale_values = data_slice.iter().step_by(4).collect::<Vec<_>>();

            let mut string_of_greyscale_data = greyscale_values
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(" ");

            string_of_greyscale_data
                .push_str(format!(" Size: {}", greyscale_values.len().to_string()).as_str());

            data_dump.set_text_content(Some(&string_of_greyscale_data));
        }) as Box<dyn FnMut(Event)>);

        save_and_next_button
            .add_event_listener_with_callback("click", on_save_and_next.as_ref().unchecked_ref())
            .unwrap();

        on_save_and_next.forget();
    }
}

// Test rust-js interop
#[wasm_bindgen]
pub fn rust_main_entry() {
    let window = window().expect("no global `window` exists");

    let document: Document = window.document().expect("Should have a document");

    draw(&document);
}
