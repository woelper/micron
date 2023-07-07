use anyhow::Result;
use egui::TextEdit;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MicronApp {
    open_files: HashMap<PathBuf, OpenedFile>,
    active_file: Option<PathBuf>,
}

impl Default for MicronApp {
    fn default() -> Self {
        Self {
            open_files: Default::default(),
            active_file: Default::default(),
        }
    }
}

impl MicronApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for MicronApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                    if ui.button("Open").clicked() {
                        if let Some(p) = rfd::FileDialog::new().pick_file() {
                            if let Ok(of) = read_file(&p) {
                                self.active_file = Some(p.clone());
                                self.open_files.insert(p, of);
                            }
                        }
                        ui.close_menu();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            let mut open_files = self.open_files.keys().collect::<Vec<_>>();
            open_files.sort();
            for f in open_files {
                if ui.button(format!("{}", f.display())).clicked() {
                    self.active_file = Some(f.clone());
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(opened_file) = self
                .open_files
                .get(&(self.active_file.clone()).unwrap_or_default())
            {
                let mut text = String::from_utf8_lossy(opened_file.buffer.as_ref()).to_string();
                ui.add(
                    TextEdit::multiline(&mut text)
                        .desired_width(f32::INFINITY)
                        .code_editor(),
                );
            }
        });
    }
}

fn read_file(path: &Path) -> Result<OpenedFile> {
    use positioned_io::{RandomAccessFile, ReadAt};

    // open a file (note: binding does not need to be mut)
    let raf = RandomAccessFile::open(path)?;

    // read up to 512 bytes
    let mut buf = [0; 512];
    raf.read_at(0, &mut buf)?;
    Ok(OpenedFile {
        cursor: 0,
        buffer: buf.to_vec(),
    })
}

#[derive(serde::Deserialize, serde::Serialize)]
struct OpenedFile {
    cursor: u64,
    buffer: Vec<u8>,
}
