use egui::Align;
use egui::Layout;
use egui::Margin;
use egui::Ui;
use egui::RichText;
use egui::Vec2;
use crate::Funkcionalnost::prenesi_video::OdpriDatoteko;
use crate::Funkcionalnost::prenesi_video::OdpriMapo;
use crate::app::YTApp;
use crate::Funkcionalnost;
use crate::structs::PrejetoEvent;
use crate::Funkcionalnost::skupno::IzpisiNapako;
use crate::Funkcionalnost::skupno::NaloziKategorije;
use crate::structs::Format;

pub fn DodajIzgled(ytapp: &mut YTApp, ui: &mut Ui) {
    //Centrira elemente
    ui.vertical_centered(|ui| {
        //Preveri če mora izpisovati napako
        if ytapp.PrikaziNapakoUI == false {
            ytapp.IzpisujNapako = false;
        }

        if ytapp.IzpisujNapako == true {
            let napaka = ytapp.Napaka.clone();
            IzpisiNapako(ytapp, ui.ctx(), ytapp.IDjiZaNapakaWindow[3], napaka);
        }

        ui.heading("Prenesi Videje");

        let margin10 = Margin::same(10);

        egui::Frame
            ::new()
            .inner_margin(margin10)
            .show(ui, |ui| {
                ui.label("URL: ");
                ui.add(egui::TextEdit::singleline(&mut ytapp.URL).desired_width(420.0));

                //Ni enablan takrat ko pridobiva podatke oz. prenaša video/audio
                if ytapp.CPPosljiEvent.kliknjen {
                    ytapp.CPPosljiEnabled = false;
                } else if ytapp.CPPrenosEvent.kliknjen {
                    ytapp.CPPosljiEnabled = false;
                } else {
                    ytapp.CPPosljiEnabled = true;
                }

                ui.add_enabled_ui(ytapp.CPPosljiEnabled, |ui| {
                    if ui.button("Pošlji").clicked() {
                        ytapp.CPPosljiEvent.kliknjen = true;
                        ytapp.CPPrikazujSpinner = true;
                        ytapp.CPReisiverPrenosPoln = false;
                        ytapp.CPPosljiPrejeto = PrejetoEvent { ..Default::default() };
                        ytapp.ImeDatoteke = String::new();
                        ytapp.MP4 = false;

                        //Izprazne izbrani format (rezolucija kodek)
                        ytapp.IzbranFormat = Format { ..Default::default() };
                    }
                });
            });

        //Doda spinner če je kliknjen gumb za pošlji
        if ytapp.CPPrikazujSpinner == true {
            //Nastavi margin za ta spinner
            egui::Frame
                ::new()
                .inner_margin(margin10)
                .show(ui, |ui| {
                    ui.add(egui::Spinner::size(egui::Spinner::new(), 20.0));
                });
        }

        //Doda ComboBox, če ni napak pri pridobivanju informacij za video
        if ytapp.CPPosljiPrejeto.aktivno == true && ytapp.CPPosljiPrejeto.napaka == false {
            // region: Izbira Tipa
            ui.label("Tip:");
            ui.allocate_ui_with_layout(Vec2::ZERO, Layout::right_to_left(Align::Min), |ui| {
                egui::Frame
                    ::new()
                    .inner_margin(Margin { left: 50, right: 0, top: 0, bottom: 0 })
                    .show(ui, |ui| {
                        egui::ComboBox
                            ::from_id_salt("Izberi Tip!")
                            .selected_text(format!("{}", ytapp.Tip))
                            .width(100.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut ytapp.Tip, "Video".to_string(), "Video");
                                ui.selectable_value(&mut ytapp.Tip, "Audio".to_string(), "Audio");
                            })
                    });
            });
            // endregion

            //Spreminjanje tip in tip prej (Kdaj se pokaže gumb za odpiranje datotek)
            if ytapp.Tip != ytapp.TipPrej {
                ytapp.ImeDatoteke.clear();
                ytapp.TipPrej = ytapp.Tip.clone();
            }

            if ytapp.Tip == "Video" {
                // region: Videjo Izbira

                ui.allocate_ui_with_layout(Vec2::ZERO, Layout::top_down(Align::Center), |ui| {
                    egui::Frame
                    ::new()
                    .inner_margin(Margin { left: -127, ..Default::default()})
                    .outer_margin(Margin { left: -107, top: 20, ..Default::default()})
                    .show(ui, |ui| {
                        egui::Grid
                            ::new("71192")
                            .spacing([20.0, 3.0])
                            .show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label("Rezolucija:");
                                });

                                ui.vertical_centered(|ui| {
                                    ui.horizontal_centered(|ui| {
                                        ui.label("Kodek:");
                                        ui.label("ℹ").on_hover_text(
                                            "AVC(H.264) Slabe kvalitete, vendar zelo podprt\nAV1 Zelo dobre kvalitete, vendar slabo podprt\nVP9 Dobre kvalitete in dobro podprt"
                                        );
                                    });
                                });

                                ui.vertical_centered(|ui| {
                                    ui.label("Kategorija:");
                                });

                                ui.end_row();

                                //Da prevzeto vrednost
                                let dolzina = ytapp.Formati.len();

                                if ytapp.IzbranFormat.ID == "" {
                                    for i in 0..dolzina {
                                        if
                                            ytapp.Formati[i].Rezolucija.contains("1920x") ||
                                            ytapp.Formati[i].Rezolucija.contains("2560x")
                                        {
                                            ytapp.IzbranFormat = ytapp.Formati[i].clone();
                                        }
                                    }
                                }

                                //Rezolucija
                                egui::Frame::new().show(ui, |ui| {
                                    egui::ComboBox
                                        ::from_id_salt("Izberi rezolucijo!")
                                        .selected_text(format!("{}", ytapp.IzbranFormat.Rezolucija))
                                        .width(130.0)
                                        .show_ui(ui, |ui| {
                                            let mut ze_dodani: Vec<String> = Vec::new();

                                            //Izpiše vse rezolucije, ki še niso bile, če je samo mp4
                                            if ytapp.MP4 {
                                                for i in 0..dolzina {
                                                    if
                                                        !ze_dodani.contains(
                                                            &ytapp.Formati[i].Rezolucija
                                                        )
                                                    {
                                                        if ytapp.Formati[i].ExtFormat == "mp4" {
                                                            ui.selectable_value(
                                                                &mut ytapp.IzbranFormat,
                                                                ytapp.Formati[i].clone(),
                                                                &ytapp.Formati[i].Rezolucija
                                                            );
                                                            ze_dodani.push(
                                                                ytapp.Formati[i].Rezolucija.clone()
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                //Izpiše vse rezolucije, ki še niso bile
                                                for i in 0..dolzina {
                                                    if
                                                        !ze_dodani.contains(
                                                            &ytapp.Formati[i].Rezolucija
                                                        )
                                                    {
                                                        ui.selectable_value(
                                                            &mut ytapp.IzbranFormat,
                                                            ytapp.Formati[i].clone(),
                                                            &ytapp.Formati[i].Rezolucija
                                                        );
                                                        ze_dodani.push(
                                                            ytapp.Formati[i].Rezolucija.clone()
                                                        );
                                                    }
                                                }
                                            }
                                        });
                                });

                                //Kodek
                                egui::Frame::new().show(ui, |ui| {
                                    egui::ComboBox
                                        ::from_id_salt("Izberi Kodek!")
                                        .selected_text(
                                            format!("{}", ytapp.IzbranFormat.VideoFormat)
                                        )
                                        .width(130.0)
                                        .show_ui(ui, |ui| {
                                            let dolzina = ytapp.Formati.len();

                                            //Izpiše vse kodeke na izbiro, ki so za mp4
                                            if ytapp.MP4 {
                                                for i in 0..dolzina {
                                                    if
                                                        ytapp.Formati[i].Rezolucija ==
                                                        ytapp.IzbranFormat.Rezolucija
                                                    {
                                                        if ytapp.Formati[i].ExtFormat == "mp4" {
                                                            ui.selectable_value(
                                                                &mut ytapp.IzbranFormat,
                                                                ytapp.Formati[i].clone(),
                                                                &ytapp.Formati[i].VideoFormat
                                                            );
                                                        }
                                                    }
                                                }
                                            } else {
                                                //Izpiše vse kodeke na izbiro
                                                for i in 0..dolzina {
                                                    if
                                                        ytapp.Formati[i].Rezolucija ==
                                                        ytapp.IzbranFormat.Rezolucija
                                                    {
                                                        ui.selectable_value(
                                                            &mut ytapp.IzbranFormat,
                                                            ytapp.Formati[i].clone(),
                                                            &ytapp.Formati[i].VideoFormat
                                                        );
                                                    }
                                                }
                                            }
                                        });
                                });

                                //Kategorije

                                egui::Frame::new().show(ui, |ui| {
                                    egui::ComboBox
                                        ::from_id_salt("Izberi Kategorijo!")
                                        .selected_text(format!("{}", ytapp.IzbranKategorija))
                                        .width(170.0)
                                        .show_ui(ui, |ui| {
                                            let dolzina = ytapp.KategorijeVideo.len();

                                            //Izpiše vse kategorije na izbiro
                                            for i in 0..dolzina {
                                                ui.selectable_value(
                                                    &mut ytapp.IzbranKategorija,
                                                    ytapp.KategorijeVideo[i].clone(),
                                                    &ytapp.KategorijeVideo[i]
                                                );
                                            }
                                        });
                                });

                                ui.end_row();
                            });
                    });
                });

                
                //Download info
                let YTDownloadInfo = format!(
                    "{}   {}   {}",
                    &ytapp.IzbranFormat.ID,
                    &ytapp.IzbranFormat.Rezolucija,
                    &ytapp.IzbranFormat.VideoFormat
                );

                egui::Frame
                ::new()
                .inner_margin(Margin { top: 4, left: -2, ..Default::default() })
                .show(ui, |ui| {
                   ui.label("🔧")
                    .on_hover_text(
                        YTDownloadInfo
                    );                   
                });                

                //MP4
                egui::Frame
                ::new()
                .inner_margin(Margin { top: 20, ..Default::default() })
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("MP4:");
                    });                   
                });

                ui.allocate_ui_with_layout(Vec2::ZERO, Layout::right_to_left(Align::Min), |ui| {
                    egui::Frame
                        ::new()
                        .inner_margin(Margin {
                            left: 32,
                            ..Default::default()
                        })
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.radio_value(&mut ytapp.MP4, true, "DA");
                                ui.radio_value(&mut ytapp.MP4, false, "NE");
                            });
                        });
                });

                egui::Frame
                ::new()
                .inner_margin(Margin { bottom: 10, ..Default::default() }).show(ui, |_|{}); 
                // endregion
            } else {
                // region: Audio Izbira
                ui.allocate_ui_with_layout(Vec2::ZERO, Layout::top_down(Align::Center), |ui| {
                    egui::Frame
                    ::new()
                    .inner_margin(Margin { left: -127, ..Default::default()})
                    .outer_margin(Margin { left: -43, top: 20, bottom: 10, ..Default::default()})
                    .show(ui, |ui| {
                        egui::Grid
                            ::new("71392")
                            .spacing([20.0, 3.0])
                            .show(ui, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.label("Žanra:");
                                });

                                ui.vertical_centered(|ui| {
                                    ui.label("Vrsta Datoteke:");
                                });

                                ui.end_row();

                                egui::Frame::new().show(ui, |ui| {
                                    //Žanra
                                    egui::ComboBox
                                        ::from_id_salt("Izberi Zanro!")
                                        .selected_text(format!("{}", ytapp.IzbranZanra))
                                        .width(155.0)
                                        .show_ui(ui, |ui| {
                                            let dolzina = ytapp.KategorijeAudio.len();

                                            //Izpiše vse kodeke na izbiro
                                            for i in 0..dolzina {
                                                ui.selectable_value(
                                                    &mut ytapp.IzbranZanra,
                                                    ytapp.KategorijeAudio[i].clone(),
                                                    &ytapp.KategorijeAudio[i]
                                                );
                                            }
                                        });
                                });

                                egui::Frame::new().show(ui, |ui| {
                                    //Vrsta
                                    egui::ComboBox
                                        ::from_id_salt("Izberi Koncna vrsta!")
                                        .selected_text(format!("{}", ytapp.IzbranVrsta))
                                        .width(155.0)
                                        .show_ui(ui, |ui| {
                                            let dolzina = ytapp.VrstaDatotek.len();

                                            //Izpiše vse kodeke na izbiro
                                            for i in 0..dolzina {
                                                ui.selectable_value(
                                                    &mut ytapp.IzbranVrsta,
                                                    ytapp.VrstaDatotek[i].clone(),
                                                    &ytapp.VrstaDatotek[i]
                                                );
                                            }
                                        });
                                });

                                ui.end_row();
                            });
                    });
                });

                

                // endregion
            }

            // region: Prenos
            ui.add_enabled_ui(!ytapp.CPPrenosEvent.kliknjen, |ui| {
                if ui.button("Prenesi").clicked() {
                    ytapp.CPPrenosEvent.kliknjen = true;
                    ytapp.CPPrenosPrejeto = PrejetoEvent { ..Default::default() };
                    ytapp.CPReisiverPrenosPoln = false;
                    ytapp.ImeDatoteke = String::new();
                }
            });

            // endregion
        }

        //Preveri če se prenaša
        if ytapp.CPPrenosPrejeto.aktivno == true && ytapp.CPPrenosPrejeto.napaka == false {
            egui::Frame
                ::new()
                .inner_margin(Margin { top: 20, bottom: 20, ..Default::default() })
                .show(ui, |ui| {
                    ui.add(
                        egui::widgets::ProgressBar::show_percentage(
                            egui::widgets::ProgressBar::new(ytapp.CPProcenti)
                        )
                    );
                    ui.label(&ytapp.CPCasPrenos);
                });
        }

        //Preveri če je prenesena datoteka
        if !ytapp.ImeDatoteke.is_empty() {
            egui::Frame
                ::new()
                .inner_margin(Margin { top: 20, bottom: 20, ..Default::default() })
                .show(ui, |ui| {
                    ui.label("Preneseno:");
                    if ui.button("Odpri").clicked() {
                        OdpriDatoteko(ytapp);
                    }

                    if ui.button("Odpri mapo").clicked() {
                        OdpriMapo(ytapp);
                    }
                });
        }
    });
}

pub fn DodajFunkcionalnost(ytapp: &mut YTApp, ctx: &egui::Context) {
    //Nalozi Kategorije v spremeljivko, če še nikoli niso bile
    if ytapp.KategorijeAudio.is_empty() || ytapp.KategorijeVideo.is_empty() {
        NaloziKategorije(ytapp);
    }

    //Če je bil kliknjen gumb za pridobivanje informacij o videju
    if ytapp.CPPosljiEvent.kliknjen == true {
        //Nastavi struct posljiprejeto na prevzete vrednosti
        ytapp.CPPosljiPrejeto = PrejetoEvent { ..Default::default() };

        //Pridobi informacije o videju
        Funkcionalnost::podatki_video::PridobiPodatkeOdVideja(ytapp);
    }

    //Če je bil kliknjen gumb za prenos videju
    if ytapp.CPPrenosEvent.kliknjen == true {
        if ytapp.Tip == "Video" {
            //Prenese video
            Funkcionalnost::prenesi_video::Prenesi_Video(ytapp);
        } else {
            //Prenese audio
            Funkcionalnost::prenesi_audio::Prenesi_Audio(ytapp);
        }

        //Refreša tako dolgo dokler ni konec s prenosom
        ctx.request_repaint();
    }
}

pub fn DodajIzgledInFunkcionalnostZaDruge(ytapp: &mut YTApp, ctx: &egui::Context) {
    // region: Nastavitve

    // region: YTDLP okno
    if ytapp.NastavitveYTDLPEvent.kliknjen {
        //Ustvari novo okno in v njega da možnosti za nastavit
        egui::Window
            ::new(RichText::new("YT-DLP").size(20.0))
            .min_width(200.0)
            .min_height(100.0)
            .default_pos([100.0, 100.0])
            .collapsible(false)
            .resizable(false)
            .open(&mut ytapp.PrikaziNastavitveYTDLPUI)
            .show(ctx, |ui| {
                //Postavi v grid zato da sta v eni vrstici
                egui::Grid::new("87553").show(ui, |ui| {
                    ui.label("Izberi pot do YT-DLP: ");
                    if ui.button("Izberi").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            //Odstrani vse kar je za zadnjo \ ali /
                            let mut pot_cela = path.display().to_string();
                            let zandji_slash = pot_cela
                                .rfind('\\')
                                .unwrap_or_else(|| pot_cela.rfind('/').unwrap());
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

    if ytapp.PrikaziNastavitveYTDLPUI == false {
        ytapp.NastavitveYTDLPEvent.kliknjen = false;
    }

    // endregion

    // region: Videji Lokacija okno
    if ytapp.NastavitveLokacijaVidejiEvent.kliknjen {
        //Ustvari novo okno in v njega da možnosti za nastavit
        egui::Window
            ::new(RichText::new("Lokacija Videjev").size(20.0))
            .min_width(200.0)
            .min_height(100.0)
            .default_pos([100.0, 100.0])
            .collapsible(false)
            .resizable(false)
            .open(&mut ytapp.PrikaziNastavitveLokacijaVidejiUI)
            .show(ctx, |ui| {
                //Postavi v grid zato da sta v eni vrstici
                egui::Grid
                    ::new("88427")
                    .spacing([0.0, 7.0])
                    .show(ui, |ui| {
                        ui.horizontal_centered(|ui| {
                            ui.label("Izberi lokacijo, kjer shranjujem videje: ");
                            if ui.button("Izberi").clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                    ytapp.PotDoVideo = Some(path.display().to_string());
                                }
                            }
                        });

                        ui.end_row();

                        //Izpiše pot do datoteke
                        if let Some(pot) = &ytapp.PotDoVideo {
                            ui.horizontal_centered(|ui| {
                                ui.label("Izbrana lokacija:");
                                ui.monospace(pot);
                            });
                        }

                        ui.end_row();
                    });
            });
    }

    if ytapp.PrikaziNastavitveLokacijaVidejiUI == false {
        ytapp.NastavitveLokacijaVidejiEvent.kliknjen = false;
    }

    // endregion

    // region: Audio Lokacija okno
    if ytapp.NastavitveLokacijaAudioEvent.kliknjen {
        //Ustvari novo okno in v njega da možnosti za nastavit
        egui::Window
            ::new(RichText::new("Lokacija Audio").size(20.0))
            .min_width(200.0)
            .min_height(100.0)
            .default_pos([100.0, 100.0])
            .collapsible(false)
            .resizable(false)
            .open(&mut ytapp.PrikaziNastavitveLokacijaAudioiUI)
            .show(ctx, |ui| {
                //Postavi v grid zato da sta v eni vrstici
                egui::Grid
                    ::new("88427")
                    .spacing([0.0, 7.0])
                    .show(ui, |ui| {
                        ui.horizontal_centered(|ui| {
                            ui.label("Izberi lokacijo, kjer shranjujem audio: ");
                            if ui.button("Izberi").clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                    ytapp.PotDoAudio = Some(path.display().to_string());
                                }
                            }
                        });
                        ui.end_row();

                        //Izpiše pot do datoteke
                        if let Some(pot) = &ytapp.PotDoAudio {
                            ui.horizontal_centered(|ui| {
                                ui.label("Izbrana lokacija:");
                                ui.monospace(pot);
                            });
                        }

                        ui.end_row();
                    });
            });
    }

    if ytapp.PrikaziNastavitveLokacijaAudioiUI == false {
        ytapp.NastavitveLokacijaAudioEvent.kliknjen = false;
    }

    // endregion

    // endregion
}
