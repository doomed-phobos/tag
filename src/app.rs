use std::{fs, io};

use crate::database::Database;

pub struct App {
  database: Database,
}

impl App {
  pub fn try_new(cc: &eframe::CreationContext) -> Result<Self, io::Error> {
    let project_dir = dirs::data_dir()
      .ok_or(io::Error::new(io::ErrorKind::NotADirectory, "Data dir isn't found"))?
      .join("tagger");
    fs::create_dir_all(&project_dir)?;
    let database = Database::try_load(project_dir)?;

    egui_extras::install_image_loaders(&cc.egui_ctx);

    Ok(Self {
      database
    })  
  }
}

impl eframe::App for App {
  fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
    
  }

  fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
    self.database.save().unwrap();
  }
}

/*
    egui::CentralPanel::default()
      .show(ctx, |ui| {
      
      ui.vertical_centered_justified(|ui| {
        for artist in &mut self.artists {
          artist.show(ui);
        }
      });
    });

*/