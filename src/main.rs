use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;
use sysinfo::{System, SystemExt, CpuExt, ComponentExt};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::net::TcpStream;
use std::time::Instant;

// Anomali durumları için struct
#[derive(Debug)]
struct AnomaliDurumu {
    timestamp: String,
    tip: String,
    deger: String,
    mesaj: String,
}

// SSH bağlantı kontrolü için C fonksiyonu
extern "C" {
    fn ssh_baglanti_kontrol(ip: *const libc::c_char, port: libc::c_int) -> libc::c_int;
}

fn main() {
    println!("🚀 Server İzleme Sistemi Başlatılıyor...");
    println!("📡 Hedef Server: 192.168.1.195:22");
    println!("💾 Disk: 1TB SATA");
    println!("=====================================");

    // Ses dosyası kontrolü
    if !std::path::Path::new("warn.mp3").exists() {
        eprintln!("⚠️  Uyarı: warn.mp3 dosyası bulunamadı!");
        eprintln!("📢 Sesli uyarılar çalışmayacak");
    }

    // Htop benzeri arayüzü başlat
    start_htop_interface();

    // Ana izleme döngüsü
    monitor_loop();
}

fn start_htop_interface() {
    // Terminali temizle
    print!("\x1B[2J\x1B[1;1H");
    println!("🖥️  HTOP BENZERİ SERVER İZLEME");
    println!("================================");
}

fn monitor_loop() {
    let mut sys = System::new_all();
    let mut last_alert_time = Instant::now();
    let alert_cooldown = Duration::from_secs(300); // 5 dakika cooldown

    loop {
        sys.refresh_all();

        // Sistem bilgilerini göster
        display_system_info(&sys);

        // Anomali kontrolleri
        let anomaliler = check_anomalies(&sys);

        // Anomali varsa işle
        if !anomaliler.is_empty() && last_alert_time.elapsed() > alert_cooldown {
            for anomali in &anomaliler {
                println!("🚨 ANOMALİ: {} - {}", anomali.tip, anomali.mesaj);
                play_warning_sound();
                // Burada e-posta/telegram bildirimi eklenebilir
            }
            last_alert_time = Instant::now();
        }

        // SSH bağlantı kontrolü
        check_ssh_connection();

        // Disk durumu kontrolü (C fonksiyonu ile)
        check_disk_status();

        // 2 saniye bekle
        thread::sleep(Duration::from_secs(2));

        // Ekranı temizle ve yeniden çiz
        print!("\x1B[2J\x1B[1;1H");
        println!("🖥️  HTOP BENZERİ SERVER İZLEME");
        println!("================================");
    }
}

fn display_system_info(sys: &System) {
    // CPU kullanımı
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let cpu_temp = get_cpu_temperature(sys);

    // RAM kullanımı
    let total_memory = sys.total_memory() / 1024 / 1024;
    let used_memory = sys.used_memory() / 1024 / 1024;
    let memory_usage_percent = (used_memory as f32 / total_memory as f32) * 100.0;

    println!("💻 CPU Kullanımı: {:.1}%", cpu_usage);
    println!("🌡️  CPU Sıcaklığı: {:.1}°C", cpu_temp);
    println!("🧠 RAM: {}/{} MB ({:.1}%)", used_memory, total_memory, memory_usage_percent);
    println!("📦 Toplam Process: {}", sys.processes().len());
    println!("⏰ Uptime: {} saniye", sys.uptime());
    println!("--------------------------------");
}

fn get_cpu_temperature(sys: &System) -> f32 {
    for component in sys.components() {
        if component.label().contains("CPU") || component.label().contains("core") {
            return component.temperature();
        }
    }
    0.0
}

fn check_anomalies(sys: &System) -> Vec<AnomaliDurumu> {
    let mut anomalies = Vec::new();
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // CPU sıcaklık kontrolü
    let cpu_temp = get_cpu_temperature(sys);
    if cpu_temp >= 100.0 {
        anomalies.push(AnomaliDurumu {
            timestamp: now.clone(),
            tip: "CPU_SICAKLIK".to_string(),
            deger: format!("{:.1}°C", cpu_temp),
            mesaj: "CPU sıcaklığı kritik seviyede!".to_string(),
        });
    }

    // RAM kullanım kontrolü
    let memory_usage_percent = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0;
    if memory_usage_percent >= 98.0 {
        anomalies.push(AnomaliDurumu {
            timestamp: now.clone(),
            tip: "RAM_KULLANIM".to_string(),
            deger: format!("{:.1}%", memory_usage_percent),
            mesaj: "RAM kullanımı kritik seviyede!".to_string(),
        });
    }

    // CPU kullanım kontrolü
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    if cpu_usage >= 95.0 {
        anomalies.push(AnomaliDurumu {
            timestamp: now.clone(),
            tip: "CPU_KULLANIM".to_string(),
            deger: format!("{:.1}%", cpu_usage),
            mesaj: "CPU kullanımı kritik seviyede!".to_string(),
        });
    }

    anomalies
}

fn play_warning_sound() {
    if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
        if let Ok(file) = File::open("warn.mp3") {
            let sink = Sink::try_new(&stream_handle).unwrap();
            let source = Decoder::new(BufReader::new(file)).unwrap();
            sink.append(source);
            sink.sleep_until_end();
        }
    }
}

fn check_ssh_connection() {
    let ip = std::ffi::CString::new("192.168.1.195").unwrap();
    let result = unsafe { ssh_baglanti_kontrol(ip.as_ptr(), 22) };

    if result == 0 {
        println!("✅ SSH Bağlantı: Aktif");
    } else {
        println!("❌ SSH Bağlantı: Kapalı");
        play_warning_sound();
    }
}

fn check_disk_status() {
    // C fonksiyonunu çağır
    unsafe {
        disk_durumu_kontrol();
    }
}

// C fonksiyonları için extern
#[link(name = "cdosya")]
extern "C" {
    fn disk_durumu_kontrol();
}