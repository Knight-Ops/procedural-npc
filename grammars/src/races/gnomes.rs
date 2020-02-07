use rand::{thread_rng, seq::SliceRandom};
use serde_derive::{Deserialize, Serialize};

use super::RandomName;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum GnomishRace {
    ForestGnome,
    RockGnome 
}

impl Default for GnomishRace {
    fn default() -> GnomishRace {
        let choices = [
            GnomishRace::ForestGnome,
            GnomishRace::RockGnome
        ];

        let mut rng = thread_rng();
        *choices.choose(&mut rng).expect("Error while choosing random race")
    }
}

pub struct Gnome {}

impl RandomName for Gnome {
    fn random_male_name() -> Result<String, &'static str> {
        let names = [
            "Alston", "Alvyn", "Anverth", "Arumawann", "Bilbron", "Boddynock", "Brocc", "Burgell", 
            "Cockaby", "Crampernap", "Dabbledob", "Delebean", "Dimble", "Eberdeb", "Eldono", "Erky", 
            "Fablen", "Fibblestib", "Fonkin", "Frouse", "Frug", "Gerbo", "Gimble", "Glim", "Igden", 
            "Jabble", "Jebeddo", "Kellen", "Kipper", "Namfoodle", "Oppleby", "Orryn", "Paggen", 
            "Pallabar", "Pog", "Qualen", "Ribbles", "Rimple", "Roondar", "Sapply", "Seebo", "Senteq", 
            "Sindri", "Umpen", "Warryn", "Wiggens", "Wobbles", "Wrenn", "Zaffrab", "Zook"
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
            "Abalaba", "Bimpnottin", "Breena", "Buvvie", "Callybon", "Caramip", "Carlin", "Cumpen", 
            "Dalaba", "Donella", "Duvamil", "Ella", "Ellyjoybel", "Ellywick", "Enidda", "Lilli", 
            "Loopmottin", "Lorilla", "Luthra", "Mardnab", "Meena", "Meeny", "Mumpena", "Nissa", 
            "Numba", "Nyx", "Oda", "Oppah", "Orla", "Panana", "Pyntle", "Quilla", "Ranala", 
            "Reddlepop", "Roywyn", "Salanop", "Shamil", "Siffress", "Symma", "Tana", "Tenena", 
            "Tervaround", "Tippletoe", "Ulla", "Unvera", "Veloptima", "Virra", "Waywocket", "Yebe", 
            "Zanna"
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
            "Albaratie", "Bafflestone", "Beren", "Boondiggles", "Cobblelob", "Daergel", "Dunben", "Fabblestabble", 
            "Fapplestamp", "Fiddlefen", "Folkor", "Garrick", "Gimlen", "Glittergem", "Gobblefirn", "Gummen", 
            "Horcusporcus", "HumpleBumple", "Ironhide", "Leffery", "Lingenhall", "Loofollue", "Maekkelferce", 
            "Miggledy", "Munggen", "Murnig", "Musgraben", "Nackle", "Ningel", "Nopenstallen", "Nuckelstamp", 
            "Offund", "Oomtrowl", "Pilwicken", "Pingun", "Quillsharpener", "Raulnor", "Reese", "Rofferton", 
            "Scheppen", "Shadowcloak", "Silverthread", "Sympony", "Tarkelby", "Timbers", "Turen", "Umbodoben", 
            "Waggletop", "Welber", "Wildwander"
        ];

        let mut rng = thread_rng();
        if let Some(name) = names.choose(&mut rng) {
            Ok(name.to_string())
        } else {
            Err("Error retrieving last name")
        }
    }
}

