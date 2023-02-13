use crate::app::YTApp;
use crate::structs::{Format, Kategorije};

use std::env;
use std::process::Command;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::os::windows::process::CommandExt;
use serde_json::{Value};
use std::fs::File;

use super::skupno::Pretvori_Non_Ascii;


const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn PridobiPodatkeOdVideja(ytapp: &mut YTApp){

    //Preveri da Reciever ni že slučajno povjen (uporabljen)
    if !ytapp.CPReisiverJSONPoln {

        let PotDoYTDLP = ytapp.PotDoYTDLP.as_ref().unwrap().clone();
        let URL = ytapp.URL.clone();

        //Odpre nov kanal za prenos informacij
        let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
        ytapp.CPReisiverJSON = receiver;
        ytapp.CPReisiverJSONPoln = true;

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
            

                //Nalozi Kategorije v spremeljivko, če še nikoli niso bile
                if ytapp.KategorijeAudio.is_empty() || ytapp.KategorijeVideo.is_empty() {
                    NaloziKategorije(ytapp);
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



pub fn NaloziKategorije(ytapp: &mut YTApp){
    //Dobi kategorije iz datoteke
    match File::open("../../assets/config/KategorijeVidejev.json"){
        Err(napaka) => {
            ytapp.CPPosljiPrejeto.napaka = true;
            ytapp.PrikaziNapakoUI = true;
            ytapp.IzpisujNapako = true;
            ytapp.Napaka = format!("JSON: {}", napaka.to_string());
        },
        Ok(datoteka) => {  

            let Kategorije: Kategorije = serde_json::from_reader(datoteka).unwrap();
                for kategorija in Kategorije.Kategorije{
                    ytapp.KategorijeVideo.push(kategorija);
                }
            

        },
        
    }
    
    //Dobi žanre iz datoteke
    match File::open("../../assets/config/ZanraPesmi.json"){
        Err(napaka) => {
            ytapp.CPPosljiPrejeto.napaka = true;
            ytapp.PrikaziNapakoUI = true;
            ytapp.IzpisujNapako = true;
            ytapp.Napaka = format!("JSON: {}", napaka.to_string());
        },
        Ok(datoteka) => {                
            let Kategorije: Kategorije = serde_json::from_reader(datoteka).unwrap();
            for kategorija in Kategorije.Kategorije{
                ytapp.KategorijeAudio.push(kategorija);
            }

        },
        
    }
    

}