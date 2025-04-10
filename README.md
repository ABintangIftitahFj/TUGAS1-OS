
# Simulasi Manajemen Proses - FCFS (First-Come, First-Served)

Aplikasi ini merupakan simulasi algoritma penjadwalan proses **First-Come, First-Served (FCFS)** yang dibuat menggunakan bahasa **Rust** dengan GUI berbasis **eframe/egui**. Aplikasi ini menampilkan proses yang telah dimasukkan oleh pengguna serta visualisasi dalam bentuk **Gantt Chart**.

## Penjelasan FCFS

Pada algoritma **FCFS (First-Come, First-Served)**, proses dieksekusi berdasarkan urutan waktu kedatangannya. Ini adalah algoritma penjadwalan proses paling sederhana dalam sistem operasi.

### Kelebihan (Pro) FCFS:
- **Sederhana dan mudah diimplementasikan** - Cukup menggunakan struktur data antrian sederhana.
- **Bebas kelaparan (starvation-free)** - Semua proses pasti akan mendapatkan jatah eksekusi CPU.
- **Adil berdasarkan urutan kedatangan** - Proses dilayani seperti antrian di dunia nyata: yang datang lebih dulu, dilayani lebih dulu.
- **Overhead rendah** - Tidak memerlukan perhitungan kompleks untuk menentukan proses berikutnya.
- **Mudah diprediksi** - Urutan eksekusi proses dapat diprediksi dengan jelas.

### Kekurangan (Cons) FCFS:
- **Efek konvoi (convoy effect)** - Proses yang membutuhkan waktu eksekusi panjang dapat menghambat proses-proses pendek yang datang setelahnya.
- **Waktu tunggu rata-rata tinggi** - Tidak optimal untuk lingkungan yang membutuhkan responsivitas tinggi.
- **Tidak mempertimbangkan prioritas** - Proses penting harus menunggu jika ada proses lain yang datang lebih dulu.
- **Tidak cocok untuk sistem interaktif** - Pengguna mungkin merasakan lag karena waktu respons yang tidak dapat diprediksi.
- **Performa buruk untuk proses dengan berbagai panjang burst time** - Jika proses panjang datang lebih dulu, efisiensi CPU menurun.

### Rumus Perhitungan Performa:

#### 1. Waiting Time (WT)
Waktu tunggu adalah selisih antara waktu mulai eksekusi dengan waktu kedatangan proses.
> Waktu proses menunggu di ready queue sebelum dieksekusi oleh CPU.

WT = Start Time - Arrival Time

#### 2. Turnaround Time (TAT)
Waktu turnaround adalah total waktu yang dibutuhkan sejak proses datang hingga selesai dieksekusi.

TAT = Completion Time - Arrival Time

Completion Time = Start Time + Burst Time

TAT = Waiting Time + Burst Time

#### 3. Rata-rata Waiting Time & Turnaround Time
Aplikasi juga menghitung rata-rata untuk semua proses:

Average WT = Total Waiting Time / Jumlah Proses

Average TAT = Total Turnaround Time / Jumlah Proses

## Implementasi dalam Sistem Operasi

FCFS adalah algoritma penjadwalan dasar yang digunakan dalam beberapa sistem batch processing dan sistem operasi generasi awal. Meskipun sederhana, FCFS jarang digunakan sebagai algoritma penjadwalan utama dalam sistem operasi modern karena kelemahan-kelemahannya. Namun, prinsip FCFS masih digunakan dalam komponen-komponen tertentu seperti:

- Antrian proses dalam sistem batch
- Penjadwalan I/O pada beberapa sistem
- Implementasi dasar dalam sistem pendidikan untuk memahami konsep penjadwalan

## Contoh Kasus Penggunaan FCFS

Misalkan ada 4 proses dengan waktu kedatangan (AT) dan burst time (BT) berikut:

| Proses | Arrival Time | Burst Time |
|--------|--------------|------------|
| P1     | 0            | 5          |
| P2     | 1            | 3          |
| P3     | 2            | 8          |
| P4     | 3            | 2          |

Dalam FCFS, urutan eksekusi akan: P1 → P2 → P3 → P4

Hasil perhitungan:
- P1: WT = 0, TAT = 5
- P2: WT = 4, TAT = 7
- P3: WT = 7, TAT = 15
- P4: WT = 15, TAT = 17

Average WT = 6.5
Average TAT = 11

## Fitur Aplikasi

- Input proses secara dinamis (nama proses, waktu kedatangan, dan waktu eksekusi)
- Penambahan dan penghapusan data proses
- Visualisasi Gantt Chart dari hasil penjadwalan FCFS
- Tombol reset untuk menghapus seluruh proses
- Antarmuka grafis yang sederhana dan intuitif
- Perhitungan otomatis metrik performa (waiting time, turnaround time)

## Screenshot
![fcfc](https://github.com/user-attachments/assets/f310937e-fea1-438b-bcc2-a83a3c0112cc)

![Recording 2025-04-09 141651](https://github.com/user-attachments/assets/40a352c7-3a1a-4638-aaef-4b5d7c6d3673)


## Cara Menjalankan

### Prasyarat
- Telah menginstall [Rust](https://www.rust-lang.org/tools/install)

### Build & Run
```bash
# Masuk ke folder proyek
cd path/to/pake-gui

# Jalankan aplikasi
cargo run
```
