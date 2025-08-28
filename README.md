# 🚀 Server Monitoring System - Rust + C

> Real-time server monitoring and anomaly detection system

![Rust](https://img.shields.io/badge/Rust-1.70%252B-orange?logo=rust)
![C](https://img.shields.io/badge/C-11%252B-blue?logo=c)
![License](https://img.shields.io/badge/License-MIT-green)
![Platform](https://img.shields.io/badge/Platform-Linux-lightgrey)

---

## 📋 Features

### 🔍 System Monitoring

- **CPU Usage** – Real-time monitoring (>95% alert)
- **CPU Temperature** – 100°C+ critical alerts
- **RAM Usage** – 98%+ critical alerts
- **Disk Status** – 1TB SATA monitoring (>95% alerts)
- **Process Count** – Active process tracking

### 🌐 Network Monitoring

- **SSH Connection** – `192.168.1.195:22` connection monitoring
- **Port Status** – Real-time port availability
- **Connection Drops** – Instant alert system

### ⚠️ Anomaly Detection

- **Kernel Panic** – System crash detection
- **High Resource Usage** – Critical level alerts
- **Audio Alerts** – Instant `warn.mp3` notifications
- **Visual Alerts** – Colored terminal warnings

### 🎨 Interface

- **Htop-like** – User-friendly terminal interface
- **Real-time** – 2-second refresh rate
- **Color Indicators** – Visual feedback with status colors

---

## 🛠️ Installation

### ✅ Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install required libraries
sudo apt update
sudo apt install build-essential libssl-dev pkg-config
```

### 📦 Project Setup

```bash
# 1. Clone the project
git clone <project-url>
cd server-monitor

# 2. Build C library
gcc -c -fPIC cdosya.c -o cdosya.o
gcc -shared -o libcdosya.so cdosya.o

# 3. Install Rust dependencies
cargo build --release

# 4. Add sound file (optional)
cp /path/to/warn.mp3 ./
```

---

## 🚀 Usage

### 🔧 Quick Start

```bash
export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH
cargo run
```

### 🧪 Production Mode

```bash
./target/release/server_monitor
```

### 🛎️ As Systemd Service

```bash
# Create systemd service file
sudo cp systemd/server-monitor.service /etc/systemd/system/

# Enable and start service
sudo systemctl daemon-reload
sudo systemctl enable server-monitor
sudo systemctl start server-monitor

# View logs
journalctl -u server-monitor -f
```

---

## 📊 Monitoring Parameters

| Parameter       | Warning Threshold | Critical Threshold | Action         |
|-----------------|-------------------|--------------------|----------------|
| CPU Usage       | 90%               | 95%                | Audio Alert    |
| CPU Temperature | 85°C              | 100°C              | Audio Alert    |
| RAM Usage       | 90%               | 98%                | Audio Alert    |
| Disk Usage      | 90%               | 95%                | Visual Warning |
| SSH Connection  | -                 | Any drop           | Immediate Alert|

---

## 🏗️ Project Structure

```
server-monitor/
├── src/
│   ├── main.rs          # Main application
│   ├── monitor.rs       # Monitoring functions
│   └── alerts.rs        # Alert system
├── cdosya.c             # C system functions
├── Cargo.toml           # Rust dependencies
├── warn.mp3             # Alert sound (add manually)
└── README.md
```

---

## 🔧 Configuration

### 🌍 Environment Variables

```bash
export SERVER_IP="192.168.1.195"
export SSH_PORT="22"
export ALERT_COOLDOWN="300"  # 5 minutes
export CHECK_INTERVAL="2"    # 2 seconds
```

### ✏️ Custom Thresholds

Edit the `check_anomalies()` function in `src/main.rs`:

```rust
// Custom thresholds
const CPU_TEMP_CRITICAL: f32 = 100.0;
const RAM_USAGE_CRITICAL: f32 = 98.0;
const CPU_USAGE_CRITICAL: f32 = 95.0;
```

---

## 🐛 Troubleshooting

### 1. Library Not Found

```bash
export LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH
```

### 2. Missing Dependencies

```bash
sudo apt install build-essential libssl-dev pkg-config
```

### 3. Audio Not Working

```bash
sudo apt install libasound2-dev
```

### 4. Permission Denied

```bash
chmod +x target/release/server_monitor
```

---

## 🤝 Contributing

1. Fork the project  
2. Create your feature branch  
   ```bash
   git checkout -b feature/AmazingFeature
   ```
3. Commit your changes  
   ```bash
   git commit -m 'Add AmazingFeature'
   ```
4. Push to the branch  
   ```bash
   git push origin feature/AmazingFeature
   ```
5. Open a Pull Request

---


## 🆘 Support

- 📧 Email: [mr.timonto@gmail.com](mr.timonto@proton.me )  
- 🐛 GitHub Issues  
- 💬 Discord: *Join our server*

---

## 📊 Performance

- **Memory Usage**: ~15MB RAM  
- **CPU Usage**: <1% average  
- **Startup Time**: ~2 seconds  
- **Update Interval**: 2 seconds  

---

⭐ **Star this repo if you find it useful!**

> This project is maintained by **Alperen ERKAN** and contributors.
