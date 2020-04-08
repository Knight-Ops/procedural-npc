use rand::{thread_rng, seq::SliceRandom};
use serde_derive::{Deserialize, Serialize};

use super::RandomName;

use std::fmt::Display;
use std::fmt;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum HumanRace {
    Human
}

impl Default for HumanRace {
    fn default() -> HumanRace {
        let choices = [
            HumanRace::Human
        ];

        let mut rng = thread_rng();
        *choices.choose(&mut rng).expect("Error while choosing random race")
    }
}

impl Display for HumanRace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HumanRace::Human => write!(f, "Human"),
        }   
    }
}

pub struct Human {}

impl RandomName for Human {
    fn random_male_name() -> Result<String, &'static str> {
        let names = [
            "Adam", "Adelard", "Airell", "Airic", "Alan", "Aldous", "Anghus", "Anselm", "Aodh", "Arnold", 
            "Bardon", "Bearacb", "Bernard", "Bertram", "Bevyn", "Boden", "Bran", "Brasil", "Bredon", "Brian", 
            "Bricriu", "Bryant", "Cadman ", "Caradoc", "Cedric", "Charles", "Clerebold", "Conalt", "Conchobar", 
            "Condon", "Conrad", "Darcy", "Devin", "Diggory", "Dillion", "Donaghy", "Donall", "Drogo", "Duer", 
            "Eghan", "Everard", "Ewyn", "Ferghus", "Frederick", "Galvyn", "Geoffrey", "Gerald", "Gilbert", 
            "Gildas", "Godfrey ", "Gunter", "Guy", "Harvey", "Henry", "Heward", "Hubert", "Hugh", "Iden", 
            "Irven", "Jocelyn", "John", "Karney", "Kayne", "Kelvyn", "Kunsgnos", "Lance", "Leigh", "Maccus", 
            "Manfred", "Miles", "Moryn", "Neale", "Nicholas", "Norman", "Odo", "Owyn", "Percival", "Peter", 
            "Pryderi", "Ralf", "Randal", "Raymond", "Reaghan", "Reynard", "Richard", "Robert", "Roger", 
            "Roland", "Rolf", "Simon", "Taliesin", "Theobald", "Theodoric", "Thomas", "Tiernay", "Timm", 
            "Turi ", "William", "Wymar"
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
            "Adelaide", "Agatha", "Agnes", "Aife", "Aina", "Alane", "Alice", "Aline", "Anne", "Ardena", 
            "Arienh", "Avelina", "Avice", "Beatha", "Beatrice", "Birgit", "Briann", "Caomh", "Cara", "Cecily", 
            "Cinnia", "Cordelia", "Deheune", "Divone", "Donia", "Doreena", "Egelina", "Eleanor", "Elizabeth", 
            "Ella", "Eloise", "Elsha", "Elysande", "Emeny ", "Emma", "Emmeline", "Enid", "Ermina", "Ethne", 
            "Eva", "Evelina", "Fianna", "Galiena", "Genevieve", "Geva", "Gilda", "Giselle", "Gitta", "Grania", 
            "Griselda", "Gwyndolin", "Hadwisa", "Helen", "Herleva", "Hugolina", "Ida", "Idelisa", "Isabella", 
            "Isolde", "Jacoba", "Jane", "Joan", "Juliana", "Katherine", "Keelin", "Kennocha", "Lavena", 
            "Lesley", "Linnette", "Lyonesse ", "Mabina", "Margery", "Marvina", "Mary", "Matilda", "Mavis", 
            "Maynild", "Millicent", "Mirna", "Morgan", "Muriel", "Nareena", "Oriana", "Oriel", "Regan", 
            "Rohesia", "Ronat", "Rosalind", "Rosamund", "Rowena", "Sarah", "Selma", "Susannah", "Sybil", 
            "Ula", "Venetia", "Williamina", "Wynne", "Yseult", "Yvonne"
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
            ""
        ];

        let mut rng = thread_rng();
        if let Some(name) = names.choose(&mut rng) {
            Ok(name.to_string())
        } else {
            Err("Error retrieving last name")
        }
    }
}

