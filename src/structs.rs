pub struct GumbEvent{
    pub kliknjen: bool,
}

pub struct PrejetoEvent{
    pub aktivno: bool,
    pub napaka: bool,
}

impl Default for PrejetoEvent{
    fn default() -> Self {
        PrejetoEvent { 
            aktivno: false, 
            napaka: false 
        }
    }
}

#[derive(PartialEq, Clone)] 
pub struct Format{
    pub ID: String,
    pub VideoFormat: String,    
    pub Rezolucija: String,
}

impl Default for Format{
    fn default() -> Self {
        Format { 
            ID: "".to_string(), 
            VideoFormat: "".to_string(), 
            Rezolucija: "".to_string()
        }
    }
}
