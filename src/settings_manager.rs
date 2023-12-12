use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dirs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    screen_shortcut: String,
    save_dir: String
}

impl Settings{

    pub  fn new()-> Self{
        Settings { screen_shortcut: "<Ctrl>a".to_string(), save_dir: dirs::download_dir().unwrap().to_str().unwrap_or_default().to_string()}
    }

    pub fn read_settings(filename:String) -> Option<Self> {
        let mut settings = Some(Settings::new());
        // Leggi il file di configurazione
        let dir_path = settings.clone().unwrap().get_directory();
        if dir_path.exists() && dir_path.is_dir() {
            let file_path = dir_path.join(filename);
            // La directory esiste, controlla se "config.json" esiste nella directory
            if file_path.exists() && file_path.is_file() {
                println!("Il file 'config.json' esiste nella directory specificata.");
                let parsed = fs::read_to_string(&file_path).expect("Impossibile leggere il file di configurazione");
                settings = serde_json::from_str(&parsed).expect("Impossibile deserializzare il file di configurazione");
            }
        } else {
            println!("La directory specificata non esiste.");
        }
        settings
    }

    pub fn get_directory(&self)-> PathBuf{  
        let home_dir = dirs::home_dir().expect("Impossibile trovare la cartella home dell'utente");
        let config_dir = home_dir.join(".MPSGU");
        // Controlla se la directory esiste
        if !config_dir.exists() {
            // Crea la cartella se non esiste
            fs::create_dir_all(&config_dir).expect("Impossibile creare la cartella di configurazione");
        }
        return config_dir;
    }

    pub fn write_settings(&self, filename:String){
        let dir_path = self.get_directory();
        // Serializza in una stringa JSON
        let serialized = serde_json::to_string_pretty(&self).expect("Errore nella serializzazione");
        // Percorso del file in cui salvare i dati
        if dir_path.exists() && dir_path.is_dir() {
            let file_path = dir_path.join(filename);
            let _ = fs::write(file_path, serialized);
            println!("Dati salvati");  
        }
    }

    pub fn get_screen_shortcut(&self) -> String {
        println!("Saved shortcut: {}", self.screen_shortcut);
        self.screen_shortcut.clone()
    }
    pub fn get_save_dir(&self) -> String {
        self.save_dir.clone()
    }
    
    pub fn set_screen_shortcut(&mut self, shortcut: String) {
        println!("New shortcut: {}, old shortcut: {}", shortcut, self.screen_shortcut);
        self.screen_shortcut = shortcut;
    }
    
    pub fn set_save_dir(&mut self, directory: String) {
        self.save_dir = directory;
    }
}