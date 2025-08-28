#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <sys/statvfs.h>
#include <libc.h>

// SSH bağlantı kontrolü
int ssh_baglanti_kontrol(const char* ip, int port) {
    int sockfd;
    struct sockaddr_in server_addr;
    
    // Socket oluştur
    sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) {
        return -1;
    }
    
    // Server adresini ayarla
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(port);
    server_addr.sin_addr.s_addr = inet_addr(ip);
    
    // Bağlanmayı dene
    int result = connect(sockfd, (struct sockaddr*)&server_addr, sizeof(server_addr));
    close(sockfd);
    
    return (result == 0) ? 0 : -1;
}

// Disk durumu kontrolü
void disk_durumu_kontrol() {
    struct statvfs stat;
    
    if (statvfs("/", &stat) == 0) {
        unsigned long total = stat.f_blocks * stat.f_frsize;
        unsigned long available = stat.f_bavail * stat.f_frsize;
        double usage_percent = 100.0 - ((double)available / total * 100.0);
        
        printf("💾 Disk Kullanımı: %.1f%% (%.1f GB/%.1f GB)\n",
               usage_percent,
               (total - available) / (1024.0 * 1024.0 * 1024.0),
               total / (1024.0 * 1024.0 * 1024.0));
        
        // Kritik disk doluluk uyarısı
        if (usage_percent > 95.0) {
            printf("🚨 KRİTİK: Disk doluluk oranı %.1f%%!\n", usage_percent);
        }
    } else {
        printf("❌ Disk bilgisi alınamadı\n");
    }
}

// Kernel panic kontrolü (basit versiyon)
int kernel_panic_kontrol() {
    FILE* dmesg = popen("dmesg -T | tail -50 | grep -i 'panic\\|Oops\\|kernel'", "r");
    if (dmesg == NULL) {
        return 0;
    }
    
    char buffer[256];
    while (fgets(buffer, sizeof(buffer), dmesg) != NULL) {
        if (strstr(buffer, "panic") || strstr(buffer, "Oops")) {
            pclose(dmesg);
            return 1;
        }
    }
    
    pclose(dmesg);
    return 0;
}