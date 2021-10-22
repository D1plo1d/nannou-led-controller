use nannou::{color::hsl, prelude::ToPrimitive};
use eyre::{
    eyre,
    // Error,
    Result,
};
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
            tail_length: 10,
            pixel_distance: 25,
            mode: TheaterChaseMode::Regular,
        }
    }
}

impl Program for TheaterChase {
    fn update(&mut self, model: &mut crate::Model) {
        let program_index = self.index % (model.total_led_count() * 2);
        let color1 = model.color.clone();
        let color2 = model.color2.clone();

        for (led_index, led_color) in model.all_leds_mut() {
            let distance_to_leading_pixel = (led_index + program_index) % self.pixel_distance;

            *led_color = if distance_to_leading_pixel < self.tail_length {
                // Set the LED hue depending on the mode and distance from the head of the tail
                let hue = match self.mode {
                    TheaterChaseMode::Rainbow => {
                        if distance_to_leading_pixel == 0 {
                            (led_index * 10) as f32
                        } else {
                            (led_index * 10 - (self.pixel_distance - distance_to_leading_pixel * 3 + 1)) as f32
                        }
                    }
                    TheaterChaseMode::Regular => color1.hue.into(),
                };

                hsl(
                    hue % 255.0 / 255.0,
                    color1.saturation,
                    color1.lightness,
                )
            } else {
                color2.clone()
            };
        }

        // Increment the counter
        self.index = if model.run_forwards {
            self.index.wrapping_add(1)
        } else {
            self.index.wrapping_sub(1)
        }
    }

    fn receive_osc_packet<'a>(
        &mut self,
        addr:  &'a[&'a str],
        args: &'a[nannou_osc::Type],
    ) -> Result<()> {
        use nannou_osc::Type::*;
        match (addr, args) {
            (["variable", "chase_mode"], [
                Float(mode_id),
            ]) => {
                self.mode = match mode_id.to_u8() {
                    Some(1u8) => TheaterChaseMode::Regular,
                    Some(2u8) => TheaterChaseMode::Rainbow,
                    _ => return Err(eyre!("Invalid chase mode: {:?}", mode_id)),
                };
            }
            (["variable", "pixel_width"], [
                Float(tail_length),
            ]) => {
                self.tail_length = tail_length
                    .to_usize()
                    .ok_or_else(|| eyre!("Invalid tail_length"))?;
            }
            (["variable", "pixel_distance"], [
                Float(pixel_distance),
            ]) => {
                self.pixel_distance = pixel_distance
                    .to_usize()
                    .ok_or_else(|| eyre!("Invalid pixel_distance"))?;
            }
            _ => {
                return Err(eyre!("Unsupported packet received. addr: {:?} args: {:?}", addr, args))
            }
        };
        Ok(())
    }

}