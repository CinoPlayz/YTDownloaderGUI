use crate::app::{YTApp, Format};

pub fn PridobiPodatkeOdVideja(ytapp: &mut YTApp, ctx: &egui::Context){
    ytapp.Formati.push(Format { Ime: "Ime1".to_string(), ID: "23".to_string() });
    ytapp.Formati.push(Format { Ime: "Ime2".to_string(), ID: "44".to_string() })
}