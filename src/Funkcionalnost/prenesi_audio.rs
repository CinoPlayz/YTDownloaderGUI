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