use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    screen_shortcut: String,
    save_dir: String,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            screen_shortcut: "<Ctrl>a".to_string(),
            save_dir: dirs::download_dir()
                .unwrap()
                .to_str()
                .unwrap_or_default()
                .to_string(),
        }
    }

    pub fn read_settings(filename: String) -> Option<Self> {
        let mut settings = Some(Settings::new());
        
        let dir_path = settings.clone().unwrap().get_directory();
        if dir_path.exists() && dir_path.is_dir() {
            let file_path = dir_path.join(filename);
            
            if file_path.exists() && file_path.is_file() {
                let parsed = fs::read_to_string(&file_path)
                    .expect("Impossibile leggere il file di configurazione");
                settings = serde_json::from_str(&parsed)
                    .expect("Impossibile deserializzare il file di configurazione");
            }
        }
        settings
    }

    pub fn get_directory(&self) -> PathBuf {
        let home_dir = dirs::home_dir().expect("Impossibile trovare la cartella home dell'utente");
        let config_dir = home_dir.join(".MPSGU");
        
        if !config_dir.exists() {
            
            fs::create_dir_all(&config_dir)
                .expect("Impossibile creare la cartella di configurazione");
        }
        return config_dir;
    }

    pub fn write_settings(&self, filename: String) {
        let dir_path = self.get_directory();
        
        let serialized = serde_json::to_string_pretty(&self).expect("Errore nella serializzazione");
        
        if dir_path.exists() && dir_path.is_dir() {
            let file_path = dir_path.join(filename);
            let _ = fs::write(file_path, serialized);
        }
    }

    pub fn get_screen_shortcut(&self) -> String {
        self.screen_shortcut.clone()
    }
    pub fn get_save_dir(&self) -> String {
        self.save_dir.clone()
    }

    pub fn set_screen_shortcut(&mut self, shortcut: String) {
        self.screen_shortcut = shortcut;
    }

    pub fn set_save_dir(&mut self, directory: String) {
        self.save_dir = directory;
    }
}
