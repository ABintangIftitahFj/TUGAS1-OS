# Simulasi Manajemen Proses - FCFS (First-Come, First-Served)

Aplikasi ini merupakan simulasi algoritma penjadwalan proses **First-Come, First-Served (FCFS)** yang dibuat menggunakan bahasa **Rust** dengan GUI berbasis **eframe/egui**. Aplikasi ini menampilkan proses yang telah dimasukkan oleh pengguna serta visualisasi dalam bentuk **Gantt Chart**.

## Penjelasan Rumus FCFS

Pada algoritma **FCFS (First-Come, First-Served)**, proses dieksekusi berdasarkan urutan waktu kedatangannya. Aplikasi ini menggunakan rumus-rumus standar berikut untuk menghitung performa dari tiap proses:

### 1. Waiting Time (WT)
Waktu tunggu adalah selisih antara waktu mulai eksekusi dengan waktu kedatangan proses.

> Waktu proses menunggu di ready queue sebelum dieksekusi oleh CPU.


WT = Start Time - Arrival Time

### 2. Turnaround Time (TAT)
Waktu turnaround adalah total waktu yang dibutuhkan sejak proses datang hingga selesai dieksekusi.

TAT = Completion Time - Arrival Time

Completion Time = Start Time + Burst Time

TAT = Waiting Time + Burst Time


### 3. Rata-rata Waiting Time & Turnaround Time
Aplikasi juga menghitung rata-rata untuk semua proses:
Average WT = Total Waiting Time / Jumlah Proses
Average TAT = Total Turnaround Time / Jumlah Proses



## Fitur

- Input proses secara dinamis (nama proses, waktu kedatangan, dan waktu eksekusi)
- Penambahan dan penghapusan data proses
- Visualisasi Gantt Chart dari hasil penjadwalan FCFS
- Tombol reset untuk menghapus seluruh proses
- Antarmuka grafis yang sederhana dan intuitif

## Screenshot
![Screenshot 2025-04-09 115145](https://github.com/user-attachments/assets/53fec36c-632d-46d4-8078-2a2d54938e05)

![Recording 2025-04-09 141651](https://github.com/user-attachments/assets/450b0785-5b6c-46fa-80bb-3808227099dc)





## Cara Menjalankan

### Prasyarat

- Telah menginstall [Rust](https://www.rust-lang.org/tools/install)

### Build & Run

```bash
# Masuk ke folder proyek
cd path/to/pake-gui

# Jalankan aplikasi
cargo run
