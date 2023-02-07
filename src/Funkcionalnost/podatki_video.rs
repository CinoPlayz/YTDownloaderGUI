use crate::app::YTApp;
use crate::structs::Format;

use std::env;
use std::process::Command;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::os::windows::process::CommandExt;

const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn PridobiPodatkeOdVideja(ytapp: &mut YTApp, _ctx: &egui::Context){
    ytapp.Formati.push(Format { Ime: "Ime1".to_string(), ID: "23".to_string(), Vrsta: "Video".to_string()});
    ytapp.Formati.push(Format { Ime: "Ime2".to_string(), ID: "44".to_string(), Vrsta: "Video".to_string()});
         

    //Preveri da Reciever ni že slučajno povjen (uporabljen)
    if !ytapp.CPReisiverJSONPoln {

        let PotDoYTDLP = ytapp.PotDoYTDLP.as_ref().unwrap().clone();
        let URL = ytapp.URL.clone();

        //Odpre nov kanal za prenos informacij
        let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
        ytapp.CPReisiverJSON = receiver;
        ytapp.CPReisiverJSONPoln = true;
        println!("reciever");

        //Zažene nov thread, kjer izvede postopek za pridobivanje informacij
        thread::spawn(move|| {

            let mut sporocilo = String::from("Ok");

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

                            println!("status: {}", output.status);
                            //println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                            
                        },
                        Err(e) => {
                            sporocilo = e.to_string();
                        },
                    } 
                }
                
            }

            match sender.send(sporocilo){
                Ok(_) => {println!("Poslano...")},
                Err(e) => {println!("{}", e)},
            }  
        });
    }
   

    //Preveri če je pridobil kakšno informacijo
    match ytapp.CPReisiverJSON.try_recv() {
        Ok(sporocilo) => {

            ytapp.CPPosljiPrejeto.aktivno = true;

            //Preveri če je bila kakšna napaka
            if sporocilo != "Ok" {
                ytapp.CPPosljiPrejeto.napaka = true;
                
                ytapp.PrikaziNapakoUI = true;
                ytapp.IzpisujNapako = true;
                ytapp.Napaka = sporocilo;
            }
            else{
                ytapp.CPPosljiPrejeto.napaka = false;
            }
            
            //Neha prikazovati spinner
            ytapp.CPPrikazujSpinner = false;
            ytapp.CPPosljiEvent.kliknjen = false;
            ytapp.CPReisiverJSONPoln = false;
        },
        Err(_) => {
            //println!("tuk2: {}", e);
            //ctx.request_repaint();
        },
    }
    
}