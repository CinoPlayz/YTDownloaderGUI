use crate::app::YTApp;
use crate::structs::PrejetoEvent;

use std::env;
use std::io::{BufReader, BufRead};
use std::process::{Command, Stdio, Child};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use std::os::windows::process::CommandExt;
use regex::Regex;


const CREATE_NO_WINDOW: u32 = 0x08000000;


pub fn Prenesi_Audio(ytapp: &mut YTApp){
    if !ytapp.CPReisiverPrenosPoln {
        let PotDoYTDLP = ytapp.PotDoYTDLP.as_ref().unwrap().clone();
        let URL = ytapp.URL.clone();
        let IzbranaVrsta = ytapp.IzbranVrsta.clone();

        //Odpre nov kanal za prenos informacij
        let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
        ytapp.CPReisiverPrenos = receiver;
        ytapp.CPReisiverPrenosPoln = true;

        //Preveri da je pot do video nastavljena
        if !ytapp.PotDoAudio.is_some() {
            match sender.send("Izberi lokacijo za shranjevanje audio".to_string()){
                Ok(_) => {},
                Err(e) => {println!("{}", e)},
            } 
        } 
        else{
            //Določi pot do audio
            let mut PotDoAudio = ytapp.PotDoAudio.as_ref().unwrap().clone();
            if PotDoAudio.contains('/'){PotDoAudio.push('/')} else {PotDoAudio.push('\\')} 
            PotDoAudio.push_str(&ytapp.IzbranZanra);
            if PotDoAudio.contains('/'){PotDoAudio.push('/')} else {PotDoAudio.push('\\')} 
            PotDoAudio.push_str(&ytapp.YTKanal);


            thread::spawn(move|| {

                let mut sporocilo = String::new();

                //Nastavi env pot tam kjer je yt-dlp
                match env::set_current_dir(&PotDoYTDLP){
                    Err(_) => {sporocilo = "Izberi pot do YT-DLP".to_string();},
                    Ok(_) => {
                        let result_otrok: Result<Child, std::io::Error>;

                        //Preveri če lahko ta tip podpira thumbnaile
                        if IzbranaVrsta == "mp3" || IzbranaVrsta == "m4a"{
                            result_otrok = Command::new("powershell")
                            .args([r".\yt-dlp.exe", "-f", "ba", "-x --audio-format", &IzbranaVrsta,  "-P", &PotDoAudio, &URL, "--embed-thumbnail", "--restrict-filenames"])
                            .creation_flags(CREATE_NO_WINDOW)
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped())
                            .spawn();
                        }
                        else{
                            result_otrok = Command::new("powershell")
                            .args([r".\yt-dlp.exe", "-f", "ba", "-x --audio-format", &IzbranaVrsta,  "-P", &PotDoAudio, &URL, "--restrict-filenames"])
                            .creation_flags(CREATE_NO_WINDOW)
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped())
                            .spawn();
                        }


                        match result_otrok {
                            //Preveri če je vredu napisan command
                            Ok(otrok) => {

                                //Če ni napaka v izpisu
                                match otrok.stdout {
                                    Some(stdout) => {
                                            
                                        //Ustvari se nov BufReader v katerega se zapišejo podatki iz stdout
                                        let stdout_reader = BufReader::new(stdout);
                        
                                        //Razdeli kar je BufReaderju v to da se izpiše vsaka posodobitev commandlina in da rezultate v array vrednost, ki obstaja 
                                        let stdout_lines2 = stdout_reader.split(b'\r');

                                        //Gre čez linije, ki se prikažejo v powershellu preveri da niso prazne in jih pošlje po channelju
                                        for line in stdout_lines2 {
                                            match line{
                                                Ok(podatek) => {

                                                    //Sestavi sporocilo
                                                    sporocilo = String::from("[ok]");
                                                    let podatek_string = String::from_utf8_lossy(&podatek).to_string();
                                                    
                                                    //Preveri če je prenos z števili                                                    
                                                    let regex = Regex::new(r":\d\d$").unwrap();

                                                    
                                                    if regex.is_match(&podatek_string){

                                                        let mut counter = 0;
                                                        let mut procent = String::new();
                                                        let mut cas = String::new();
    
                                                        for beseda in podatek_string.split_whitespace(){
                                                            counter += 1;
    
                                                            //Dobi drugo besedo iz tega stringa (ostanek)
                                                            if counter == 2 {
                                                                procent = beseda.to_string();
                                                            }
    
                                                            //Dobi vse zadnje besede (hitrost in cas)
                                                            if counter > 5 {
                                                                cas.push_str(beseda);
                                                                cas.push(' ');
                                                            }
                                                        }
    
                                                        sporocilo.push_str(format!("[prenos]{}|{}", procent, cas).as_str());
                                                        
                                                        
                                                    }
                                                    else{
                                                        sporocilo.push_str(&podatek_string);
                                                    }
                                                

                                                    //Poslje sporocilo
                                                    match sender.send(sporocilo.clone()){
                                                        Ok(_) => {},
                                                        Err(e) => {println!("{}", e)},
                                                    } 

                                                },
                                                Err(e) => eprintln!("Nekaj narobe s podatkom: {}", e),
                                            }
                                        
                                        
                                        }
                                        
                                    }
                                    None => println!("Napaka pri otroku")
                                }

                                //Če je napaka v izpisu
                                match otrok.stderr{
                                    Some(stderr) => {
                                        //Ustvari se nov BufReader v katerega se zapišejo podatki iz stdout
                                        let stderr_reader = BufReader::new(stderr);
                        
                                        //Razdeli kar je BufReaderju v to da se izpiše vsaka posodobitev commandlina in da rezultate v array vrednost, ki obstaja 
                                        let stderr_lines2 = stderr_reader.split(b'\r');

                                        //Gre čez linije, ki se prikažejo v powershellu preveri da niso prazne in jih pošlje po channelju
                                        for line in stderr_lines2 {
                                            match line{
                                                Ok(podatek) => {

                                                    //Sestavi napako
                                                    let podatek_string = String::from_utf8_lossy(&podatek).to_string();
                                                                                       
                                                    //Preveri da ni warning ampak samo error
                                                    if !podatek_string.contains("WARNING:"){
                                                        //Poslje napako
                                                        match sender.send(podatek_string.clone()){
                                                            Ok(_) => {},
                                                            Err(e) => {println!("{}", e)},
                                                        } 
                                                    }
                                                    
                                                    

                                                },
                                                Err(e) => eprintln!("Nekaj narobe s podatkom: {}", e),
                                            }
                                        }
                                    },
                                    None => {},
                                }
                            }

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
     match ytapp.CPReisiverPrenos.try_recv() {
        Ok(mut sporocilo) => {

            ytapp.CPPrenosPrejeto.aktivno = true;

            //Dobi sporocilo, če je vredu vse potem so prve 4 byti [ok]
            let mut prvi_4 = sporocilo.clone();
            prvi_4.truncate(4);  
            

            //Preveri če je bila kakšna napaka
            if prvi_4 != "[ok]" {
                ytapp.CPPrenosPrejeto.napaka = true;
                ytapp.CPPrenosEvent.kliknjen = false;
                
                ytapp.PrikaziNapakoUI = true;
                ytapp.IzpisujNapako = true;
                ytapp.Napaka = sporocilo;
            }
            else{
                ytapp.CPPrenosPrejeto.napaka = false;
                
                //Odstrani prve štiri znake
                sporocilo = sporocilo.replace("[ok]", "");

                //Preveri da je podatek za procent in cas
                if sporocilo.contains("[prenos]"){
                    sporocilo = sporocilo.replace("[prenos]", "");

                    //Nastavi Cas za prenos
                    let cas = sporocilo.split_off(sporocilo.find('|').unwrap()+1);
                    ytapp.CPCasPrenos = cas;
                    
                    //Odstrani %
                    sporocilo.pop();
                    sporocilo.pop();
                   

                    //Pretvori v f32
                    let pretvori_v_f32 = sporocilo.parse::<f32>().unwrap();
                    
                    ytapp.CPProcenti = pretvori_v_f32 / 100.0;
                }
                //Dobi ime datoteke in konec prenosa, če ni treba thumbnaila
                else if sporocilo.contains("[ExtractAudio] Destination:") && (ytapp.IzbranVrsta == "wav" || ytapp.IzbranVrsta == "aac") {
                    
                    
                    if ytapp.IzbranVrsta == "mp3" || ytapp.IzbranVrsta == "m4a"{
                      
                    }
                    else{
                        ytapp.CPPrenosEvent.kliknjen = false;
                        ytapp.CPReisiverPrenosPoln = false;
                        ytapp.CPPrenosPrejeto = PrejetoEvent{ ..Default::default()};

                        //Dobi podatke od "
                        sporocilo.drain(..sporocilo.find("[ExtractAudio] Destination: ").unwrap()+28);
                        

                        sporocilo.truncate(sporocilo.find("\n").unwrap());    
                        
                        ytapp.ImeDatoteke = sporocilo;
                    }

                }
                //Preveri če je res konec ali se mora dodati thumbnail (mp3)
                else if sporocilo.contains("[EmbedThumbnail] ffmpeg: Adding thumbnail to"){
                    ytapp.CPPrenosEvent.kliknjen = false;
                    ytapp.CPReisiverPrenosPoln = false;
                    ytapp.CPPrenosPrejeto = PrejetoEvent{ ..Default::default()};

                    //Dobi podatke od "
                    sporocilo.drain(..sporocilo.find("[EmbedThumbnail] ffmpeg: Adding thumbnail to \"").unwrap()+46);
                    

                    sporocilo.truncate(sporocilo.find('"').unwrap());    
                    
                    ytapp.ImeDatoteke = sporocilo;
                }
                 //Preveri če je res konec ali se mora dodati thumbnail (m4a)
                 else if sporocilo.contains("[EmbedThumbnail] mutagen: Adding thumbnail to"){
                    ytapp.CPPrenosEvent.kliknjen = false;
                    ytapp.CPReisiverPrenosPoln = false;
                    ytapp.CPPrenosPrejeto = PrejetoEvent{ ..Default::default()};

                    //Dobi podatke od "
                    sporocilo.drain(..sporocilo.find("[EmbedThumbnail] mutagen: Adding thumbnail to \"").unwrap()+47);
                    

                    sporocilo.truncate(sporocilo.find('"').unwrap());    
                    
                    ytapp.ImeDatoteke = sporocilo;
                }
                else if sporocilo.contains(" has already been downloaded"){
                    ytapp.CPPrenosEvent.kliknjen = false;
                    ytapp.CPReisiverPrenosPoln = false;
                    ytapp.CPPrenosPrejeto = PrejetoEvent{ ..Default::default()};


                    //Dobi ime datoteke naprej vse od [download] , ter nato vse do  has already been downloaded
                    sporocilo.drain(..sporocilo.rfind("[download] ").unwrap()+11);

                    sporocilo.truncate(sporocilo.rfind(" has already been downloaded").unwrap());

                    ytapp.ImeDatoteke = sporocilo;
                }
                

            }

        }
        Err(_) => {
        },
    }
}