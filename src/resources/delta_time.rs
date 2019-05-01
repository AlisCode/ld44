use quicksilver::prelude::Window;

#[derive(Debug, Default)]
pub struct DeltaTime {
    pub delta: f32,
}

impl DeltaTime {
    /// Ticks the DeltaTime
    pub fn tick(&mut self, window: &Window) {
        self.delta = 1. / window.current_fps() as f32;

        // default case
        if !self.delta.is_finite() {
            self.delta = 1. / 60.;
        }
    }
}
