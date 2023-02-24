use crate::YTApp;
use crate::structs::Kategorije;
use std::path::Path;
use egui::RichText;
use egui::Id;
use std::fs::File;

fn nalozi_sliko_iz_poti(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

pub fn nalozi_sliko_napaka(ytapp: &mut YTApp, ctx: &egui::Context) {

    if ytapp.TextureNapaka.is_none(){
        //Dobi podatke iz slike
        let nalozi_sliko = nalozi_sliko_iz_poti(Path::new("assets/icon/napaka-icon-36px.png")).expect("Ni možno naložiti slike");

        //Nastavi TextureNapaka na spodnje podatke
        ytapp.TextureNapaka.get_or_insert_with(|| {           

            // Load the texture only once.
            ctx.load_texture(
                "Napaka_slika",
                nalozi_sliko,
                Default::default()
            )
        });
    }
   
}

pub fn IzpisiNapako(ytapp: &mut YTApp, ctx: &egui::Context, ID: u16, napaka: String){

    //Ustvari nov egui window za napako in mu nastavi id
    egui::Window::new(RichText::new("Napaka").size(20.0))
    .id(Id::new(ID))
    .min_width(200.0)
    .min_height(100.0)        
    .collapsible(false)
    .resizable(false)
    .default_pos([125.0, 200.0])
    .open(&mut ytapp.PrikaziNapakoUI).show(ctx, |ui| {    
        //Dobi podatke iz TextureNapaka
        let texture: &egui::TextureHandle = &ytapp.TextureNapaka.as_mut().unwrap();

        //Dobi besedilo razdeljeno na več vrstič
        let napaka_multi_line = NarediNoveVrstice(napaka);

        //Postavi v grid zato da sta v eni vrstici
        egui::Grid::new("586013").show(ui, |ui| {
            ui.image(texture, texture.size_vec2());
            ui.label(egui::RichText::new(napaka_multi_line));
            ui.end_row();
        });
      

    });
}


pub fn NarediNoveVrstice(besedilo: String) -> String{
    let mut novo_besedilo = String::new();
    let mut koliko_bytov: u32 = 0;
    let mut nova_beseda;

    //Na vsakih 100 bajtov naredi novo vrstico
    for beseda in besedilo.split_whitespace(){

        nova_beseda = String::from(beseda);
        koliko_bytov += beseda.len() as u32;

        if koliko_bytov > 30 {
            nova_beseda.push_str("\n");
            koliko_bytov = 0;
        }
        else{
            nova_beseda.push(' ');
        }


        novo_besedilo.push_str(nova_beseda.as_str());
    }   

    return  novo_besedilo;

    
}


pub fn NaloziKategorije(ytapp: &mut YTApp){
    //Dobi kategorije iz datoteke
    match File::open("assets/config/KategorijeVidejev.json"){
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
    match File::open("assets/config/ZanraPesmi.json"){
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



pub fn Pretvori_Non_Ascii(string: String) -> String{
    let mut ascii_string: String = String::new();

    for znak in string.chars(){
        if znak.is_ascii() {
            ascii_string.push(znak)
        }
        else{
            match znak {
                '¡' | '¿' | '·' => ascii_string.push('.'),
                'Ä' | 'ä' | 'À' | 'à' | 'Á' | 'á' | 'Â' | 'â' | 'Ã' | 'ã' | 'Å' | 'å' | 'Ǎ' | 'ǎ' | 'Ą' | 'ą' | 'Ă' | 'ă' | 'Æ' | 'æ' | 'Ā' | 'ā' => if znak.is_uppercase() { ascii_string.push('A') } else { ascii_string.push('a') },
                'Ç' | 'ç' | 'Ć' | 'ć' | 'Ĉ' | 'ĉ' | 'Č' | 'č' => if znak.is_uppercase() { ascii_string.push('C') } else {ascii_string.push('c') },
                'Ď' | 'đ' | 'Đ' | 'ď' | 'ð' => if znak.is_uppercase() { ascii_string.push('D')} else { ascii_string.push('d')},
                'È' | 'è' | 'É' | 'é' | 'Ê' | 'ê' | 'Ë' | 'ë' | 'Ě' | 'ě' | 'Ę' | 'ę' | 'Ė' | 'ė' | 'Ē' | 'ē' => if znak.is_uppercase() { ascii_string.push('E')} else { ascii_string.push('e')},
                'Ĝ' | 'ĝ' | 'Ģ' | 'ģ' | 'Ğ' | 'ğ' => if znak.is_uppercase() { ascii_string.push('G')} else { ascii_string.push('g')},
                'Ĥ' | 'ĥ' => if znak.is_uppercase() { ascii_string.push('H')} else { ascii_string.push('h')},
                'Ì' | 'ì' | 'Í' | 'í' | 'Î' | 'î' | 'Ï' | 'ï' | 'ı' | 'Ī' | 'ī' | 'Į' | 'į' => if znak.is_uppercase() { ascii_string.push('I')} else { ascii_string.push('i')},
                'Ĵ' | 'ĵ' => if znak.is_uppercase() { ascii_string.push('J')} else { ascii_string.push('j')},
                'Ķ' | 'ķ' => if znak.is_uppercase() { ascii_string.push('K')} else { ascii_string.push('k')},
                'Ĺ' | 'ĺ' | 'Ļ' | 'ļ' | 'Ł' | 'ł' | 'Ľ' | 'ľ' | 'Ŀ' | 'ŀ' => if znak.is_uppercase() { ascii_string.push('L')} else { ascii_string.push('l')},
                'Ñ' | 'ñ' | 'Ń' | 'ń' | 'Ň' | 'ň' | 'Ņ' | 'ņ' => if znak.is_uppercase() { ascii_string.push('N')} else { ascii_string.push('n')},
                'Ö' | 'ö' | 'Ò' | 'ò' | 'Ó' | 'ó' | 'Ô' | 'ô' | 'Õ' | 'õ' | 'Ő' | 'ő' | 'Ø' | 'ø' | 'Œ' | 'œ'  => if znak.is_uppercase() { ascii_string.push('O')} else { ascii_string.push('o')},
                'Ŕ' | 'ŕ' | 'Ř' | 'ř' => if znak.is_uppercase() { ascii_string.push('R')} else { ascii_string.push('r')},
                'ẞ' | 'ß' | 'Ś' | 'ś' | 'Ŝ' | 'ŝ' | 'Ş' | 'ş' | 'Š' | 'š' | 'Ș' | 'ș' => if znak.is_uppercase() { ascii_string.push('S')} else { ascii_string.push('s')},
                'Ť' | 'ť' | 'Ţ' | 'ţ' | 'Þ' | 'þ' | 'Ț' | 'ț'  => if znak.is_uppercase() { ascii_string.push('T')} else { ascii_string.push('t')},
                'Ü' | 'ü' | 'Ù' | 'ù' | 'Ú' | 'ú' | 'Û' | 'û' | 'Ű' | 'ű' | 'Ũ' | 'ũ' | 'Ų' | 'ų' | 'Ů' | 'ů' | 'Ū' | 'ū' => if znak.is_uppercase() { ascii_string.push('U')} else { ascii_string.push('u')},
                'Ŵ' | 'ŵ' => if znak.is_uppercase() { ascii_string.push('W')} else { ascii_string.push('w')},
                'Ý' | 'ý' | 'Ÿ' | 'ÿ' | 'Ŷ' | 'ŷ' => if znak.is_uppercase() { ascii_string.push('Y')} else { ascii_string.push('y')},
                'Ź' | 'ź' | 'Ž' | 'ž' | 'Ż' | 'ż' => if znak.is_uppercase() { ascii_string.push('Z')} else { ascii_string.push('z')},
                _ => ()
            }
        }

        
    }

    if ascii_string.is_empty(){
        ascii_string = "Neznano".to_string();
    }

    return  ascii_string;

}

