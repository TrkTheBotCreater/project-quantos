// =================================================================================
// 🌪️ KAOS MOTORU (SAFE & WRAPPING)
// =================================================================================
pub mod chaos {
    // Büyük katsayılar Hénon haritasının i128 sınırlarını zorlamasına neden olur.
    // 'wrapping_' fonksiyonları ile Debug modunda panik almadan çalışmasını sağlıyoruz.
    const SCALE: i128 = 100_000_000;
    const A_PARAM: i128 = 140_000_000;
    const B_PARAM: i128 = 30_000_000;
    const K_PARAM: i128 = 5_000_000;

    #[derive(Debug, Clone)]
    pub struct ChaosEngine {
        pub x: i128, pub y: i128, pub z: i128, pub w: i128,
    }

    impl ChaosEngine {
        pub fn new(seed_val: i128) -> Self {
            Self {
                x: seed_val, 
                y: seed_val.wrapping_add(SCALE), 
                z: seed_val.wrapping_sub(SCALE), 
                w: seed_val.wrapping_add(SCALE / 2),
            }
        }
        pub fn varsayilan() -> Self {
            Self::new(35_000_000)
        }
        fn step(&mut self) {
            // FIX: Tüm matematiksel işlemler 'wrapping' yapısına geçirildi.
            // Bu sayede sayı i128 sınırını aşsa bile panik vermez, başa sarar.
            // Bu davranış kaos üretimi için uygundur (Non-linearite artar).
            
            let y_sq = (self.y.wrapping_mul(self.y)).wrapping_div(SCALE);
            let b_z = (B_PARAM.wrapping_mul(self.z)).wrapping_div(SCALE);
            let k_w = (K_PARAM.wrapping_mul(self.w)).wrapping_div(SCALE);
            
            let term1 = A_PARAM.wrapping_sub(y_sq);
            let term2 = term1.wrapping_sub(b_z);
            let x_new = term2.wrapping_add(k_w);
            
            self.w = self.z; 
            self.z = self.y; 
            self.y = self.x; 
            self.x = x_new;
        }
        pub fn get_next_byte(&mut self) -> u8 {
            self.step();
            let mix = self.x ^ self.y.rotate_left(15) ^ self.z.rotate_left(30) ^ self.w.rotate_left(45);
            (mix & 0xFF) as u8
        }
    }
}
