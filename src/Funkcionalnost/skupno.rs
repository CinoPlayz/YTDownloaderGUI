use crate::YTApp;
use std::path::Path;
use egui::RichText;
use egui::Id;

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

fn nalozi_sliko_napaka(ytapp: &mut YTApp, ctx: &egui::Context) {

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

pub fn IzpisiNapako(ytapp: &mut YTApp, ctx: &egui::Context, ID: u16, napaka: &str){

    nalozi_sliko_napaka(ytapp, ctx);

    //Ustvari nov egui window za napako in mu nastavi id
    egui::Window::new(RichText::new("Napaka").size(20.0))
    .id(Id::new(ID))
    .min_width(200.0)
    .min_height(100.0)
    .collapsible(false)
    .resizable(false)
    .open(&mut ytapp.PrikaziNapakoUI).show(ctx, |ui| {    
        //Dobi podatke iz TextureNapaka
        let texture: &egui::TextureHandle = &ytapp.TextureNapaka.as_mut().unwrap();

        //Postavi v grid zato da sta v eni vrstici
        egui::Grid::new("586013").show(ui, |ui| {
            ui.image(texture, texture.size_vec2());

            ui.label(napaka);
            ui.end_row();
        });
      

    });
}
