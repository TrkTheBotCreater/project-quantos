// Kaos Motoru Modülü

pub const SCALE: i128 = 100_000_000;

pub struct ChaosEngine {
    x: i128, y: i128, z: i128, w: i128,
    a: i128, b: i128, k: i128,
}

impl ChaosEngine {
    pub fn new(seed: [u8; 32]) -> Self {
        // ... (Senin 'new' fonksiyonun buraya) ...
        // Seed'den parametreleri türet
        Self { x: 0, y: 0, z: 0, w: 0, a: 176, b: 10, k: 5 } // Örnek
    }

    pub fn step(&mut self) {
        // ... (Senin 'step' fonksiyonunu buraya yapıştır) ...
    }

    pub fn get_byte(&mut self) -> u8 {
        self.step();
        (self.x.abs() % 256) as u8
    }

    pub fn is_chaotic(&mut self) -> bool {
        // ... (Senin Lyapunov Bekçisi kodunu buraya yapıştır) ...
        true
    }
}
