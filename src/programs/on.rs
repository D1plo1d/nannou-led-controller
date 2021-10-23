use eyre::{
    eyre,
    // Error,
    Result,
};
use nannou::prelude::ToPrimitive;
use crate::program::Program;

/// Turns all the LEDs on for a period and then fades to black
#[derive(Debug)]
pub struct On {
    first_frame: usize,
    ticks_until_start_of_fade: usize,
    fade_ticks: usize,
}

impl On {
    pub fn new(_model: &crate::Model) -> Result<Self> {
        let on = Self {
            first_frame: 0,
            ticks_until_start_of_fade: 5,
            fade_ticks: 20,
        };

        Ok(on)
    }
}

impl Program for On {
    fn update(&mut self, model: &mut crate::Model, frame_index: usize) {
        let frame_index = frame_index - self.first_frame;

        if frame_index == 0 {
            // Turn all the LEDs on
            let color = model.color.clone();

            for (_, led_color) in model.all_leds_mut() {
                *led_color = color.clone();
            }
        }
        if frame_index >= self.ticks_until_start_of_fade {
            // Fade all the LEDs out
            for (_, led_color) in model.all_leds_mut() {
                led_color.lightness = (
                    led_color.lightness - 1.0 / (self.fade_ticks as f32)
                ).max(0.0);
            }
        }
    }

    fn receive_osc_packet<'a>(
        &mut self,
        addr:  &'a[&'a str],
        args: &'a[nannou_osc::Type],
        frame_index: usize,
    ) -> Result<()> {
        use nannou_osc::Type::*;

        match (addr, args) {
            (["1", "fader1"], [
                Float(value),
            ]) => {
                self.ticks_until_start_of_fade = (*value * 40.0)
                    .to_usize()
                    .ok_or_else(|| eyre!("Invalid ticks_until_start_of_fade"))?;
            }
            (["1", "fader2"], [
                Float(value),
            ]) => {
                self.fade_ticks = (*value * 40.0)
                    .to_usize()
                    .ok_or_else(|| eyre!("Invalid fade_ticks"))?;
            }
            (["1", "push2"], _) => {
                self.first_frame = frame_index;
            }
            _ => {
                return Err(eyre!("Unsupported packet received. addr: {:?} args: {:?}", addr, args))
            }
        };

        Ok(())
    }
}
