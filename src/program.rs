use eyre::{
    // eyre,
    // Error,
    Result,
};

pub trait Program where Self: std::fmt::Debug {
    fn update(&mut self, model: &mut crate::Model);
    fn receive_osc_packet<'a>(&mut self, addr: &'a str, args: &'a[nannou_osc::Type]) -> Result<()>;
}

pub fn program_from_osc_addr(s: &str) -> Option<Box<dyn Program>> {
    match s {
        // "/program/pulse" => Ok(Box::new(crate::pulse::Pulse)),
        "/program/scanner" => Some(Box::new(crate::scanner::Scanner::default())),
        // "/program/fireworks" => Some(Self::Fireworks(Fireworks)),
        // "/program/blink" => Some(Self::Blink(Blink)),
        "/program/theaterchase" => Some(Box::new(crate::theater_chase::TheaterChase::default())),
        // "/program/vumeter" => Some(Self::VUMeter(VUMeter)),
        // "/program/preprogram" => Some(Self::PreProgram(PreProgram)),
        _ => None,
    }
}
