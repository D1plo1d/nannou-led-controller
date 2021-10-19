use nannou::{color::{Hsl, IntoColor}};
use eyre::{
    eyre,
    // Error,
    Result,
};
use crate::program::Program;

#[derive(Debug)]
pub struct Scanner {
    scan_index: usize,
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
            scan_index: Default::default(),
            tail_length: 30.0,
            mode: ScannerMode::ParallelStrips,
        }
    }
}

impl Program for Scanner {
    fn update(&mut self, model: &mut crate::Model) {
        match self.mode {
            ScannerMode::ContinuousStrip => {
                let total_led_count = model.led_strips
                    .iter()
                    .fold(0, |sum, led_strip| sum + led_strip.len());
                // let total_led_count = 150;

                let all_leds = model.led_strips
                    .iter_mut()
                    .flat_map(|led_strip| led_strip.iter_mut())
                    .enumerate();

                self.update_leds(model.color.clone(), total_led_count, all_leds);
            }
            ScannerMode::ParallelStrips => {
                for led_strip in model.led_strips.iter_mut() {
                    let strip_len = led_strip.len();
                    let leds = led_strip.iter_mut().enumerate();
                    self.update_leds(model.color.clone(), strip_len, leds);
                }
            }
        };
        // Increment the counter
        self.scan_index = if model.run_forwards {
            self.scan_index.wrapping_add(1)
        } else {
            self.scan_index.wrapping_sub(1)
        }
    }

    fn receive_osc_packet<'a>(&mut self, addr: &'a str, args: &'a[nannou_osc::Type]) -> Result<()> {
        use nannou_osc::Type::*;
        match (addr, args) {
            ("/variable/tail_length", [
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
        leds: impl Iterator<Item = (usize, &'a mut crate::LedColor)>
    ) {
        let scan_index = self.scan_index % (led_count * 2);

        for (led_index, led_color) in leds {
            *led_color = if led_index == scan_index {
                color.clone()
            } else if led_index == led_count * 2 - scan_index {
                color.clone()
            } else {
                let mut hsl = led_color.into_hsl::<nannou::color::encoding::Srgb>();
                hsl.lightness = (hsl.lightness - 1.0 / self.tail_length).max(0.0);
                hsl.into()
            };
        }
    }
}

