use std::path::Path;

use egui::Id;
use egui::RichText;
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use egui::global_dark_light_mode_switch;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct YTApp {
    name: String,
    age: u32,
    picked_path: Option<String>,
    

    //Kar se ne rabi shraniti
    #[serde(skip)]
    OdprtiOkniZaNapako: bool,
    #[serde(skip)]
    IDjiZaNapakaWindow: Vec<u16>,
    #[serde(skip)]
    TextureNapaka: Option<egui::TextureHandle>,
    #[serde(skip)]
    PrikaziNapakoUI: bool,
}

impl Default for YTApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            OdprtiOkniZaNapako: true,
            IDjiZaNapakaWindow: Vec::from([26252, 18405, 12010, 43838]),
            TextureNapaka: None,
            PrikaziNapakoUI: true,
            picked_path: None
        }
    }
}

fn setup_custom_text_style(ctx: &egui::Context) {
    // Get current context style
    let mut style = (*ctx.style()).clone();

    // Redefine text_styles
    style.text_styles = [
    (Heading, FontId::new(30.0, Proportional)),
    (Name("Heading2".into()), FontId::new(25.0, Proportional)),
    (Name("Context".into()), FontId::new(10.0, Proportional)),
    (Name("NapakaLabel".into()), FontId::new(10.0, Proportional)),
    (Body, FontId::new(18.0, Proportional)),
    (Monospace, FontId::new(14.0, Proportional)),
    (Button, FontId::new(18.0, Proportional)),
    (Small, FontId::new(10.0, Proportional)),
    ].into();

    // Mutate global style with above changes
    ctx.set_style(style);
}

impl YTApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        setup_custom_text_style(&cc.egui_ctx);
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()        
    }

    fn nalozi_sliko_iz_poti(&mut self, path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
        let image = image::io::Reader::open(path)?.decode()?;
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        Ok(egui::ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        ))
    }

    fn nalozi_sliko_napaka(&mut self, ctx: &egui::Context) {

        if self.TextureNapaka.is_none(){
            //Dobi podatke iz slike
            let nalozi_sliko = self.nalozi_sliko_iz_poti(Path::new("assets/icon/napaka-icon-36px.png")).expect("Ni možno naložiti slike");

            //Nastavi TextureNapaka na spodnje podatke
            self.TextureNapaka.get_or_insert_with(|| {           

                // Load the texture only once.
                ctx.load_texture(
                    "Napaka_slika",
                    nalozi_sliko,
                    Default::default()
                )
            });
        }
       
    }

}

impl eframe::App for YTApp {

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {     
        
        //Zgronji del (TopPanel)
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Program", |ui| {
                    if ui.button("Zapri").clicked() {
                        frame.close();
                    }
                });

                /*ui.menu_button("Nastavitve", |ui| {
                    ui.add_sized([400.0, 10.0], egui::TextEdit::singleline(potdoytdlp).hint_text("Pot do yt-dlp.exe"));
                });*/

                global_dark_light_mode_switch(ui);
                
            });
        });


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


            IzpisiNapako(self, ctx, self.IDjiZaNapakaWindow[0], "Napaka 1");
            IzpisiNapako(self, ctx, self.IDjiZaNapakaWindow[1], "Napaka 2 dfgfdggggg dsfgedfjigufdn gfdsidgnjdfh sfdsdf");

            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

        });
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
}

fn IzpisiNapako(ytapp: &mut YTApp, ctx: &egui::Context, id_windowa: u16, napaka: &str){

    ytapp.nalozi_sliko_napaka(ctx);

    //Ustvari nov egui window za napako in mu nastavi id
    egui::Window::new(RichText::new("Napaka").size(20.0))
    .id(Id::new(id_windowa))
    .min_width(200.0)
    .min_height(100.0)
    .collapsible(false)
    .resizable(false)
    .open(&mut ytapp.OdprtiOkniZaNapako).show(ctx, |ui| {    
        //Dobi podatke iz TextureNapaka
        let texture: &egui::TextureHandle = &ytapp.TextureNapaka.as_mut().unwrap();

        //Postavi v grid zato da sta v eni vrstici
        egui::Grid::new("586013").show(ui, |ui| {
            ui.image(texture, texture.size_vec2());

            ui.label(napaka);
            ui.end_row();
        });
      

    });
}