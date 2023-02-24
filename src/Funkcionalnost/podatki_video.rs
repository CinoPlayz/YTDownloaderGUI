use crate::app::YTApp;
use crate::structs::Format;
use std::env;
use std::process::Command;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::os::windows::process::CommandExt;
use serde_json::Value;

use super::skupno::Pretvori_Non_Ascii;


const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn PridobiPodatkeOdVideja(ytapp: &mut YTApp){

    //Preveri da Reciever ni že slučajno povjen (uporabljen)
    if !ytapp.CPReisiverJSONPoln {

        let mut PotDoYTDLP = String::new();
        match &ytapp.PotDoYTDLP {
            Some(pot) => {PotDoYTDLP.push_str(&pot);},
            None => {PotDoYTDLP.push_str("Pot ni podana.");},
        }

        let URL = ytapp.URL.clone();

        //Odpre nov kanal za prenos informacij
        let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
        ytapp.CPReisiverJSON = receiver;
        ytapp.CPReisiverJSONPoln = true;

        //Preveri da je pot do YTDLP podana
        if PotDoYTDLP == "Pot ni podana."{
            match sender.send("Pot do YT-DLP ni podana".to_string()){
                Ok(_) => {},
                Err(e) => {println!("{}", e)},
            } 
        }
        //Preveri da ni slučajno playlist
        else if URL.contains(".com/playlist?list"){
            match sender.send("Možnost za playlist še ni implemintirana".to_string()){
                Ok(_) => {},
                Err(e) => {println!("{}", e)},
            } 
        }
        else if URL.contains(".com/c/") || URL.contains(".com/@"){
            match sender.send("Možnost za kanale še ni implemintirana".to_string()){
                Ok(_) => {},
                Err(e) => {println!("{}", e)},
            } 
        }
        else{
            //Zažene nov thread, kjer izvede postopek za pridobivanje informacij
            thread::spawn(move|| {

                let sporocilo;

                //Nastavi env pot tam kjer je yt-dlp
                match env::set_current_dir(&PotDoYTDLP){
                    Err(_) => {sporocilo = "Izberi pot do YT-DLP".to_string();},
                    Ok(_) => {
                        let output = Command::new("powershell")
                        .args([r".\yt-dlp.exe", "-j", &URL])
                        .creation_flags(CREATE_NO_WINDOW)
                        .output();

                        match output {
                            //Preveri če je vredu napisan command
                            Ok(output) => {
                                //Preveri če je prišlo na napako med izvajanjem in da ni to samo WARNING
                                let stderr =  String::from_utf8_lossy(&output.stderr);
                                if stderr != "" && !stderr.contains("WARNING:"){
                                    sporocilo = stderr.to_string();                                
                                }
                                else{
                                    //AVC(H.264) videji so slabe kvalitete, vendar podprti skoraj da vsepovsod
                                    //AV1 videji so zelo dobre kvalitete, vendar ne tako dobro podprti
                                    //VP9 so dobre kvalitete in podprti kar dobro

                                    sporocilo = format!("[ok]{}", String::from_utf8_lossy(&output.stdout).to_string());

                                }
                                
                            },
                            Err(e) => {
                                sporocilo = e.to_string();
                            },
                        } 
                    }
                    
                }

                match sender.send(sporocilo){
                    Ok(_) => {},
                    Err(e) => {println!("{}", e)},
                }  
            });
        }

        
    }
   

    //Preveri če je pridobil kakšno informacijo
    match ytapp.CPReisiverJSON.try_recv() {
        Ok(mut sporocilo) => {

            ytapp.CPPosljiPrejeto.aktivno = true;

            //Dobi sporocilo, če je vredu vse potem so prve 4 byti [ok]
            let mut prvi_4 = sporocilo.clone();
            prvi_4.truncate(4);           


            //Preveri če je bila kakšna napaka
            if prvi_4 != "[ok]" {
                ytapp.CPPosljiPrejeto.napaka = true;
                ytapp.CPPosljiEvent.kliknjen = false;
                
                ytapp.PrikaziNapakoUI = true;
                ytapp.IzpisujNapako = true;
                ytapp.Napaka = sporocilo;
            }
            else{
                ytapp.CPPosljiPrejeto.napaka = false;

                //Odstrani prve štiri znake
                sporocilo.remove(0);
                sporocilo.remove(0);
                sporocilo.remove(0);
                sporocilo.remove(0);

                ytapp.Formati.clear();

                let v: Value = serde_json::from_str(&sporocilo).unwrap();

                if let Some(formats) = v["formats"].as_array(){
                    for format in formats{

                        //Preveri da je tak format, kjer je videjo in nič audia
                        if format["video_ext"] != "none" && format["audio_ext"] == "none" {
                            let id = format["format_id"].to_string().replace("\"", "");
                            let mut video_format = format["vcodec"].to_string().replace("\"", "");
                            let rezolucija = format["resolution"].to_string().replace("\"", "");
                            let ext_format = format["video_ext"].to_string().replace("\"", "");

                            if video_format.contains("av01"){ video_format= "AV1".to_string();}
                            else if video_format.contains("avc"){ video_format= "AVC".to_string();}
                            else if video_format.contains("vp9"){ video_format= "VP9".to_string();}

                            ytapp.Formati.push(Format { ID: id, VideoFormat: video_format, Rezolucija: rezolucija, ExtFormat: ext_format });
                        }
                    }
                }

                //Dobi ime kanala v ascii obliki
                ytapp.YTKanal = Pretvori_Non_Ascii(v["channel"].to_string());
        
                //Dobi tage ter pregleda, če je kakšen enak kategoriji
                if let Some(tags) = v["tags"].as_array(){
                    for tag in tags{
                        let tag_upper = tag.to_string().to_uppercase().replace("\"", "");
                        let mut najden = false;

                        //Preveri če je tag enak enemu izmed kategorij za video
                        let kategorije = ytapp.KategorijeVideo.clone();
                        let dolzina = kategorije.len();

                        for i in 0..dolzina{
                            let kategorija = kategorije[i].clone().to_uppercase();

                            if kategorija == tag_upper{
                                ytapp.IzbranKategorija = kategorije[i].clone();
                                najden = true;
                                break;
                            }

                        }


                        //Preveri če je tag enak enemu izmed žanr za audio
                        if !najden{
                            let zanre = ytapp.KategorijeAudio.clone();
                            let dolzina = zanre.len();

                            for i in 0..dolzina{
                                let zanra = zanre[i].clone().to_uppercase();

    
                                if zanra == tag_upper{
                                    ytapp.IzbranZanra = zanre[i].clone();
                                    break;
                                }
    
                            }
                        }

                        
                    }
                }
            

                
                
            }
            
            //Neha prikazovati spinner
            ytapp.CPPrikazujSpinner = false;
            ytapp.CPPosljiEvent.kliknjen = false;
            ytapp.CPReisiverJSONPoln = false;
        },
        Err(_) => {
        },
    }
    
}

