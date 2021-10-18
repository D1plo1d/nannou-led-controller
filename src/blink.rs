// use nannou::color::{Hsl, IntoColor};

// use crate::program::Program;

// #[derive(Debug)]
// pub struct Blink {
//     // TODO: pallets
//     // pallet: LedColorPallet
// }

// impl Default for Blink {
//     fn default() -> Self {
//         Self {
//         }
//     }
// }

// impl Program for Blink {
//     fn update(&mut self, model: &mut crate::Model) {
//         for led_strip in model.led_strips.iter_mut() {
//             let strip_len = led_strip.len();
//             let leds = led_strip.iter_mut().enumerate();
//             self.update_leds(model.color.clone(), strip_len, leds);
//         }
//     }
// }

// impl Blink {
//     fn update_leds<'a>(
//         &'a mut self,
//         color: Hsl<nannou::color::encoding::Srgb>,
//         led_count: usize,
//         leds: impl Iterator<Item = (usize, &'a mut crate::LedColor)>
//     ) {
//         let scan_index = self.scan_index % (led_count * 2);

//         for (led_index, led_color) in leds {
//             *led_color = if led_index == scan_index {
//                 color.clone()
//             } else if led_index == led_count * 2 - scan_index {
//                 color.clone()
//             } else {
//                 let mut hsl = led_color.into_hsl::<nannou::color::encoding::Srgb>();
//                 hsl.lightness = (hsl.lightness - 1.0 / self.tail_length).max(0.0);
//                 hsl.into()
//             };
//         }
//     }
// }

