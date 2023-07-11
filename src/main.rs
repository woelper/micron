#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions::default();
    
    // let mut tree = create_tree();
    
    eframe::run_native(
        "micron",
        native_options,
        Box::new(|cc| Box::new(micron::MicronApp::new(cc))),
    )

    
}


// impl egui_tiles::Behavior<Pane> for TreeBehavior {
    //     fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
    //         format!("Pane {}", pane.nr).into()
    //     }
    
    //     fn pane_ui(
    //         &mut self,
    //         ui: &mut egui::Ui,
    //         _tile_id: egui_tiles::TileId,
    //         pane: &mut Pane,
    //     ) -> egui_tiles::UiResponse {
    //         // Give each pane a unique color:
    //         let color = egui::epaint::Hsva::new(0.103 * pane.nr as f32, 0.5, 0.5, 1.0);
    //         ui.painter().rect_filled(ui.max_rect(), 0.0, color);
    
    //         ui.label(format!("The contents of pane {}.", pane.nr));
    
    //         // You can make your pane draggable like so:
    //         if ui
    //             .add(egui::Button::new("Drag me!").sense(egui::Sense::drag()))
    //             .drag_started()
    //         {
    //             egui_tiles::UiResponse::DragStarted
    //         } else {
    //             egui_tiles::UiResponse::None
    //         }
    //     }
    // }

    // eframe::run_simple_native("My egui App", native_options, move |ctx, _frame| {
    //     egui::CentralPanel::default().show(ctx, |ui| {
    //         let mut behavior = TreeBehavior {};
    //         tree.ui(&mut behavior, ui);
    //     });
    // })


struct Pane {
    nr: usize,
}

struct TreeBehavior {}

// fn create_tree() -> egui_tiles::Tree<Pane> {
//     let mut next_view_nr = 0;
//     let mut gen_pane = || {
//         let pane = Pane { nr: next_view_nr };
//         next_view_nr += 1;
//         pane
//     };

//     let mut tiles = egui_tiles::Tiles::default();

//     let mut tabs = vec![];
//     tabs.push({
//         let children = (0..7).map(|_| tiles.insert_pane(gen_pane())).collect();
//         tiles.insert_horizontal_tile(children)
//     });
//     tabs.push({
//         let cells = (0..11).map(|_| tiles.insert_pane(gen_pane())).collect();
//         tiles.insert_grid_tile(cells)
//     });
//     tabs.push(tiles.insert_pane(gen_pane()));

//     let root = tiles.insert_tab_tile(tabs);

//     egui_tiles::Tree::new(root, tiles)
// }