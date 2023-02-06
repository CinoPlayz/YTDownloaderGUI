use egui::Ui;
use egui::RichText;
use egui::style::Margin;
use crate::structs::Format;
use crate::app::YTApp;
use crate::Funkcionalnost;
use crate::structs::PrejetoEvent;
use crate::Funkcionalnost::skupno::IzpisiNapako;

pub fn DodajIzgled(ytapp: &mut YTApp,  ui: &mut Ui){
    //Centrira elemente
    ui.vertical_centered(|ui| {

        //Preveri če mora izpisovati napako
        if ytapp.PrikaziNapakoUI == false{
            ytapp.IzpisujNapako = false;
        }

        if ytapp.IzpisujNapako == true{
            let napaka = ytapp.Napaka.clone();
            IzpisiNapako(ytapp, ui.ctx(), ytapp.IDjiZaNapakaWindow[3], &napaka)
        }
        ytapp.Formati.push(Format { Ime: "Ime1".to_string(), ID: "23".to_string(), Vrsta: "Video".to_string()});
        ytapp.Formati.push(Format { Ime: "Ime2".to_string(), ID: "44".to_string(), Vrsta: "Video".to_string()});
    

        ui.heading("Prenesi Videje");

        let margin10 = Margin::same(10.0);

        egui::Frame::none()
        .inner_margin(margin10)
        .show(ui, |ui| {

            let url_label = ui.label("URL: ");
            ui.text_edit_singleline(&mut ytapp.URL).labelled_by(url_label.id);
    
            if ui.button("Pošlji").clicked(){
                ytapp.CPPosljiEvent.kliknjen = true;
                ytapp.CPPrikazujSpinner = true;
            }
        });
      

        //Doda spinner če je kliknjen gumb za pošlji
        if ytapp.CPPrikazujSpinner == true{
            //Nastavi margin za ta spinner
            egui::Frame::none()
            .inner_margin(margin10)
            .show(ui, |ui| {
                ui.add(egui::Spinner::size(egui::Spinner::new(), 20.0));
            });            
        }

        //Doda ComboBox, če ni napak pri pridobivanju informacij za video
        if ytapp.CPPosljiPrejeto.aktivno == true && ytapp.CPPosljiPrejeto.napaka == false {
            let izbrani = &mut ytapp.IzbranFormat;
            egui::ComboBox::from_label("Izberi rezolucijo!")
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

    //Če je bil kliknjen gumb za pridobivanje informacij o videju
    if ytapp.CPPosljiEvent.kliknjen == true{
        //Nastavi struct posljiprejeto na prevzete vrednosti
        ytapp.CPPosljiPrejeto = PrejetoEvent{ ..Default::default()};

        //Pridobi informacije o videju
        Funkcionalnost::podatki_video::PridobiPodatkeOdVideja(ytapp, ctx);  

        ytapp.CPPosljiEvent.kliknjen = false;      
    }
    
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
                        //Odstrani vse kar je za zadnjo \ ali /
                        let mut pot_cela = path.display().to_string();
                        let zandji_slash = pot_cela.rfind('\\').unwrap_or_else(||  pot_cela.rfind('/').unwrap());
                        pot_cela.truncate(zandji_slash);
                        ytapp.PotDoYTDLP = Some(pot_cela);
                    }
                }
                ui.end_row();

                //Izpiše pot do datoteke
                if let Some(pot) = &ytapp.PotDoYTDLP {
                    ui.label("Izbrana mapa:");
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
