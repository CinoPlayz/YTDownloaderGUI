
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

use crate::DeliZaslona;
//use crate::Funkcionalnost;
use crate::structs::GumbEvent;
use crate::structs::PrejetoEvent;
use crate::structs::Format;
use crate::Funkcionalnost::skupno::IzpisiNapako;


#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct YTApp {
    pub URL: String,
    pub age: u32,
    pub PotDoYTDLP: Option<String>,
    pub PotDoVideo: Option<String>,
    

    //Kar se ne rabi shraniti

    //Napaka
    #[serde(skip)]
    pub TextureNapaka: Option<egui::TextureHandle>,
    #[serde(skip)]
    pub PrikaziNapakoUI: bool,
    #[serde(skip)]
    pub IDjiZaNapakaWindow: Vec<u16>,
    #[serde(skip)]
    pub IzpisujNapako: bool,
    #[serde(skip)]
    pub Napaka: String,

    //Nastavitve
    #[serde(skip)]
    pub PrikaziNastavitveYTDLPUI: bool,
    #[serde(skip)]
    pub NastavitveYTDLPEvent: GumbEvent,
    #[serde(skip)]
    pub PrikaziNastavitveLokacijaVidejiUI: bool,
    #[serde(skip)]
    pub NastavitveLokacijaVidejiEvent: GumbEvent,

    //Central-Panel
    #[serde(skip)]
    pub CPPosljiEvent: GumbEvent,
    #[serde(skip)]
    pub CPPosljiPrejeto: PrejetoEvent,
    #[serde(skip)]
    pub CPPrikazujSpinner: bool,
    #[serde(skip)]
    pub CPReisiverJSON: Receiver<String>,
    #[serde(skip)]
    pub CPReisiverJSONPoln: bool,



    //Funkcionalnost
    #[serde(skip)]
    pub Formati: Vec<Format>,
    #[serde(skip)]
    pub IzbranFormat: Format,
    #[serde(skip)]
    pub Kategorije: Vec<String>,
    #[serde(skip)]
    pub IzbranKategorija: String,


}

impl Default for YTApp {
    fn default() -> Self {
        Self {
            URL: "".to_owned(),
            age: 42,
            PotDoYTDLP: None,
            PotDoVideo: None,
            IDjiZaNapakaWindow: Vec::from([64345, 38015, 41661, 32302, 35660, 64159, 48057, 12441, 15910, 48957, 
                30690, 29088, 22894, 54035, 19348, 34923, 59481, 45316, 46313, 50076]),
            TextureNapaka: None,           
            PrikaziNapakoUI: true,  
            IzpisujNapako: false,    
            Napaka: "".to_string(),   
            
            //Nastavitve
            PrikaziNastavitveYTDLPUI: true,
            NastavitveYTDLPEvent: GumbEvent { kliknjen: false },
            PrikaziNastavitveLokacijaVidejiUI: true,
            NastavitveLokacijaVidejiEvent: GumbEvent { kliknjen: false},

            //Central-Panel
            CPPosljiEvent: GumbEvent { kliknjen: false },
            CPPosljiPrejeto: PrejetoEvent {..Default::default() },
            CPPrikazujSpinner: false,
            CPReisiverJSON: mpsc::channel().1,
            CPReisiverJSONPoln: false,

            //Funkcionalnost
            Formati: Vec::new(),
            IzbranFormat: Format { ..Default::default()},
            Kategorije: Vec::new(),
            IzbranKategorija: String::new(),
            
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

   

}

impl eframe::App for YTApp {

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {     
        
        //Zgronji del (TopPanel)
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            
            //Doda menubar
            egui::menu::bar(ui, |ui| {

                //Doda izgled in funkcionalnost temu menu baru
                DeliZaslona::menu_bar::DodajIzgled(self, ui, frame);                
                
            });
        });


        egui::CentralPanel::default().show(ctx, |ui| {
            
            DeliZaslona::central_panel::DodajIzgled(self, ui);
            DeliZaslona::central_panel::DodajFunkcionalnost(self, ctx);

            
      
            DeliZaslona::central_panel::DodajIzgledInFunkcionalnostZaDruge(self, ctx);
            
            ui.label("neki \n dsfsd");

            IzpisiNapako(self, ctx, self.IDjiZaNapakaWindow[0], "Napaka 1". to_string());
            let string = "line one \n line two";
            IzpisiNapako(self, ctx,  self.IDjiZaNapakaWindow[1], string.to_string());
        

        });
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }
}

