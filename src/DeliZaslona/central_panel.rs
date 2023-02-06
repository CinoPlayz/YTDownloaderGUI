use egui::Ui;
use egui::RichText;
use crate::app::YTApp;

pub fn DodajIzgled(ytapp: &mut YTApp,  ui: &mut Ui){
    ui.heading("My egui Application");
    ui.horizontal(|ui| {
        let name_label = ui.label("Your name: ");
        ui.text_edit_singleline(&mut ytapp.name)
            .labelled_by(name_label.id);
    });
    ui.add(egui::Slider::new(&mut ytapp.age, 0..=120).text("age"));
    if ui.button("Click each year").clicked() {
        ytapp.age += 1;
    }
    ui.label(format!("Hello '{}', age {}", ytapp.name, ytapp.age));


   
}

pub fn DodajIzgledInFunkcionalnostZaDruge(ytapp: &mut YTApp, ctx: &egui::Context){

    // region: Nastavitve

    // region: YTDLP okno
    if ytapp.NastavitveYTDLPEvent.kliknjen {            
        //Ustvari novo okno in v njega da možnosti za nastavit
        egui::Window::new(RichText::new("YT-DLP").size(20.0))
        .min_width(200.0)
        .min_height(100.0)
        .default_pos([100.0, 100.0])
        .collapsible(false)
        .resizable(false)
        .open(&mut ytapp.PrikaziNastavitveYTDLPUI).show(ctx, |ui| {    

            //Postavi v grid zato da sta v eni vrstici
            egui::Grid::new("87553").show(ui, |ui| {
                
                ui.label("Izberi pot do YT-DLP: ");
                if ui.button("Izberi").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        ytapp.PotDoYTDLP = Some(path.display().to_string());
                    }
                }
                ui.end_row();

                //Izpiše pot do datoteke
                if let Some(pot) = &ytapp.PotDoYTDLP {
                    ui.label("Izbrana datoteka:");
                    ui.monospace(pot);
                }

                ui.end_row();
            });
        

        });

        

    }
    
    if ytapp.PrikaziNastavitveYTDLPUI == false{
        ytapp.NastavitveYTDLPEvent.kliknjen = false;
    }

    // endregion



    // region: Videji Lokacija okno
    if ytapp.NastavitveLokacijaVidejiEvent.kliknjen {            
        //Ustvari novo okno in v njega da možnosti za nastavit
        egui::Window::new(RichText::new("Lokacija Videjev").size(20.0))
        .min_width(200.0)
        .min_height(100.0)
        .default_pos([100.0, 100.0])
        .collapsible(false)
        .resizable(false)
        .open(&mut ytapp.PrikaziNastavitveLokacijaVidejiUI).show(ctx, |ui| {    

            //Postavi v grid zato da sta v eni vrstici
            egui::Grid::new("88427").show(ui, |ui| {
                
                ui.label("Izberi lokacijo, kjer shranjujem videje: ");
                if ui.button("Izberi").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        ytapp.PotDoVideo = Some(path.display().to_string());
                    }
                }
                ui.end_row();

                //Izpiše pot do datoteke
                if let Some(pot) = &ytapp.PotDoVideo {
                    ui.label("Izbrana lokacija:");
                    ui.monospace(pot);
                }

                ui.end_row();
            });
        

        });

        

    }
    
    if ytapp.PrikaziNastavitveLokacijaVidejiUI == false{
        ytapp.NastavitveLokacijaVidejiEvent.kliknjen = false;
    }

    // endregion


    // endregion
}
