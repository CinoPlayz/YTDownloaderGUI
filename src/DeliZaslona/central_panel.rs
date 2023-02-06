use egui::Ui;
use egui::RichText;
use crate::app::YTApp;
use crate::Funkcionalnost;

pub fn DodajIzgled(ytapp: &mut YTApp,  ui: &mut Ui){
    //Centrira elemente
    ui.vertical_centered(|ui| {
        ui.heading("Prenesi Videje");

        let url_label = ui.label("URL: ");
        ui.text_edit_singleline(&mut ytapp.URL).labelled_by(url_label.id);

        if ui.button("Pošlji").clicked(){
            ytapp.CPPosljiEvent.kliknjen = true;
        }

        if ytapp.CPPosljiEvent.napaka == false {
            let izbrani = &mut ytapp.IzbranFormat;
            let neki = &ytapp.Formati[0].Ime;
            egui::ComboBox::from_label("Select one!")
                .selected_text(format!("{}", izbrani.Ime))
                .show_ui(ui, |ui| {
                    ui.selectable_value( izbrani,  ytapp.Formati[0].clone(), &ytapp.Formati[0].Ime);
                    ui.selectable_value(izbrani,  ytapp.Formati[1].clone(), &ytapp.Formati[1].Ime);
                }
            );

            ui.label(&izbrani.ID);
        }

    });
    
    ui.add(egui::Slider::new(&mut ytapp.age, 0..=120).text("age"));
    if ui.button("Click each year").clicked() {
        ytapp.age += 1;
    }
    //ui.label(format!("Hello '{}', age {}", ytapp.name, ytapp.age));


   
}


pub fn DodajFunkcionalnost(ytapp: &mut YTApp, ctx: &egui::Context){
    Funkcionalnost::podatki_video::PridobiPodatkeOdVideja(ytapp, ctx);
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
