extern crate copypasta;
use configparser::ini::Ini;
use copypasta::{ClipboardContext, ClipboardProvider};
use std::fs;
use std::path;
fn main() {
    let mut config = Ini::new();
    let mut clipboard = ClipboardContext::new().unwrap();
    let mut last_clip = clipboard.get_contents().unwrap();
    if !path::Path::new("clip_history").exists() {
        config.set("Clipboard", "history", Some(String::from("Hi There 👋")));
        config.set("Clipboard", "cpStatus", Some(String::from("0")));
        config.set("Clipboard", "cpData", Some(String::from("")));
        config.write("clip_history");
        println!("Clipboard made");
    }
    config.load("clip_history");

    loop {
        if last_clip != clipboard.get_contents().unwrap() {
            let new_clip = config.get("Clipboard", "history").unwrap()
                + " (MADE_BY_CELIBISTRIAL) "
                + &clipboard.get_contents().unwrap();
            config.set("Clipboard", "history", Some(new_clip));
            last_clip = clipboard.get_contents().unwrap();
            config.write("clip_history");
        }
        if config
            .getboolcoerce("Clipboard", "cpStatus")
            .unwrap()
            .unwrap()
        {
            config.set("Clipboard", "cpStatus", Some("0".to_string()));
            clipboard.set_contents(config.get("Clipboard", "cpData").unwrap());
        }
    }
}
