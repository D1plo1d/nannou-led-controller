use nannou::color::{Hsl, IntoColor, hsl};

use crate::program::Program;

#[derive(Debug)]
pub struct TheaterChase {
    index: usize,
    tail_length: usize,
    pixel_distance: usize,
    mode: TheaterChaseMode,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum TheaterChaseMode {
    Rainbow,
    Regular,
}

impl Default for TheaterChase {
    fn default() -> Self {
        Self {
            index: Default::default(),
            tail_length: 30,
            pixel_distance: 25,
            mode: TheaterChaseMode::Regular,
        }
    }
}

impl Program for TheaterChase {
    fn update(&mut self, model: &mut crate::Model) {
        match self.mode {
            TheaterChaseMode::Rainbow => {
                unimplemented!();
                // let total_led_count = model.led_strips
                //     .iter()
                //     .fold(0, |sum, led_strip| sum + led_strip.len());
                // // let total_led_count = 150;

                // let all_leds = model.led_strips
                //     .iter_mut()
                //     .flat_map(|led_strip| led_strip.iter_mut())
                //     .enumerate();

                // self.update_leds(model.color.clone(), total_led_count, all_leds);
            }
            TheaterChaseMode::Regular => {
                for led_strip in model.led_strips.iter_mut() {
                    let strip_len = led_strip.len();
                    let leds = led_strip.iter_mut().enumerate();
                    self.update_leds(
                        model.color.clone(),
                        model.color2.clone(),
                        strip_len,
                        leds
                    );
                }
            }
        };
        // Increment the counter
        self.index = self.index.wrapping_add(1);
    }
}

impl TheaterChase {
    fn update_leds<'a>(
        &'a mut self,
        color1: Hsl<nannou::color::encoding::Srgb>,
        color2: Hsl<nannou::color::encoding::Srgb>,
        led_count: usize,
        leds: impl Iterator<Item = (usize, &'a mut crate::LedColor)>
    ) {
        let program_index = self.index % (led_count * 2);

        for (led_index, led_color) in leds {
            let tail_distance = (led_index + program_index) % self.pixel_distance;

            *led_color = if tail_distance == 0 {
                hsl(
                    (led_index as f32 * 10.0) % 255.0 / 255.0,
                    color1.saturation,
                    color1.lightness,
                )
            } else if tail_distance < self.tail_length {
                hsl(
                    ((led_index as f32 * 10.0 - (self.pixel_distance - tail_distance * 3 + 1) as f32)) % 255.0 / 255.0,
                    color1.saturation,
                    color1.lightness,
                )
            } else {
                color2.clone()
            };
        }
    }
}

