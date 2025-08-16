use serde::{ Deserialize, Serialize };
pub struct GumbEvent {
    pub kliknjen: bool,
}

pub struct PrejetoEvent {
    pub aktivno: bool,
    pub napaka: bool,
    pub sporocilo: String,
}

#[derive(Serialize, Deserialize)]
pub struct Kategorije {
    pub Kategorije: Vec<String>,
}

impl Default for PrejetoEvent {
    fn default() -> Self {
        PrejetoEvent {
            aktivno: false,
            napaka: false,
            sporocilo: "".to_string(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Format {
    pub ID: String,
    pub VideoFormat: String,
    pub Rezolucija: String,
    pub ExtFormat: String,
}

impl Default for Format {
    fn default() -> Self {
        Format {
            ID: "".to_string(),
            VideoFormat: "".to_string(),
            Rezolucija: "".to_string(),
            ExtFormat: "".to_string(),
        }
    }
}
