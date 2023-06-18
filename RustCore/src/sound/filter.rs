
static TWO_PI: f32 = 2.0 * std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub(crate) struct BandPass {
    high_pass: HighPass,
    low_pass: LowPass
}

impl BandPass {
    pub(crate) fn new(frequency: f32, amplitude: f32, range: f32) -> Self {
        Self {
            high_pass: HighPass::new(frequency, amplitude, range),
            low_pass: LowPass::new(frequency, amplitude, range)
        }
    }

    pub(super) fn next_value(&mut self, value: f32, time: f32) -> f32 {
        let val1 = self.low_pass.next_value(value, time);
        self.high_pass.next_value(val1, time)
    }

    pub(crate) fn change_to(&mut self, filter: BandPass) {
        self.high_pass.change_to(filter.high_pass);
        self.low_pass.change_to(filter.low_pass);
    }
}

#[derive(Debug, Clone, Copy)]
struct HighPass {
    r: f32,
    c: f32,
    q: f32
}

impl HighPass {
    fn new(frequency: f32, amplitude: f32, range: f32) -> Self {
        let ex_frequency = frequency / range;

        let rc = 1.0 / TWO_PI / ex_frequency;
        let impedance = 1.0 / amplitude.powf(3.0);

        let r = impedance / 2f32.sqrt();
        let c = rc / r;

        Self { r, c, q: 0.0 }
    }
    
    fn next_value(&mut self, value: f32, time: f32) -> f32 {
        let q_inf = self.c * value;
        let time_constant = self.r * self.c;
        let circuit = time / time_constant;
        
        self.q = self.q + (q_inf - self.q) * (1.0 - (-circuit).exp());
        
        value - self.q / self.c
    }

    pub(crate) fn change_to(&mut self, filter: HighPass) {
        self.r = filter.r;
        self.c = filter.c;
    }
}

#[derive(Debug, Clone, Copy)]
struct LowPass {
    r: f32,
    c: f32,
    q: f32
}

impl LowPass {
    fn new(frequency: f32, amplitude: f32, range: f32) -> Self {
        let ex_frequency = frequency * range;

        let rc = 1.0 / TWO_PI / ex_frequency;
        let impedance = 1.0 / amplitude.sqrt();

        let r = impedance / 2f32.sqrt();
        let c = rc / r;

        Self { r, c, q: 0.0 }
    }

    fn next_value(&mut self, value: f32, time: f32) -> f32 {
        let q_inf = self.c * value;
        let time_constant = self.r * self.c;
        let circuit = time / time_constant;

        self.q = self.q + (q_inf - self.q) * (1.0 - (-circuit).exp());

        self.q / self.c
    }

    pub(crate) fn change_to(&mut self, filter: LowPass) {
        self.r = filter.r;
        self.c = filter.c;
    }
}