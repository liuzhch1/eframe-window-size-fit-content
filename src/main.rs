fn main() {
  let app = AppGUI::default();
  eframe::run_native(
    "eframe window size fit content",
    eframe::NativeOptions::default(),
    Box::new(|_cc| Ok(Box::new(app))),
  )
  .unwrap();
}

struct AppGUI {
  name: String,
  age: u8,
  content: String,
  _first_render: bool,
}

impl Default for AppGUI {
  fn default() -> Self {
    Self {
      name: "John".to_string(),
      age: 20,
      content: "This demo shows how the eframe window size fit content works.".to_string(),
      _first_render: true,
    }
  }
}
impl AppGUI {
  fn pre_render(&mut self, ctx: &eframe::egui::Context) {
    egui::Window::new("pre_render")
      .title_bar(false)
      .fixed_pos((0.0, 0.0))
      .show(ctx, |ui| {
        self.render(ui);
      });
  }
  fn render(&mut self, ui: &mut egui::Ui) {
    ui.add(
      egui::Label::new(egui::RichText::new("Hello, world!").heading())
        .wrap_mode(egui::TextWrapMode::Extend),
    );
    ui.label(format!("Name: {}", self.name));
    ui.label(format!("Age: {}", self.age));
    ui.label(format!("{} ", self.content));
  }
}

impl eframe::App for AppGUI {
  fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
    if self._first_render {
      self.pre_render(ctx);
      self._first_render = false;
      let window_size = ctx.used_size();
      ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(window_size));
    } else {
      egui::CentralPanel::default().show(ctx, |ui| {
        self.render(ui);
      });
    }
  }
}
