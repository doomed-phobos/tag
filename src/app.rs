use eframe::egui;
use egui_flex::{Flex, item};

pub struct App {
  artists: Vec<ArtistContainer>,
}

impl App {
  pub fn new(cc: &eframe::CreationContext) -> Self {
    egui_extras::install_image_loaders(&cc.egui_ctx);

    Self {
      artists: vec![
        ArtistContainer {
          name: "anon 2-okuren".to_owned(),
          tags: vec![
            Tag{name: "Tag 1".to_owned(), image_filename: None},
            Tag{name: "Tag 2".to_owned(), image_filename: Some("/home/ceres/Documents/tag/test.png".to_owned())},
            Tag{name: "Tag 3".to_owned(), image_filename: None},
            Tag{name: "Tag 4".to_owned(), image_filename: None},
          ],
          ..Default::default()
        },
      ],
    }
  }
}

struct Tag {
  name: String,
  image_filename: Option<String>
}

struct ArtistContainer {
  name: String,
  tags: Vec<Tag>,
  selected: i32,
}

impl Default for ArtistContainer {
  fn default() -> Self {
    Self {
      tags: Vec::new(),
      selected: -1,
      name: String::new(),
    }
  }
}

impl ArtistContainer {
  const PADDING: egui::Vec2 = egui::vec2(6.0, 4.0);
  const CORNER_RADIOUS: f32 = 16.0;

  fn show(&mut self, ui: &mut egui::Ui) {
    egui::Frame::group(ui.style())
      .show(ui, |ui| {
        ui.heading(&self.name);
        ui.separator();
        ui.horizontal(|ui| {
          if let Some(tag) = self.tags.get(self.selected as usize) {
            if let Some(filename) = &tag.image_filename {
              ui.add(
                egui::Image::new(format!("file://{filename}"))
                  .texture_options(egui::TextureOptions::LINEAR.with_mipmap_mode(Some(egui::TextureFilter::Linear)))
                  .fit_to_exact_size(egui::Vec2::splat(350.0))
              );
            }
          }
          ui.separator();

          ui.vertical(|ui| {
            ui.horizontal(|ui| {
              ui.strong("Tags: ");

              ui.scope(|ui| {
                ui.spacing_mut().button_padding = Self::PADDING;
                Flex::horizontal()
                  .width(ui.available_width())
                  .wrap(true)
                  .show(ui, |flex| {
                    for (i, tag) in self.tags.iter().enumerate() {
                      let selected = i == self.selected as usize;
                      if tag.image_filename.is_some() {
                        if flex.add(item(), egui::Button::new(&tag.name)
                          .selected(selected)
                          .corner_radius(Self::CORNER_RADIOUS)).clicked() {
                          self.selected = i as i32;
                        }
                      } else {
                        // Simulated Non-Events Button
                        flex.add_ui(item(), |ui| {
                          let visuals = ui.visuals();
                          let (fill, stroke) = if selected {
                            let selection = visuals.selection;
                            (selection.bg_fill, selection.stroke)
                          } else {
                            let visuals = visuals.widgets.inactive;
                            (visuals.bg_fill, visuals.bg_stroke)
                          };
                          let color = visuals.widgets.inactive.text_color();

                          egui::Frame::new()
                            .fill(fill)
                            .stroke(stroke)
                            .corner_radius(Self::CORNER_RADIOUS)
                            .inner_margin(Self::PADDING)
                            .show(ui, |ui| {
                              ui.colored_label(color, &tag.name);
                            });
                        });
                      }
                    }
                  });
              });
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
              ui.strong("Links: ");
              
              Flex::horizontal()
                .width(ui.available_width())
                .wrap(true)
                .show(ui, |flex| {
                  for i in 0..50 {
                    flex.add_ui(
                      item(),
                      |ui| {
                        ui.hyperlink(format!("Link {i}"));
                      }
                    );
                  }

                });
            });
          });
        });
      });
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