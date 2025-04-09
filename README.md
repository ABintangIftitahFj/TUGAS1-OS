# Simulasi Manajemen Proses - FCFS (First-Come, First-Served)

Aplikasi ini merupakan simulasi algoritma penjadwalan proses **First-Come, First-Served (FCFS)** yang dibuat menggunakan bahasa **Rust** dengan GUI berbasis **eframe/egui**. Aplikasi ini menampilkan proses yang telah dimasukkan oleh pengguna serta visualisasi dalam bentuk **Gantt Chart**.

## Fitur

- Input proses secara dinamis (nama proses, waktu kedatangan, dan waktu eksekusi)
- Penambahan dan penghapusan data proses
- Visualisasi Gantt Chart dari hasil penjadwalan FCFS
- Tombol reset untuk menghapus seluruh proses
- Antarmuka grafis yang sederhana dan intuitif

## Screenshot
![Screenshot 2025-04-09 115145](https://github.com/user-attachments/assets/53fec36c-632d-46d4-8078-2a2d54938e05)



## Cara Menjalankan

### Prasyarat

- Telah menginstall [Rust](https://www.rust-lang.org/tools/install)

### Build & Run

```bash
# Masuk ke folder proyek
cd path/to/pake-gui

# Jalankan aplikasi
cargo run
