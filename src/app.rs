use eframe::egui;
use crate::artist::{ArtistContainer, Tag};

pub struct App {
  artists: Vec<ArtistContainer>,
}

impl App {
  pub fn new(cc: &eframe::CreationContext) -> Self {
    egui_extras::install_image_loaders(&cc.egui_ctx);

    Self {
      artists: vec![
        ArtistContainer::new(
          "anon 2-okuren",
          vec![
            Tag::new("Tag 1"),
            Tag::new_with_image("Tag 2", "/home/ceres/Documents/tag/test.png"),
            Tag::new_with_image("Tag 3", "/home/ceres/Documents/tag/test.webp"),
            Tag::new("Tag 4"),
          ]
        ),
      ],
    }
  }
}

impl eframe::App for App {
  fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
    egui::CentralPanel::default()
      .show(ctx, |ui| {
      
      ui.vertical_centered_justified(|ui| {
        for artist in &mut self.artists {
          artist.show(ui);
        }
      });
    });
  }
}