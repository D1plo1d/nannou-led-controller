use nannou::{color::{Hsl}};
use eyre::{
    eyre,
    // Error,
    Result,
};
use crate::program::Program;

#[derive(Debug)]
pub struct Scanner {
    tail_length: f32,
    mode: ScannerMode,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum ScannerMode {
    ContinuousStrip,
    ParallelStrips,
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            tail_length: 30.0,
            mode: ScannerMode::ParallelStrips,
        }
    }
}

impl Program for Scanner {
    fn update(&mut self, model: &mut crate::Model, frame_index: usize) {
        match self.mode {
            ScannerMode::ContinuousStrip => {
                self.update_leds(
                    model.color.clone(),
                    model.total_led_count(),
                    model.all_leds_mut(),
                    frame_index,
                );
            }
            ScannerMode::ParallelStrips => {
                for led_strip in model.led_strips.iter_mut() {
                    let strip_len = led_strip.len();
                    let leds = led_strip.iter_mut().enumerate();
                    self.update_leds(
                        model.color.clone(),
                        strip_len,
                        leds,
                        frame_index,
                    );
                }
            }
        };
    }

    fn receive_osc_packet<'a>(
        &mut self,
        addr:  &'a[&'a str],
        args: &'a[nannou_osc::Type],
        _frame_index: usize,
    ) -> Result<()> {
        use nannou_osc::Type::*;
        match (addr, args) {
            (["variable", "tail_length"], [
                Float(tail_length),
            ]) => {
                self.tail_length = *tail_length;
            }
            _ => {
                return Err(eyre!("Unsupported packet received. addr: {:?} args: {:?}", addr, args))
            }
        };
        Ok(())
    }
}

impl Scanner {
    fn update_leds<'a>(
        &'a mut self,
        color: Hsl<nannou::color::encoding::Srgb>,
        led_count: usize,
        leds: impl Iterator<Item = (usize, &'a mut crate::LedColor)>,
        frame_index: usize,
    ) {
        let frame_index = frame_index % (led_count * 2);

        for (led_index, led_color) in leds {
            *led_color = if led_index == frame_index {
                color.clone()
            } else if led_index == led_count * 2 - frame_index {
                color.clone()
            } else {
                led_color.lightness = (led_color.lightness - 1.0 / self.tail_length).max(0.0);
                *led_color
            };
        }
    }
}

