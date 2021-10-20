use nannou::{color::{Gradient, Hsl, IntoColor, Srgb}, rand};
use eyre::{
    // eyre,
    // Error,
    Result,
};
use crate::program::Program;

#[derive(Debug)]
pub struct Blink {
    index: usize,
    led_gradient_index: Vec<usize>,
    led_next_blink: Vec<usize>,
    gradient: Gradient<Hsl>,
}

impl Default for Blink {
    fn default() -> Self {
        let mut blink = Self {
            index: 0,
            led_gradient_index: vec![0; 64*1024],
            led_next_blink: vec![0; 64*1024],
            gradient: Gradient::with_domain(vec![
                (0.0, Srgb::new(0.0, 0.0, 0.0).into_hsl()),
                (63.0, Srgb::new(239.0 / 255.0, 0.0, 122.0  / 255.0).into_hsl()),
                (191.0, Srgb::new(252.0 / 255.0, 255.0 / 255.0, 78.0 / 255.0).into_hsl()),
                (255.0, Srgb::new(0.0, 0.0, 0.0).into_hsl()),
            ]),
        };

        blink.led_next_blink.fill_with(|| {
            rand::random::<usize>() % 400
        });

        blink
    }
}

impl Program for Blink {
    fn update(&mut self, model: &mut crate::Model) {
        let all_leds = model.led_strips
            .iter_mut()
            .flat_map(|led_strip| led_strip.iter_mut())
            .enumerate();

        self.update_leds(
            all_leds,
        );

        // Increment the counter
        self.index = if model.run_forwards {
            self.index.wrapping_add(1)
        } else {
            self.index.wrapping_sub(1)
        }
    }

    fn receive_osc_packet<'a>(&mut self, addr: &'a str, args: &'a[nannou_osc::Type]) -> Result<()> {
        // use nannou_osc::Type::*;
        // match (addr, args) {
        //     _ => {
        //         return Err(eyre!("Unsupported packet received. addr: {:?} args: {:?}", addr, args))
        //     }
        // };
        Ok(())
    }

}

impl Blink {
    fn update_leds<'a>(
        &'a mut self,
        leds: impl Iterator<Item = (usize, &'a mut crate::LedColor)>
    ) {
        let gradient_size: usize = 255;

        for (led_index, led_color) in leds {
            if self.index >= self.led_next_blink[led_index] {
                let led_gradient_index = &mut self.led_gradient_index[led_index];
                *led_color = self.gradient.get(*led_gradient_index as f32);
                *led_gradient_index += 1;

                if *led_gradient_index > gradient_size {
                    *led_gradient_index = 0;
                    self.led_next_blink[led_index] = self.index + rand::random::<usize>() % 400;
                }
            }
        }
    }
}
