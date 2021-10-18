use nannou::prelude::*;
use local_ip_address::local_ip;
use nannou_osc::Packet;
use program::Program;

use crate::{program::program_from_osc_addr};

fn main() {
    nannou::app(model).update(update).run();
}

mod pulse;
mod program;
mod scanner;
mod theater_chase;

pub type LedColor = Hsl<nannou::color::encoding::Srgb>;
pub type LedStripVec = Vec<[LedColor; 150]>;

pub struct Model {
    pub receiver: nannou_osc::Receiver,
    pub led_strips: LedStripVec,
    pub color: crate::LedColor,
    pub color2: crate::LedColor,
    pub fps: f32,
    pub fps_offset: f32,
    pub program: Option<Box<dyn Program>>,
}

// Make sure this matches the `TARGET_PORT` in the `osc_sender.rs` example.
const PORT: u16 = 8000;

fn model(app: &App) -> Model {
    let _w_id = app
        .new_window()
        .title("OSC Receiver")
        .size(1440, 550)
        // .raw_event(raw_window_event)
        .view(view)
        .build()
        .unwrap();

    // Bind an `osc::Receiver` to a port.
    let receiver = nannou_osc::receiver(PORT).unwrap();
    if let Ok(ip_address) = local_ip() {
        println!("Listening for OSC packets at {}:{}\n", ip_address, PORT);
    } else {
        println!("Listening for OSC packets on port {}\n", PORT);
    }

    let led_strips = vec![[crate::LedColor::default(); 150]; 16];
    // led_strips[1][5].blue = 1.0;

    Model {
        receiver,
        led_strips,
        color: nannou::color::rgb(1.0, 0.0, 0.0).into(),
        color2: nannou::color::rgb(0.0, 0.0, 0.0).into(),
        fps: 40.0,
        fps_offset: 0.0,
        program: None,
        // program: Some(Box::new(scanner::Scanner::default())),
    }
}

// fn raw_window_event(app: &App, model: &mut Model, event: &ui::RawWindowEvent) {
//     model.ui.handle_raw_event(app, event);
// }

fn update(_app: &App, model: &mut Model, _update: Update) {

    // Receive any pending osc packets.
    for (packet, _) in model.receiver.try_iter() {
        println!("Received OSC packet: {:?}", packet);
        use nannou_osc::{ Type::*, Message};
        let empty_args = vec![];

        let (addr, args) = match &packet {
            Packet::Message(Message { addr, args }) => {
                (&addr[..], &args.as_ref().unwrap_or(&empty_args)[..])
            }
            _ => {
                println!("Unsupported packet received: {:?}", packet);
                continue;
            },
        };
        match (addr, args) {
            // Hue and Saturation
            ("/variable/color1", [
                Float(hue),
                Float(saturation),
            ]) => {
                model.color = hsl(hue / 255.0, saturation / 255.0, model.color.lightness);
            }
            // Brightness
            ("/variable/value1", [
                Float(lightness),
            ]) => {
                model.color.lightness = lightness / 255.0;
            }
            ("/variable/interval", [
                // Input is between 0 and 255
                Float(input),
            ]) => {
                model.fps = *input;
                // TODO: There is an upstream issue to fix rate mode in Nannou.
                // if model.fps != 0.0 {
                //     app.set_loop_mode(LoopMode::rate_fps(model.fps as f64))
                // }
            }
            (addr, _) => {
                if let Ok(program) = program_from_osc_addr(&addr) {
                    model.program = Some(program);
                } else {
                    println!("Unsupported packet received: {:?}", packet);
                }
            }
        }
        println!("Program: {:?}", model.program);
    }

    if let Some(mut program) = model.program.take() {
        model.fps_offset += model.fps;
        let frames = model.fps_offset as usize / 40;
        model.fps_offset = model.fps_offset % 40.0;

        for _ in 0..frames {
            program.update(model);
        }

        model.program = Some(program);
    }

    if model.fps != 0.0 {
        std::thread::sleep(std::time::Duration::from_secs_f64(1f64 / model.fps as f64));
    }
    // thread::sleep(time::Duration::from_millis(100));
}

const PAGE_MARGIN: f32 = 6.0;

fn view(app: &App, model: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    let win_rec = app.main_window().rect();
    let origin_x = win_rec.left() + PAGE_MARGIN;
    let origin_y = win_rec.top() - PAGE_MARGIN;

    // set background to blue
    draw.background().color(nannou::color::BLACK);

    const LED_BOX_SIZE: f32 = 8.0;
    const LED_BORDER_SIZE: f32 = 0.5;
    const STROKE_WEIGHT: f32 = 0.5;
    const TEXT_HEIGHT: u32 = 14;

    for (strip_index, led_strip) in model.led_strips.iter().enumerate() {
        let offset_y = origin_y - strip_index as f32 * 32.0;

        let text = format!("LED STRIP #{}", strip_index);
        draw.text(&text)
            .color(WHITE)
            .font_size(TEXT_HEIGHT)
            .y(offset_y - (TEXT_HEIGHT as f32) / 2.0)
            .wh(win_rec.wh());

        for (led_index, led_color) in led_strip.iter().enumerate() {
            let x =
                origin_x
                + (LED_BOX_SIZE + LED_BORDER_SIZE * 2.0) * (led_index as f32 + 0.5);
            let y =
                offset_y - (TEXT_HEIGHT as f32) - PAGE_MARGIN
                - (LED_BOX_SIZE + LED_BORDER_SIZE * 2.0) * 0.5;

            draw.rect()
                .x(x)
                .y(y)
                .w_h(LED_BOX_SIZE + LED_BORDER_SIZE * 2.0, LED_BOX_SIZE + LED_BORDER_SIZE * 2.0)
                .stroke(gray(0.7))
                .stroke_weight(STROKE_WEIGHT)
                .color(BLACK);
                // .hsv(1.0, 1.0, 1.0);

            draw.rect()
                .x(x - STROKE_WEIGHT / 2.0)
                .y(y)
                .w_h(LED_BOX_SIZE, LED_BOX_SIZE)
                .color(*led_color);
                // .hsv(1.0, 1.0, 1.0);
        }
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
