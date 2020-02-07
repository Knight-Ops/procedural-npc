use rand::{thread_rng, seq::SliceRandom, Rng};
use serde_derive::{Deserialize, Serialize};

use super::RandomName;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum TieflingRace {
    Tiefling
}

impl Default for TieflingRace {
    fn default() -> TieflingRace {
        let choices = [
            TieflingRace::Tiefling
        ];

        let mut rng = thread_rng();
        *choices.choose(&mut rng).expect("Error while choosing random race")
    }
}

pub struct Tiefling {}

impl RandomName for Tiefling {
    fn random_male_name() -> Result<String, &'static str> {
        let names = [
            "Abad", "Ahrim", "Akmen", "Amnon", "Andram", "Astar", "Balam", "Barakas", "Bathin", "Caim", 
            "Chem", "Cimer", "Cressel", "Damakos", "Ekemon", "Euron", "Fenriz", "Forcas", "Habor", "Iados", 
            "Kairon", "Leucis", "Mamnen", "Mantus", "Marbas", "Melech", "Merihim", "Modean", "Mordai", 
            "Mormo", "Morthos", "Nicor", "Nirgel", "Oriax", "Paymon", "Pelaios", "Purson", "Qemuel", "Raam", 
            "Rimmon", "Sammal", "Skamoso", "Tethren", "Thamuz", "Therai", "Valafar", "Vassago", "Xappan", 
            "Zepar", "Zephan"
        ];

        let virtue_name = Self::random_last_name().expect("Error getting tiefling virtue name");

        let mut rng = thread_rng();
        if let Some(name) = names.choose(&mut rng) {
            if rng.gen_range(0.0, 1.0) > 0.5 {
                Ok(format!("{}", name))
            } else {
                Ok(format!("{}", virtue_name))
            }
        } else {
            Err("Error retrieving male name")
        }
    }

    fn random_female_name() -> Result<String, &'static str> {
        let names = [
            "Akta", "Anakis", "Armara", "Astaro", "Aym", "Azza", "Beleth", "Bryseis", "Bune", 
            "Criella", "Damaia", "Decarabia", "Ea", "Gadreel", "Gomory", "Hecat", "Ishte", "Jezebeth", 
            "Kali", "Kallista", "Kasdeya", "Lerissa", "Lilith", "Makaria", "Manea", "Markosian", 
            "Masterma", "Naamah", "Nemeia", "Nija", "Orianna", "Osah", "Phelaia", "Prosperine", 
            "Purah", "Pyra", "Rieta", "Ronobe", "Ronwe", "Seddit", "Seere", "Sekhmet", "Semyaza", 
            "Shava", "Shax", "Sorath", "Uzza", "Vapula", "Vepar", "Verin"
        ];

        let virtue_name = Self::random_last_name().expect("Error getting tiefling virtue name");

        let mut rng = thread_rng();
        if let Some(name) = names.choose(&mut rng) {
            if rng.gen_range(0.0, 1.0) > 0.5 {
                Ok(format!("{}", name))
            } else {
                Ok(format!("{}", virtue_name))
            }
        } else {
            Err("Error retrieving female name")
        }
    }

    fn random_last_name() -> Result<String, &'static str> {
        let names = [
            "Ambition", "Art", "Carrion", "Chant", "Creed", "Death", "Debauchery", "Despair", "Doom", "Doubt", 
            "Dread", "Ecstasy", "Ennui", "Entropy", "Excellence", "Fear", "Glory", "Gluttony", "Grief", "Hate", 
            "Hope", "Horror", "Ideal", "Ignominy", "Laughter", "Love", "Lust", "Mayhem", "Mockery", "Murder", 
            "Muse", "Music", "Mystery", "Nowhere", "Open", "Pain", "Passion", "Poetry", "Quest", "Random", 
            "Reverence", "Revulsion", "Sorrow", "Temerity", "Torment", "Tragedy", "Vice", "Virtue", "Weary", "Wit"
        ];

        let mut rng = thread_rng();
        if let Some(name) = names.choose(&mut rng) {
            Ok(format!("{}", name))
        } else {
            Err("Error retrieving female name")
        }
    }
}

