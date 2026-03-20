use std::collections::HashSet;

use eframe::egui;

use crate::database::Tag;

pub struct AddArtistWindow {
  pub open: bool,
  artist_name: String,
  filter_tag: String,
  selected: HashSet<usize>,
  pub tags: Vec<Tag>,
}

impl AddArtistWindow {
  pub fn new() -> Self {
    Self {
      open: false,
      artist_name: String::new(),
      filter_tag: String::new(),
      selected: HashSet::new(),
      tags: Vec::new(),
    }
  }

  pub fn show(&mut self, ctx: &egui::Context) {
    egui::Window::new("Add Artist")
      .open(&mut self.open)
      .resizable(false)
      .show(ctx, |ui| {
      egui::Grid::new("add_artist_grid")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
          ui.label("Artist Name");
          ui.add(egui::TextEdit::singleline(&mut self.artist_name).hint_text("artist name..."));
          ui.end_row();
          
          ui.label("Tags");
          ui.vertical(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.filter_tag).hint_text("Filter..."));
            egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
              self.selected.iter().for_each(|&x| {
                let item = &self.tags[x];
                ui.label(item.name.as_str());
              });

              ui.vertical_centered_justified(|ui| {
                let tags: Vec<(usize, &str)> = self.tags
                  .iter()
                  .enumerate()
                  .filter(|&x| !self.selected.contains(&x.0))
                  .filter(|&x| self.filter_tag.is_empty() || x.1.name.as_str().contains(self.filter_tag.as_str()))
                  .map(|x| (x.0, x.1.name.as_str()))
                  .collect();

                tags.iter().for_each(|x| {
                  if ui.button(x.1).clicked() {
                    self.selected.insert(x.0);
                  }
                });
              });
            });
          });
        });
    });
  }
}