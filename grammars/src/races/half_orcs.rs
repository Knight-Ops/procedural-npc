use rand::{thread_rng, seq::SliceRandom};
use serde_derive::{Deserialize, Serialize};

use super::RandomName;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum HalfOrcRace {
    HalfOrc
}

impl Default for HalfOrcRace {
    fn default() -> HalfOrcRace {
        let choices = [
            HalfOrcRace::HalfOrc
        ];

        let mut rng = thread_rng();
        *choices.choose(&mut rng).expect("Error while choosing random race")
    }
}

pub struct HalfOrc {}

impl RandomName for HalfOrc {
    fn random_male_name() -> Result<String, &'static str> {
        let names = [
            "Argran", "Braak", "Brug", "Cagak", "Dench", "Dorn", "Dren", "Druuk", "Feng", "Gell", 
            "Gnarsh", "Grumbar", "Gubrash", "Hagren", "Henk", "Hogar", "Holg", "Imsh", "Karash", "Karg", 
            "Keth", "Korag", "Krusk", "Lubash", "Megged", "Mhurren", "Mord", "Morg", "Nil", "Nybarg", 
            "Odorr", "Ohr", "Rendar", "Resh", "Ront", "Rrath", "Sark", "Scrag", "Sheggen", "Shump", 
            "Tanglar", "Tarak", "Thar", "Thokk", "Trag", "Ugarth", "Varg", "Vilberg", "Yurk", "Zed"
        ];

        let mut rng = thread_rng();
        if let Some(name) = names.choose(&mut rng) {
            Ok(format!("{}", name))
        } else {
            Err("Error retrieving male name")
        }
    }

    fn random_female_name() -> Result<String, &'static str> {
        let names = [
            "Arha", "Baggi", "Bendoo", "Bilga", "Brakka", "Creega", "Drenna", "Ekk", "Emen", "Engong", 
            "Fistula", "Gaaki", "Gorga", "Grai", "Greeba", "Grigi", "Gynk", "Hrathy", "Huru", "Ilga", 
            "Kabbarg", "Kansif", "Lagazi", "Lezre", "Murgen", "Murook", "Myev", "Nagrette", "Neega", 
            "Nella", "Nogu", "Oolah", "Ootah", "Ovak", "Ownka", "Puyet", "Reeza", "Shautha", "Silgre", 
            "Sutha", "Tagga", "Tawar", "Tomph", "Ubada", "Vanchu", "Vola", "Volen", "Vorka", "Yevelda", 
            "Zagga"
        ];

        let mut rng = thread_rng();
        if let Some(name) = names.choose(&mut rng) {
            Ok(format!("{}", name))
        } else {
            Err("Error retrieving female name")
        }
    }

    fn random_last_name() -> Result<String, &'static str> {
        unimplemented!()
    }
}

