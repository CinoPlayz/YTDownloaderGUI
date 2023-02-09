use crate::app::YTApp;

use std::env;
use std::process::Command;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::os::windows::process::CommandExt;


const CREATE_NO_WINDOW: u32 = 0x08000000;


pub fn Prenesi_Video(ytapp: &mut YTApp){
    if !ytapp.CPReisiverPrenosPoln {
        let PotDoYTDLP = ytapp.PotDoYTDLP.as_ref().unwrap().clone();
        let URL = ytapp.URL.clone();

        //Odpre nov kanal za prenos informacij
        let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
        ytapp.CPReisiverPrenos = receiver;
        ytapp.CPReisiverPrenosPoln = true;

        //Preveri da je pot do video nastavljena
        if !ytapp.PotDoVideo.is_some() {
            match sender.send("Izberi lokacijo za shranjevanje videjev".to_string()){
                Ok(_) => {},
                Err(e) => {println!("{}", e)},
            } 
        } 
        else{
            let PotDoVideo = ytapp.PotDoVideo.as_ref().unwrap().clone();

            //Zažene nov thread, kjer izvede postopek za prenos
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
                                //Preveri če je prišlo na napako med izvajanjem
                                let stderr =  String::from_utf8_lossy(&output.stderr);
                                if stderr != ""{
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
}