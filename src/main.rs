#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod operations;

use eframe::egui;
use std::path::PathBuf;

#[derive(PartialEq, Clone, Copy)]
enum Page { Overview, Files, Settings }

struct WziperApp {
    page: Page,
    files: Vec<PathBuf>,
    output_dir: PathBuf,
    format: usize,
    level: usize,
    status: String,
    busy: bool,
}

impl Default for WziperApp {
    fn default() -> Self {
        Self {
            page: Page::Overview,
            files: Vec::new(),
            output_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            format: 0,
            level: 2,
            status: "Ready. Add files or folders to begin.".into(),
            busy: false,
        }
    }
}

impl WziperApp {
    fn add_files(&mut self) {
        if let Some(paths) = rfd::FileDialog::new().pick_files() {
            for p in paths { if !self.files.contains(&p) { self.files.push(p); } }
            self.status = format!("{} item(s) in the queue.", self.files.len());
        }
    }
    fn add_folder(&mut self) {
        if let Some(p) = rfd::FileDialog::new().pick_folder() {
            if !self.files.contains(&p) { self.files.push(p); }
            self.status = format!("{} item(s) in the queue.", self.files.len());
        }
    }
    fn choose_output(&mut self) {
        if let Some(p) = rfd::FileDialog::new().pick_folder() { self.output_dir = p; }
    }
    fn compress(&mut self) {
        if self.files.is_empty() { self.status = "Add at least one file or folder first.".into(); return; }
        self.busy = true;
        let result = operations::compress(&self.files, &self.output_dir, self.format, self.level);
        self.status = match result {
            Ok(p) => format!("Archive created: {}", p.display()),
            Err(e) => format!("Compression failed: {e:#}"),
        };
        self.busy = false;
    }
    fn extract(&mut self) {
        if let Some(p) = rfd::FileDialog::new().add_filter("Archives", &["zip", "gz", "zst", "tar"]).pick_file() {
            let dest = rfd::FileDialog::new().set_directory(&self.output_dir).pick_folder()
                .unwrap_or_else(|| self.output_dir.clone());
            self.busy = true;
            self.status = match operations::extract(&p, &dest) {
                Ok(()) => format!("Extracted to {}", dest.display()),
                Err(e) => format!("Extraction failed: {e:#}"),
            };
            self.busy = false;
        }
    }
}

impl eframe::App for WziperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.visuals = egui::Visuals::light();
        style.visuals.panel_fill = egui::Color32::from_rgb(246, 247, 250);
        style.visuals.window_fill = egui::Color32::WHITE;
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        ctx.set_style(style);

        egui::SidePanel::left("sidebar").exact_width(205.0).resizable(false).show(ctx, |ui| {
            ui.add_space(18.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("◈").size(27.0).color(egui::Color32::from_rgb(225, 190, 48)));
                ui.label(egui::RichText::new("Wziper").size(23.0).strong().color(egui::Color32::from_rgb(48, 54, 67)));
            });
            ui.add_space(28.0);
            ui.label(egui::RichText::new("WORKSPACE").size(10.0).strong().color(egui::Color32::GRAY));
            ui.add_space(8.0);
            nav_button(ui, "⌂", "Overview", &mut self.page, Page::Overview);
            nav_button(ui, "▤", "File Management", &mut self.page, Page::Files);
            nav_button(ui, "⚙", "Settings", &mut self.page, Page::Settings);
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
                ui.separator();
                ui.label(egui::RichText::new("Wziper 0.1.0 • Preview").size(10.0).color(egui::Color32::GRAY));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(match self.page { Page::Overview => "Management Overview", Page::Files => "File Management", Page::Settings => "Settings" });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("＋ Add files").clicked() { self.add_files(); }
                    if ui.button("＋ Add folder").clicked() { self.add_folder(); }
                });
            });
            ui.add_space(12.0);

            match self.page {
                Page::Overview | Page::Files => {
                    ui.columns(3, |cols| {
                        stat_card(&mut cols[0], "Items queued", &self.files.len().to_string(), "Ready for compression");
                        stat_card(&mut cols[1], "Archive format", ["ZIP", "TAR.GZ", "TAR.ZST"][self.format], "Choose a format below");
                        stat_card(&mut cols[2], "Compression", ["Fast", "Balanced", "Maximum"][self.level], "Adjustable profile");
                    });
                    ui.add_space(10.0);
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Compression queue").strong().size(17.0));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Clear queue").clicked() { self.files.clear(); }
                                if ui.button("＋ Add").clicked() { self.add_files(); }
                            });
                        });
                        ui.separator();
                        if self.files.is_empty() {
                            ui.add_space(25.0);
                            ui.vertical_centered(|ui| {
                                ui.label(egui::RichText::new("＋").size(38.0).color(egui::Color32::from_rgb(203, 184, 86)));
                                ui.label(egui::RichText::new("Drop files here or add files to start").strong());
                                ui.label(egui::RichText::new("Your originals are never modified.").color(egui::Color32::GRAY).size(12.0));
                            });
                            ui.add_space(25.0);
                        } else {
                            egui::ScrollArea::vertical().max_height(210.0).show(ui, |ui| {
                                let mut remove = None;
                                for (i, p) in self.files.iter().enumerate() {
                                    ui.horizontal(|ui| {
                                        ui.label("▧");
                                        ui.label(p.file_name().unwrap_or_default().to_string_lossy().to_string());
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            if ui.small_button("Remove").clicked() { remove = Some(i); }
                                        });
                                    });
                                }
                                if let Some(i) = remove { self.files.remove(i); }
                            });
                        }
                    });
                    ui.add_space(10.0);
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Archive settings").strong().size(17.0));
                        ui.horizontal(|ui| {
                            ui.label("Format");
                            egui::ComboBox::from_id_salt("format").selected_text(["ZIP", "TAR.GZ", "TAR.ZST"][self.format]).show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.format, 0, "ZIP — best compatibility");
                                ui.selectable_value(&mut self.format, 1, "TAR.GZ — broad support");
                                ui.selectable_value(&mut self.format, 2, "TAR.ZST — fast modern compression");
                            });
                        });
                        ui.horizontal(|ui| {
                            ui.label("Profile");
                            egui::ComboBox::from_id_salt("level").selected_text(["Fast", "Balanced", "Maximum"][self.level]).show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.level, 0, "Fast");
                                ui.selectable_value(&mut self.level, 1, "Balanced");
                                ui.selectable_value(&mut self.level, 2, "Maximum");
                            });
                        });
                        ui.horizontal(|ui| {
                            ui.label("Save to");
                            ui.label(self.output_dir.display().to_string());
                            if ui.button("Browse…").clicked() { self.choose_output(); }
                        });
                        ui.add_space(5.0);
                        ui.horizontal(|ui| {
                            if ui.add_sized([160.0, 38.0], egui::Button::new(egui::RichText::new("Create archive").strong())).clicked() { self.compress(); }
                            if ui.add_sized([130.0, 38.0], egui::Button::new("Extract archive")).clicked() { self.extract(); }
                        });
                    });
                }
                Page::Settings => {
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Wziper settings").strong().size(18.0));
                        ui.label("This preview keeps settings local to the current session.");
                        ui.label("Default archive format and compression profile can be selected on the Overview page.");
                        ui.label("Supported creation formats: ZIP, TAR.GZ, TAR.ZST.");
                        ui.label("Supported extraction formats: ZIP, TAR.GZ, TAR.ZST.");
                    });
                }
            }
            ui.add_space(10.0);
            ui.group(|ui| {
                ui.label(egui::RichText::new("Activity").strong());
                ui.label(&self.status);
                if self.busy { ui.spinner(); }
            });
        });
    }
}

fn nav_button(ui: &mut egui::Ui, icon: &str, title: &str, page: &mut Page, target: Page) {
    let selected = *page == target;
    let text = format!("{icon}   {title}");
    if ui.add_sized([185.0, 36.0], egui::Button::new(text).selected(selected)).clicked() { *page = target; }
}
fn stat_card(ui: &mut egui::Ui, title: &str, value: &str, hint: &str) {
    ui.group(|ui| {
        ui.label(egui::RichText::new(title).size(12.0).color(egui::Color32::GRAY));
        ui.label(egui::RichText::new(value).size(22.0).strong());
        ui.label(egui::RichText::new(hint).size(10.0).color(egui::Color32::GRAY));
    });
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1120.0, 760.0])
            .with_min_inner_size([850.0, 600.0])
            .with_title("Wziper — File Compressor"),
        ..Default::default()
    };
    eframe::run_native("Wziper", options, Box::new(|_cc| Ok(Box::new(WziperApp::default()))))
}
