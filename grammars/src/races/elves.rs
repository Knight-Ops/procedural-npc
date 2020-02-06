use rand::{thread_rng, seq::SliceRandom};
use serde_derive::{Deserialize, Serialize};

use crate::Gender;

use super::RandomName;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ElvenRace {
    SunElfHighElf,
    MoonElfHighElf,
    WoodElf,
    DarkElfDrow,
    
}

impl Default for ElvenRace {
    fn default() -> ElvenRace {
        let choices = [
            ElvenRace::SunElfHighElf,
            ElvenRace::MoonElfHighElf,
            ElvenRace::WoodElf,
            ElvenRace::DarkElfDrow,
        ];

        let mut rng = thread_rng();
        *choices.choose(&mut rng).expect("Error while choosing random race")
    }
}

pub struct Elf {}

impl RandomName for Elf {
    fn random_male_name() -> Result<String, &'static str> {
        let names = [
            "Adran", "Aelar", "Aerdeth", "Ahvain", "Aramil", "Arannis", "Aust", "Azaki", "Beiro", 
            "Berrian", "Caeldrim", "Carric", "Dayereth", "Dreali", "Efferil", "Eiravel", "Enialis", 
            "Erdan", "Erevan", "Fivin", "Galinndan", "Gennal", "Hadarai", "Halimath", "Heian", "Himo", 
            "Immeral", "Ivellios", "Korfel", "Lamlis", "Laucian", "Lucan", "Mindartis", "Naal", "Nutae", 
            "Paelias", "Peren", "Quarion", "Riardon", "Rolen", "Soveliss", "Suhnae", "Thamior", "Tharivol", 
            "Theren", "Theriatis", "Thervan", "Uthemar", "Vanuath", "Varis"
        ];

        let last_name = Self::random_last_name().expect("Error while retrieving last name");

        let mut rng = thread_rng();
        if let Some(name) = names.choose(&mut rng) {
            Ok(format!("{} {}", name, last_name))
        } else {
            Err("Error retrieving male name")
        }
    }

    fn random_female_name() -> Result<String, &'static str> {
        let names = [
            "Adrie", "Ahinar", "Althaea", "Anastrianna", "Andraste", "Antinua", "Arara", 
            "Baelitae", "Bethrynna", "Birel", "Caelynn", "Chaedi", "Claira", "Dara", "Drusilia", 
            "Elama", "Enna", "Faral", "Felosial", "Hatae", "Ielenia", "Ilanis", "Irann", "Jarsali", 
            "Jelenneth", "Keyleth", "Leshanna", "Lia", "Maiathah", "Malquis", "Meriele", "Mialee", 
            "Myathethil", "Naivara", "Quelenna", "Quillathe", "Ridaro", "Sariel", "Shanairla", "Shava", 
            "Silaqui", "Sumnes", "Theirastra", "Thiala", "Tiaathque", "Traulam", "Vadania", "Valanthe", 
            "Valna", "Xanaphia"
        ];

        let last_name = Self::random_last_name().expect("Error while retrieving last name");

        let mut rng = thread_rng();
        if let Some(name) = names.choose(&mut rng) {
            Ok(format!("{} {}", name, last_name))
        } else {
            Err("Error retrieving female name")
        }
    }

    fn random_last_name() -> Result<String, &'static str> {
        let names = [
            "Aloro", "Amakiir", "Amastacia", "Ariessus", "Arnuanna", "Berevan", "Caerdonel", "Caphaxath", 
            "Casilltenirra", "Cithreth", "Dalanthan", "Eathalena", "Erenaeth", "Ethanasath", "Fasharash", 
            "Firahel", "Floshem", "Galanodel", "Goltorah", "Hanali", "Holimion", "Horineth", "Iathrana", 
            "Ilphelkiir", "Iranapha", "Koehlanna", "Lathalas", "Liadon", "Meliamne", "Mellerelel", "Mystralath", 
            "Naïlo", "Netyoive", "Ofandrus", "Ostoroth", "Othronus", "Qualanthri", "Raethran", "Rothenel", 
            "Selevarun", "Siannodel", "Suithrasas", "Sylvaranth", "Teinithra", "Tiltathana", "Wasanthi", 
            "Withrethin", "Xiloscient", "Xistsrith", "Yaeldrin"
        ];

        let mut rng = thread_rng();
        if let Some(name) = names.choose(&mut rng) {
            Ok(name.to_string())
        } else {
            Err("Error retrieving last name")
        }
    }
}

