struct HighPass {
    r: f32,
    c: f32,
    q: f32
}

impl HighPass {
    fn new() -> Self {
        todo!()
    }
    
    fn next_value(&mut self, value: f32, time: f32) -> f32 {
        let q_inf = self.c * value;
        let time_constant = self.r * self.c;
        let circuit = time / time_constant;
        
        self.q = self.q + (q_inf - self.q) * (-circuit).exp();
        
        value - self.q / self.c
    }
}

struct LowPass {
    r: f32,
    c: f32,
    q: f32
}

impl LowPass {
    fn next_value(&mut self, value: f32, time: f32) -> f32 {
        let q_inf = self.c * value;
        let time_constant = self.r * self.c;
        let circuit = time / time_constant;

        self.q = self.q + (q_inf - self.q) * (-circuit).exp();

        self.q / self.c
    }
}

pub(crate) struct BandPass {
    highPass: HighPass,
    lowPass: LowPass
}

impl BandPass {
    fn next_value(&mut self, value: f32, time: f32) -> f32 {
        let val1 = self.lowPass.next_value(value, time);
        self.highPass.next_value(val1, time)
    }
}