use rand::{thread_rng, seq::SliceRandom};
use serde_derive::{Deserialize, Serialize};

use crate::Gender;

use super::RandomName;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum DwarvernRace {
    HillDwarf,
    MountainDwarf
}

impl Default for DwarvernRace {
    fn default() -> DwarvernRace {
        let choices = [
            DwarvernRace::HillDwarf,
            DwarvernRace::MountainDwarf
        ];

        let mut rng = thread_rng();
        *choices.choose(&mut rng).expect("Error while choosing random race")
    }
}

pub struct Dwarf {}

impl RandomName for Dwarf {
    fn random_male_name() -> Result<String, &'static str> {
        let names = [
            "Adrik", "Alberich", "Baern", "Barendd", "Beloril", "Brottor", "Dain", "Dalgal", "Darrak", 
            "Delg", "Duergath", "Dworic", "Eberk", "Einkil", "Elaim", "Erias", "Fallond", "Fargrim", 
            "Gardain", "Gilthur", "Gimgen", "Gimurt", "Harbek", "Kildrak", "Kilvar", "Morgran", "Morkral", 
            "Nalral", "Nordak", "Nuraval", "Oloric", "Olunt", "Orsik", "Oskar", "Rangrim", "Reirak", 
            "Rurik", "Taklinn", "Thoradin", "Thorin", "Thradal", "Tordek", "Traubon", "Travok", "Ulfgar", 
            "Uraim", "Veit", "Vonbin", "Vondal", "Whurbin"
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
            "Anbera", "Artin", "Audhild", "Balifra", "Barbena", "Bardryn", "Bolhild", "Dagnal", 
            "Dariff", "Delre", "Diesa", "Eldeth", "Eridred", "Falkrunn", "Fallthra", "Finellen", 
            "Gillydd", "Gunnloda", "Gurdis", "Helgret", "Helja", "Hlin", "Ilde", "Jarana", "Kathra", 
            "Kilia", "Kristryd", "Liftrasa", "Marastyr", "Mardred", "Morana", "Nalaed", "Nora", 
            "Nurkara", "Oriff", "Ovina", "Riswynn", "Sanni", "Therlin", "Thodris", "Torbera", "Tordrid", 
            "Torgga", "Urshar", "Valida", "Vistra", "Vonana", "Werydd", "Whurdred", "Yurgunn"
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
            "Aranore", "Balderk", "Battlehammer", "Bigtoe", "Bloodkith", "Bofdann", "Brawnanvil", "Brazzik", 
            "Broodfist", "Burrowfound", "Caebrek", "Daerdahk", "Dankil", "Daraln", "Deepdelver", "Durthane", 
            "Eversharp", "Fallack", "Fireforge", "Foamtankard", "Frostbeard", "Glanhig", "Goblinbane", "Goldfinder", 
            "Gorunn", "Graybeard", "Hammerstone", "Helcral", "Holderhek", "Ironfist", "Loderr", "Lutgehr", 
            "Morigak", "Orcfoe", "Rakankrak", "Ruby-Eye", "Rumnaheim", "Silveraxe", "Silverstone", "Steelfist", 
            "Stoutale", "Strakeln", "Strongheart", "Thrahak", "Torevir", "Torunn", "Trollbleeder", "Trueanvil", 
            "Trueblood", "Ungart" 
        ];

        let mut rng = thread_rng();
        if let Some(name) = names.choose(&mut rng) {
            Ok(name.to_string())
        } else {
            Err("Error retrieving last name")
        }
    }
}

