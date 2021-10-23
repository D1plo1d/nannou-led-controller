use eyre::{
    eyre,
    // Error,
    Result,
};

pub trait Program where Self: std::fmt::Debug {
    fn update(&mut self, model: &mut crate::Model, frame_index: usize);
    fn receive_osc_packet<'a>(
        &mut self,
        addr: &'a[&'a str],
        args: &'a[nannou_osc::Type],
        frame_index: usize,
    ) -> Result<()>;
}

pub struct ProgramExecutor {
    pub program: Box<dyn Program>,
    pub frame_index: usize,
}

impl ProgramExecutor {
    pub fn new(program: impl Program + 'static) -> Self {
        Self {
            program: Box::new(program),
            frame_index: 0,
        }
    }

    pub fn from_program_name(
        program_name: &str,
        model: &crate::Model
    ) -> Result<Self> {
        match program_name {
            "blink" => Ok(Self::new(crate::programs::Blink::new(model)?)),
            "on" => Ok(Self::new(crate::programs::On::new(model)?)),
            // "pulse" => Ok(Box::new(crate::pulse::Pulse)),
            "scanner" => Ok(Self::new(crate::programs::Scanner::default())),
            // "fireworks" => Ok(Self::Fireworks(Fireworks)),
            "theaterchase" => Ok(Self::new(crate::programs::TheaterChase::default())),
            // "vumeter" => Ok(Self::VUMeter(VUMeter)),
            // "preprogram" => Ok(Self::PreProgram(PreProgram)),
            _ => Err(eyre!("Invalid program name: {}", program_name)),
        }
    }

    pub fn update(&mut self, model: &mut crate::Model) {
        self.program.update(model, self.frame_index);

        self.frame_index = if model.run_forwards {
            self.frame_index.wrapping_add(1)
        } else {
            self.frame_index.wrapping_sub(1)
        }
    }
}
