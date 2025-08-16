use egui::{Context, Ui};
use eframe::Frame;
use egui::global_theme_preference_switch;

use crate::app::YTApp;

pub fn DodajIzgled(ytapp: &mut YTApp, ui: &mut Ui, ctx: &Context) {
    ui.menu_button("Program", |ui| {
        if ui.button("Zapri").clicked() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        ui.label(format!("Verzija: {}", ytapp.AppVerzija.as_ref().unwrap().clone()));
    });

    ui.menu_button("Nastavitve", |ui| {
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

    global_theme_preference_switch(ui);
}
