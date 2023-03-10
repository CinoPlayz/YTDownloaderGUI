use egui::Ui;
use eframe::Frame;
use egui::global_dark_light_mode_switch;

use crate::app::YTApp;

pub fn DodajIzgled(ytapp: &mut YTApp, ui: &mut Ui, frame: &mut Frame){
    ui.menu_button("Program", |ui| {
        if ui.button("Zapri").clicked() {
            frame.close();
        }
        ui.label(format!("Verzija: {}", ytapp.AppVerzija.as_ref().unwrap().clone()));
    });

    ui.menu_button("Nastavitve", |ui|{

        //YT-DLP
        if ui.button("YT-DLP").clicked() {
            ytapp.NastavitveYTDLPEvent.kliknjen = true;
            ytapp.PrikaziNastavitveYTDLPUI = true;
        }

        //Pot za shranjevanje videjev
        if ui.button("Lokacija Videjev").clicked() {
            ytapp.NastavitveLokacijaVidejiEvent.kliknjen = true;
            ytapp.PrikaziNastavitveLokacijaVidejiUI = true;
        }

          //Pot za shranjevanje videjev
          if ui.button("Lokacija Audio").clicked() {
            ytapp.NastavitveLokacijaAudioEvent.kliknjen = true;
            ytapp.PrikaziNastavitveLokacijaAudioiUI = true;
        }

 
      
    });
    

    global_dark_light_mode_switch(ui);

    
}