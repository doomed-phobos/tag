use eframe::egui;
use egui_flex::{Flex, item};

pub struct Tag {
  name: String,
  image_filename: Option<String>
}

impl Tag {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      image_filename: None
    }
  }

  pub fn new_with_image(name: impl Into<String>, image_filename: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      image_filename: Some(image_filename.into())
    }
  }
}

pub struct ArtistContainer {
  name: String,
  tags: Vec<Tag>,
  selected: i32,
  show_modal: bool,
  thumbnail_filename: Option<String>
}

impl ArtistContainer {
  const PADDING: egui::Vec2 = egui::vec2(6.0, 4.0);
  const CORNER_RADIOUS: f32 = 16.0;

  pub fn new(name: impl Into<String>, tags: Vec<Tag>) -> Self {
    Self {
      name: name.into(),
      tags,
      selected: -1,
      show_modal: false,
      thumbnail_filename: None
    }
  }

  pub fn show(&mut self, ui: &mut egui::Ui) {
    egui::Frame::group(ui.style())
      .show(ui, |ui| {
        if self.show_modal {
          let modal = egui::Modal::new(egui::Id::new("image-modal")).show(ui.ctx(), |ui| {
            ui.label("HOla");
          });

          if modal.should_close() {
            self.show_modal = false;
          }
        }

        ui.heading(&self.name);
        ui.separator();

        ui.horizontal(|ui| {
          self.draw_image(ui);

          self.draw_data(ui);
        });
      });
  }

  fn draw_image(&mut self, ui: &mut egui::Ui) {
    if let Some(filename) = &self.thumbnail_filename {
      ui.add(
          egui::Image::new(filename)
            .texture_options(egui::TextureOptions::LINEAR.with_mipmap_mode(Some(egui::TextureFilter::Linear)))
            .fit_to_exact_size(egui::Vec2::splat(350.0))
        );

      ui.separator();
    }
  }

  fn draw_data(&mut self, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
      ui.horizontal(|ui| {
        ui.strong("Tags: ");
        ui.scope(|ui| {
          ui.spacing_mut().button_padding = Self::PADDING;

          let ctx = ui.ctx().clone();
          Flex::horizontal()
            .width(ui.available_width())
            .wrap(true)
            .show(ui, |flex| {
              for (i, tag) in self.tags.iter().enumerate() {
                let i = i as i32;
                let selected = i == self.selected;
                if let Some(new_filename) = &tag.image_filename {
                  if flex.add(item(), egui::Button::new(&tag.name)
                    .selected(selected)
                    .corner_radius(Self::CORNER_RADIOUS)).clicked() && !selected {
                    
                    if let Some(prev_filename) = &self.thumbnail_filename {
                      ctx.forget_image(prev_filename.as_str());
                    }
                    
                    self.selected = i;
                    self.thumbnail_filename = Some(format!("file://{new_filename}"));
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
  }
}