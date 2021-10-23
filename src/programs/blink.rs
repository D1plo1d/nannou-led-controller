use nannou::{color::{Gradient, Hsl}, prelude::ToPrimitive, rand};
use eyre::{
    eyre,
    // Error,
    Result,
};
use crate::program::Program;

#[derive(Debug)]
pub struct Blink {
    max_ticks_until_blink: usize,
    led_next_blink: Vec<usize>,
    gradient: Gradient<Hsl>,
}

impl Blink {
    pub fn new(model: &crate::Model) -> Result<Self> {
        let max_ticks_until_blink = 400;

        let mut blink = Self {
            max_ticks_until_blink,
            led_next_blink: vec![0; model.total_led_count()],
            gradient: crate::svg_palette::to_gradient("bhw1_14").unwrap(),
        };

        blink.led_next_blink.fill_with(|| {
            rand::random::<usize>() % max_ticks_until_blink
        });

        Ok(blink)
    }
}

impl Program for Blink {
    fn update(&mut self, model: &mut crate::Model, frame_index: usize) {
        let gradient_size: usize = 255;

        for (led_index, led_color) in model.all_leds_mut() {
            if frame_index >= self.led_next_blink[led_index] {
                let led_gradient_index = frame_index - self.led_next_blink[led_index];
                *led_color = self.gradient.get(led_gradient_index as f32);

                if led_gradient_index == gradient_size {
                    let next_blink_offset = if self.max_ticks_until_blink == 0 {
                        0
                    } else {
                        rand::random::<usize>() % self.max_ticks_until_blink
                    };

                    self.led_next_blink[led_index] = frame_index + next_blink_offset;
                }
            }
        }
    }

    fn receive_osc_packet<'a>(
        &mut self,
        addr:  &'a[&'a str],
        args: &'a[nannou_osc::Type],
        _frame_index: usize,
    ) -> Result<()> {
        use nannou_osc::Type::*;
        match (addr, args) {
            (["variable", "blinkrandomtime"], [
                Float(max_seconds_until_blink),
            ]) => {
                // ticks are calculated at 40 fps (but can be scaled by the program speed)
                self.max_ticks_until_blink = (max_seconds_until_blink * 40.0 / 1000.0)
                    .to_usize()
                    .ok_or_else(|| eyre!("Invalid tail_length"))?;
            },
            (["palette", palette_name], _) => {
                self.gradient = crate::svg_palette::to_gradient(palette_name)?;
            }
            _ => {
                return Err(eyre!("Unsupported packet received. addr: {:?} args: {:?}", addr, args))
            }
        };
        Ok(())
    }
}
