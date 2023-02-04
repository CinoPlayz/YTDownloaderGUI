use egui::Id;
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct YTApp {
    name: String,
    age: u32,

    //Kar se ne rabi shraniti
    #[serde(skip)]
    open: bool,
    #[serde(skip)]
    IDjiZaNapakaWindow: Vec<u16>,
}

impl Default for YTApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            open: false,
            IDjiZaNapakaWindow: Vec::from([26252, 18405, 12010, 43838])
        }
    }
}

impl YTApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()        
    }
}

impl eframe::App for YTApp {



    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            egui::Window::new("Napaka").open(&mut self.open).show(ctx, |ui| {
                ui.label("Hello World!");
            });

            IzpisiNapako(self, ctx, self.IDjiZaNapakaWindow[0], "Napaka 1");
            IzpisiNapako(self, ctx, self.IDjiZaNapakaWindow[1], "Napaka 2");
        });
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
}

fn IzpisiNapako(neki: &mut YTApp, ctx: &egui::Context, id_windowa: u16, napaka: &str){

    //Ustvari nov egui window za napako in mu nastavi id
    egui::Window::new("Napaka").id(Id::new(id_windowa)).open(&mut neki.open).show(ctx, |ui| {
        ui.label(napaka);
    });
}