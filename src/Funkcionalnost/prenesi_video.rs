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


pub fn Prenesi_Video(ytapp: &mut YTApp){
    if !ytapp.CPReisiverPrenosPoln {
        let PotDoYTDLP = ytapp.PotDoYTDLP.as_ref().unwrap().clone();
        let URL = ytapp.URL.clone();

        //Odpre nov kanal za prenos informacij
        let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
        ytapp.CPReisiverPrenos = receiver;
        ytapp.CPReisiverPrenosPoln = true;

        //Preveri, če je mp4, izbran tudi tak kodek, ki se pretvori v mp4
        let mut napaka_prej = false;
        if ytapp.MP4{
            if ytapp.IzbranFormat.ExtFormat != "mp4"{
                napaka_prej = true;
                match sender.send("Izberi kodek, ki podpira mp4".to_string()){
                    Ok(_) => {},
                    Err(e) => {println!("{}", e)},
                } 
            }
        }

        //Preveri da je pot do video nastavljena
        if !ytapp.PotDoVideo.is_some() {
            match sender.send("Izberi lokacijo za shranjevanje videjev".to_string()){
                Ok(_) => {},
                Err(e) => {println!("{}", e)},
            } 
        } 
        else{
            if !napaka_prej{
                //Določi pot do videja
                let mut PotDoVideo = ytapp.PotDoVideo.as_ref().unwrap().clone();
                if PotDoVideo.contains('/'){PotDoVideo.push('/')} else {PotDoVideo.push('\\')} 
                PotDoVideo.push_str(&ytapp.IzbranKategorija);
                if PotDoVideo.contains('/'){PotDoVideo.push('/')} else {PotDoVideo.push('\\')} 
                PotDoVideo.push_str(&ytapp.YTKanal);

                let FormatID = ytapp.IzbranFormat.ID.clone();
                let JeMP4 = ytapp.MP4.clone();


                //Zažene nov thread, kjer izvede postopek za prenos
                thread::spawn(move|| {

                    let mut sporocilo = String::new();

                    //Nastavi env pot tam kjer je yt-dlp
                    match env::set_current_dir(&PotDoYTDLP){
                        Err(_) => {sporocilo = "Izberi pot do YT-DLP".to_string();},
                        Ok(_) => {
                            let FormatPrenos = format!("{}+ba", FormatID);

                            let ruslt_otrok: Result<Child, std::io::Error>;

                            if JeMP4 {
                                ruslt_otrok = Command::new("powershell")
                                .args([r".\yt-dlp.exe", "-f", &FormatPrenos, "-P", &PotDoVideo, "-S res,ext:mp4:m4a --recode mp4", &URL, "--embed-thumbnail", "--restrict-filenames"])
                                .creation_flags(CREATE_NO_WINDOW)
                                .stdout(Stdio::piped())
                                .spawn();
                            }
                            else{
                                ruslt_otrok = Command::new("powershell")
                                .args([r".\yt-dlp.exe", "-f", &FormatPrenos, "-P", &PotDoVideo, &URL, "--embed-thumbnail", "--restrict-filenames"])
                                .creation_flags(CREATE_NO_WINDOW)
                                .stdout(Stdio::piped())
                                .spawn();
                            }
                          

                            match ruslt_otrok {
                                //Preveri če je vredu napisan command
                                Ok(otrok) => {
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

                    //println!("{}", sporocilo);

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
                //Dobi ime datoteke in konec prenosa
                else if sporocilo.contains("[Merger] Merging formats into \"") {
                    //Preveri če je res konec ali se mora pretvoriti v mp4
                    if !ytapp.MP4{
                        ytapp.CPPrenosEvent.kliknjen = false;
                        ytapp.CPReisiverPrenosPoln = false;
                        ytapp.CPPrenosPrejeto = PrejetoEvent{ ..Default::default()};

                        //Dobi podatke od "
                        sporocilo.drain(..sporocilo.find("[Merger] Merging formats into \"").unwrap()+31);
                        

                        sporocilo.truncate(sporocilo.find('"').unwrap());    
                        
                        ytapp.ImeDatoteke = sporocilo;
                    }
                    else{
                        //Preveri če je bilo že preneseno v formatu mp4
                        if sporocilo.contains("[VideoConvertor] Not converting media file \""){
                            ytapp.CPPrenosEvent.kliknjen = false;
                            ytapp.CPReisiverPrenosPoln = false;
                            ytapp.CPPrenosPrejeto = PrejetoEvent{ ..Default::default()};
    
                            //Dobi podatke od "
                            sporocilo.drain(..sporocilo.find("[VideoConvertor] Not converting media file \"").unwrap()+44);
                            sporocilo.truncate(sporocilo.find('"').unwrap());    
                            ytapp.ImeDatoteke = sporocilo;
                        }
                        else{
                            ytapp.CPPrenosEvent.kliknjen = false;
                            ytapp.CPReisiverPrenosPoln = false;
                            ytapp.CPPrenosPrejeto = PrejetoEvent{ ..Default::default()};

                            let PotDoVideo = ytapp.PotDoVideo.as_ref().unwrap().clone();

                            if PotDoVideo.contains('/') {
                                ytapp.ImeDatoteke = format!("{}/{}/{}", &PotDoVideo, ytapp.IzbranKategorija, ytapp.YTKanal);
                            }
                            else{
                                ytapp.ImeDatoteke = format!("{}\\{}\\{}", &PotDoVideo, ytapp.IzbranKategorija, ytapp.YTKanal);
                            }
                            
                            println!("Sporocilo:   {}", sporocilo);
                        }
                        
                    }
                    

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


pub fn OdpriDatoteko(ytapp: &mut YTApp){
    match Command::new("powershell")
    .args([r"ii", "-LiteralPath", format!("'{}'", &ytapp.ImeDatoteke).as_str()])
    .creation_flags(CREATE_NO_WINDOW)
    .output(){
        Ok(_) => {},
        Err(e) => println!("Ne morem odpreti datoteke: {}", e),
    }
}

pub fn OdpriMapo(ytapp: &mut YTApp){
    let mut mapa = ytapp.ImeDatoteke.clone();
    if mapa.rfind('\\').is_some(){
        mapa.truncate(mapa.rfind('\\').unwrap());        
    }
    else {
        mapa.truncate(mapa.rfind('/').unwrap());
    }

    match Command::new("powershell")
    .args(["ii", "-LiteralPath", format!("'{}'", &mapa).as_str()])
    .creation_flags(CREATE_NO_WINDOW)
    .output(){
        Ok(_) => {},
        Err(e) => println!("Ne morem odpreti mape: {}", e),
    }
}