use eyre::{
    eyre,
    // Error,
    Result,
};

pub trait Program where Self: std::fmt::Debug {
    fn update(&mut self, model: &mut crate::Model);
}

pub fn program_from_osc_addr(s: &str) -> Result<Box<dyn Program>> {
    match s {
        // "/program/pulse" => Ok(Box::new(crate::pulse::Pulse)),
        "/program/scanner" => Ok(Box::new(crate::scanner::Scanner::default())),
        // "/program/fireworks" => Ok(Self::Fireworks(Fireworks)),
        // "/program/blink" => Ok(Self::Blink(Blink)),
        // "/program/theaterchase" => Ok(Self::TheaterChase(TheaterChase)),
        // "/program/vumeter" => Ok(Self::VUMeter(VUMeter)),
        // "/program/preprogram" => Ok(Self::PreProgram(PreProgram)),
        _ => Err(eyre!("Invalid program string")),
    }
}
