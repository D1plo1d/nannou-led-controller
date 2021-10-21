use eyre::{
    eyre,
    // Error,
    Result,
};

pub trait Program where Self: std::fmt::Debug {
    fn update(&mut self, model: &mut crate::Model);
    fn receive_osc_packet<'a>(&mut self, addr: &'a[&'a str], args: &'a[nannou_osc::Type]) -> Result<()>;
}

pub fn program_from_osc_addr(program_name: &str, model: &crate::Model) -> Result<Box<dyn Program>> {
    match program_name {
        // "pulse" => Ok(Box::new(crate::pulse::Pulse)),
        "scanner" => Ok(Box::new(crate::programs::Scanner::default())),
        // "fireworks" => Ok(Self::Fireworks(Fireworks)),
        "blink" => Ok(Box::new(crate::programs::Blink::new(model)?)),
        "theaterchase" => Ok(Box::new(crate::programs::TheaterChase::default())),
        // "vumeter" => Ok(Self::VUMeter(VUMeter)),
        // "preprogram" => Ok(Self::PreProgram(PreProgram)),
        _ => Err(eyre!("Invalid program name: {}", program_name)),
    }
}
