mod app;
mod database;

fn main() {
  let native_options = eframe::NativeOptions {
    centered: true,
    viewport: eframe::egui::ViewportBuilder::default().with_inner_size([800.0, 400.0]),
    ..Default::default()
  };
  
  eframe::run_native(
    "Tagger",
    native_options,
    Box::new(|cc| Ok(Box::new(app::App::try_new(cc).unwrap())))
  ).expect("Failed to run app");
}