mod chaos_engine;
mod mq_system;

use mq_system::QuantosKEM;
use std::time::Instant;

fn main() {
    println!("==================================================");
    println!("   PROJECT QUANTOS: Hyperchaotic MQ-KEM System    ");
    println!("==================================================");

    // 1. KeyGen
    let start = Instant::now();
    println!("\n[1] Anahtarlar Üretiliyor (Chaos + MQ)...");
    let seed = [0u8; 32]; // Gerçek hayatta rastgele olmalı
    let kem = QuantosKEM::keygen(seed);
    println!(" -> Süre: {:.2?}", start.elapsed());

    // 2. Encapsulation (Alice)
    println!("\n[2] Alice: Şifreleme ve Kapsülleme Yapıyor...");
    let (ciphertext, shared_secret_alice) = kem.encapsulate();
    println!(" -> Ciphertext Boyutu: {} bytes", ciphertext.len());
    println!(" -> Alice'in Paylaşılan Anahtarı (Hash): {:02x?}", &shared_secret_alice[0..4]);

    // 3. Decapsulation (Bob)
    println!("\n[3] Bob: Şifre Çözüyor ve Anahtarı Açıyor...");
    let start_dec = Instant::now();
    let shared_secret_bob = kem.decapsulate(&ciphertext);
    println!(" -> Süre: {:.2?}", start_dec.elapsed());
    println!(" -> Bob'un Paylaşılan Anahtarı (Hash): {:02x?}", &shared_secret_bob[0..4]);

    // 4. Sonuç
    if shared_secret_alice == shared_secret_bob {
        println!("\n✅ BAŞARILI: Kuantum Dirençli Tünel Kuruldu!");
    } else {
        println!("\n❌ HATA: Anahtarlar uyuşmadı!");
    }
}
