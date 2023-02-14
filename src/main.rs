#![allow(non_snake_case)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod DeliZaslona;
mod Funkcionalnost;
mod structs;

use eframe::egui;
use app::YTApp;
use eframe::egui::{Style, Visuals};

fn main() {
    //Dobi themo računalnika
    let mode = dark_light::detect();
    let thema;

    match mode {
        dark_light::Mode::Dark => {thema = Visuals::dark()},
        dark_light::Mode::Light => {thema = Visuals::light()},
        dark_light::Mode::Default => {thema = Visuals::dark()},
    }
    
    tracing_subscriber::fmt::init();

    //Window options
    let options = eframe::NativeOptions {
        icon_data: Some(load_icon("assets/icon/icon-red.png")), 
        initial_window_size: Some(egui::Vec2::new(500.0, 620.0)),
        min_window_size: Some(egui::Vec2::new(500.0, 620.0)),
        ..Default::default()
    };
    
    let _native_options = eframe::NativeOptions::default();
    
    match eframe::run_native(
        "YT Downloader",
        options,
        Box::new(|creation_context| {
            let style = Style {
                //Spremeni themo gleda themo računalnika
                visuals: thema,
                ..Style::default()
            };
            creation_context.egui_ctx.set_style(style);
            Box::new(YTApp::new(creation_context))
        }),
    ){
        Ok(_) => {},
        Err(napaka) => println!("Napaka pri zagonu: {}", napaka),
    }

    fn load_icon(path: &str) -> eframe::IconData {
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open(path)
            .expect("Ne morem odpreti icone")
            .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
    
    
        };
    
        eframe::IconData {
            rgba: icon_rgba,
            width: icon_width,
            height: icon_height,
        }
    }

}

