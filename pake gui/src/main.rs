use eframe::egui;
use egui::{Color32, TextEdit, Vec2};

#[derive(Default, Debug, Clone)]
struct Process {
    name: String,
    arrival_time: u32,
    burst_time: u32,
    waiting_time: u32,
    turnaround_time: u32,
}

#[derive(Default)]
struct MyApp {
    processes: Vec<Process>,
    input_name: String,
    input_arrival: String,
    input_burst: String,
    results: Vec<Process>,
    show_results: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        let frame = egui::CentralPanel::default();
        
        frame.show(ctx, |ui| {
            // Tambahkan spacing dan style dasar
            ui.spacing_mut().item_spacing = Vec2::new(8.0, 10.0);
            
            // Header
            ui.heading("Simulasi FCFS - Manajemen Proses");
            ui.label("First Come First Serve Process Scheduling");
            ui.add_space(10.0);

            // Form input proses
            ui.group(|ui| {
                ui.label("Input Data Proses");
                ui.separator();
                
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Nama Proses:");
                        ui.add(TextEdit::singleline(&mut self.input_name)
                            .desired_width(100.0));
                    });
                    
                    ui.vertical(|ui| {
                        ui.label("Arrival Time:");
                        ui.add(TextEdit::singleline(&mut self.input_arrival)
                            .desired_width(80.0));
                    });
                    
                    ui.vertical(|ui| {
                        ui.label("Burst Time:");
                        ui.add(TextEdit::singleline(&mut self.input_burst)
                            .desired_width(80.0));
                    });

                    ui.vertical(|ui| {
                        ui.add_space(18.0); // Align with text fields
                        if ui.button("Tambah Proses").clicked() {
                            if let (Ok(arrival), Ok(burst)) = (
                                self.input_arrival.parse::<u32>(),
                                self.input_burst.parse::<u32>(),
                            ) {
                                let name = if self.input_name.trim().is_empty() {
                                    format!("P{}", self.processes.len() + 1)
                                } else {
                                    self.input_name.clone()
                                };
                                
                                self.processes.push(Process {
                                    name,
                                    arrival_time: arrival,
                                    burst_time: burst,
                                    ..Default::default()
                                });

                                // Reset input fields
                                self.input_name.clear();
                                self.input_arrival.clear();
                                self.input_burst.clear();
                            }
                        }
                    });
                });
            });

            ui.add_space(8.0);
            
            // Daftar proses
            if !self.processes.is_empty() {
                ui.group(|ui| {
                    ui.label("Daftar Proses");
                    ui.separator();
                    
                    egui::Grid::new("proses_grid")
                        .striped(true)
                        .spacing([20.0, 6.0])
                        .show(ui, |ui| {
                            ui.strong("Nama");
                            ui.strong("Arrival Time");
                            ui.strong("Burst Time");
                            ui.strong("");
                            ui.end_row();

                            let mut to_remove = None;
                            for (idx, p) in self.processes.iter().enumerate() {
                                ui.label(&p.name);
                                ui.label(p.arrival_time.to_string());
                                ui.label(p.burst_time.to_string());
                                
                                if ui.small_button("Hapus").clicked() {
                                    to_remove = Some(idx);
                                }
                                
                                ui.end_row();
                            }
                            
                            if let Some(idx) = to_remove {
                                self.processes.remove(idx);
                            }
                        });
                });
                
                ui.add_space(12.0);
                
                // Tombol aksi
                ui.horizontal(|ui| {
                    if ui.button("Simulasikan FCFS").clicked() {
                        self.results = self.processes.clone();
                        self.results.sort_by_key(|p| p.arrival_time);

                        let mut time = 0;
                        for p in &mut self.results {
                            if time < p.arrival_time {
                                time = p.arrival_time;
                            }
                            p.waiting_time = time - p.arrival_time;
                            p.turnaround_time = p.waiting_time + p.burst_time;
                            time += p.burst_time;
                        }
                        
                        self.show_results = true;
                    }

                    if ui.button("Reset Semua").clicked() {
                        self.processes.clear();
                        self.results.clear();
                        self.input_name.clear();
                        self.input_arrival.clear();
                        self.input_burst.clear();
                        self.show_results = false;
                    }
                });
            }

            // Hasil Simulasi
            if self.show_results && !self.results.is_empty() {
                ui.add_space(16.0);
                
                ui.group(|ui| {
                    ui.heading("Hasil Simulasi FCFS");
                    ui.separator();
                    
                    // Menghitung rata-rata waiting time dan turnaround time
                    let avg_waiting: f32 = self.results.iter().map(|p| p.waiting_time as f32).sum::<f32>() / self.results.len() as f32;
                    let avg_turnaround: f32 = self.results.iter().map(|p| p.turnaround_time as f32).sum::<f32>() / self.results.len() as f32;
                    
                    ui.horizontal(|ui| {
                        ui.label("Rata-rata Waiting Time:");
                        ui.strong(format!("{:.2} ms", avg_waiting));
                        ui.add_space(20.0);
                        ui.label("Rata-rata Turnaround Time:");
                        ui.strong(format!("{:.2} ms", avg_turnaround));
                    });
                    
                    ui.add_space(8.0);
                    
                    // Visualisasi Gantt chart sederhana tanpa menggunakan fitur drawing kompleks
                    ui.label("Gantt Chart Sederhana:");
                    ui.separator();
                    
                    let mut current_time = 0;
                    
                    // Menampilkan proses-proses dalam bentuk horizontal bar sederhana
                    for (i, p) in self.results.iter().enumerate() {
                        let start_time = p.arrival_time + p.waiting_time;
                        if start_time > current_time {
                            // Ada gap, tampilkan idle
                            ui.horizontal(|ui| {
                                ui.label(format!("[{}] Idle: {} - {}", i, current_time, start_time));
                                ui.colored_label(Color32::LIGHT_GRAY, format!("({} ms)", start_time - current_time));
                            });
                        }
                        
                        ui.horizontal(|ui| {
                            ui.label(format!("[{}] Proses {}: {} - {}", i, p.name, start_time, start_time + p.burst_time));
                            ui.colored_label(Color32::LIGHT_BLUE, format!("({} ms)", p.burst_time));
                        });
                        
                        current_time = start_time + p.burst_time;
                    }
                    
                    ui.add_space(8.0);
                    ui.separator();
                    
                    // Tabel hasil simulasi
                    egui::Grid::new("hasil_simulasi")
                        .striped(true)
                        .spacing([20.0, 6.0])
                        .show(ui, |ui| {
                            ui.strong("Proses");
                            ui.strong("Arrival Time");
                            ui.strong("Burst Time");
                            ui.strong("Waiting Time");
                            ui.strong("Turnaround Time");
                            ui.end_row();

                            for p in &self.results {
                                ui.label(&p.name);
                                ui.label(p.arrival_time.to_string());
                                ui.label(p.burst_time.to_string());
                                ui.label(p.waiting_time.to_string());
                                ui.label(p.turnaround_time.to_string());
                                ui.end_row();
                            }
                        });
                });
            }
            
            ui.add_space(10.0);
            ui.separator();
            ui.label("© 2025 - Simulasi FCFS");
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Simulasi FCFS - Manajemen Proses",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}