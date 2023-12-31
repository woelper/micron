use anyhow::Result;
use egui::{Color32, TextEdit, Vec2, ScrollArea, TextStyle};
use log::info;
use positioned_io::{RandomAccessFile, ReadAt};
use std::io::Read;
use std::{
    collections::{BTreeSet, HashMap},
    fs::{metadata, File},
    path::{Path, PathBuf},
};

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct Settings {
    line_numbers: bool,
    tree_view: bool,
    recent_files: BTreeSet<PathBuf>,
    editor_font_size: f32,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MicronApp {
    #[serde(skip)]
    open_files: HashMap<PathBuf, OpenedFile>,
    active_file: Option<PathBuf>,
    settings: Settings,
}

impl Default for MicronApp {
    fn default() -> Self {
        Self {
            open_files: Default::default(),
            active_file: Default::default(),
            settings: Default::default(),
        }
    }
}

impl MicronApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters):
        fonts.font_data.insert(
            "mono".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/FiraCode-Regular.ttf")),
        ); // .ttf and .otf supported

        fonts.font_data.insert(
            "sans".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/FiraCode-Regular.ttf")),
        ); // .ttf and .otf supported

        // Put my font first (highest priority):
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "sans".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "mono".to_owned());

        cc.egui_ctx.set_fonts(fonts);

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
                                self.settings.recent_files.insert(p.clone());
                                self.open_files.insert(p, of);
                            }
                        }
                        ui.close_menu();
                    }

                    ui.menu_button("Recent", |ui| {
                        for p in &self.settings.recent_files {
                            if let Some(fname) = p.file_name() {
                                if ui.button(fname.to_string_lossy().to_string()).clicked() {
                                    if let Ok(of) = read_file(&p) {
                                        self.active_file = Some(p.clone());
                                        self.open_files.insert(p.clone(), of);
                                    }

                                    ui.close_menu();
                                }
                            }
                        }
                    });
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            let mut open_files = self.open_files.keys().collect::<Vec<_>>();
            open_files.sort();

            ui.vertical_centered_justified(|ui| {
                for f in open_files {
                    if ui
                        .button(format!(
                            "{}",
                            f.file_name()
                                .map(|f| f.to_string_lossy().to_string())
                                .unwrap_or_default()
                        ))
                        .on_hover_text(f.display().to_string())
                        .clicked()
                    {
                        self.active_file = Some(f.clone());
                    }
                }
            });
            if ui.button("Close all").clicked() {
                self.open_files.clear();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(opened_file) = self
                .open_files
                .get_mut(&(self.active_file.clone()).unwrap_or_default())
            {
                let mut text = String::from_utf8_lossy(opened_file.buffer.as_ref()).to_string();

                egui::ScrollArea::vertical()
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        let mut theme =
                            crate::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
                        ui.collapsing("Theme", |ui| {
                            ui.group(|ui| {
                                theme.ui(ui);
                                theme.clone().store_in_memory(ui.ctx());
                            });
                        });

                        let ext = opened_file
                            .path
                            .extension()
                            .map(|e| e.to_string_lossy().to_string().to_lowercase())
                            .unwrap_or_default();

                        let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                            let mut layout_job = crate::syntax_highlighting::highlight(
                                ui.ctx(),
                                &theme,
                                string,
                                ext.as_str(),
                            );
                            layout_job.wrap.max_width = wrap_width;
                            ui.fonts(|f| f.layout_job(layout_job))
                        };

                        if opened_file.partial {
                            ui.label("Large file mode");

                            let response = ui.add(
                                egui::Slider::new(&mut opened_file.cursor, 0..=opened_file.len)
                                    .logarithmic(true),
                            );
                            if response.changed() {
                                opened_file.seek().unwrap_or_default();
                            }
                        }

                        // let text_style = TextStyle::Monospace;
                        // let row_height = ui.text_style_height(&text_style);
                        // let num_rows = text.lines().count();
                        // ScrollArea::vertical().auto_shrink([false; 2]).show_rows(
                        //     ui,
                        //     row_height,
                        //     num_rows,
                        //     |ui, row_range| {
                        //         for row in row_range {
                        //             let text = format!("This is row {}/{}", row + 1, num_rows);
                        //             ui.label(text);
                        //         }
                        //     },
                        // );



                        egui::ScrollArea::vertical().show(ui, |ui| {
                            if ui
                                .add(
                                    egui::TextEdit::multiline(&mut text)
                                        .font(egui::TextStyle::Monospace) // for cursor height
                                        .code_editor()
                                        .desired_rows(10)
                                        .lock_focus(true)
                                        .desired_width(f32::INFINITY)
                                        .frame(false)
                                        .margin(Vec2::new(2., 2.))
                                        .layouter(&mut layouter),
                                )
                                .changed()
                            {
                                opened_file.buffer = text.into_bytes();
                            }
                        });
                    });
            }
        });
    }
}

fn read_file(path: &Path) -> Result<OpenedFile> {
    // open a file (note: binding does not need to be mut)
    let raf = RandomAccessFile::open(path)?;
    let meta = metadata(path)?;

    const MAX_BYTES: u64 = 5 * 1000000;

    if meta.len() < MAX_BYTES.try_into()? {
        // read up to 512 bytes
        let mut buf = vec![];
        File::read_to_end(&mut File::open(path)?, &mut buf)?;
        Ok(OpenedFile {
            cursor: 0,
            buffer: buf,
            partial: false,
            path: path.into(),
            len: meta.len(),
        })
    } else {
        info!("Large file");
        let mut buf = [0; 10000];
        raf.read_at(0, &mut buf)?;
        Ok(OpenedFile {
            cursor: 0,
            buffer: buf.to_vec(),
            partial: true,
            path: path.into(),
            len: meta.len(),
        })
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct OpenedFile {
    cursor: u64,
    buffer: Vec<u8>,
    partial: bool,
    path: PathBuf,
    len: u64,
}

impl OpenedFile {
    pub fn seek(&mut self) -> Result<()> {
        let mut buf = [0; 10000];

        // let meta = metadata(path)?;
        let raf = RandomAccessFile::open(&self.path)?;
        raf.read_at(self.cursor, &mut buf)?;

        self.buffer = buf.to_vec();

        Ok(())
    }
}
