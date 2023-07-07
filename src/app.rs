use std::path::Path;
use anyhow::Result;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MicronApp {
    // Example stuff:
    label: String,

  
}

impl Default for MicronApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
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
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, value } = self;


        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                    if ui.button("Open").clicked() {
                        rfd::
                        ui.ctx().clos
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
        
           
        });

        egui::CentralPanel::default().show(ctx, |ui| {

          
        });

      
    }
}

fn read_file(path: &Path) -> Result<Vec<u8>>{
    use positioned_io::{RandomAccessFile, ReadAt};

    // open a file (note: binding does not need to be mut)
    let raf = RandomAccessFile::open(path)?;

    // read up to 512 bytes
    let mut buf = [0; 512];
    let bytes_read = raf.read_at(2048, &mut buf)?;
    Ok(buf.to_vec())
}

#[derive(serde::Deserialize, serde::Serialize)]
struct FileProperties {
    cursor: u64,
    buffer: Vec<u8>,
}
