use rand::{thread_rng, seq::SliceRandom};
use serde_derive::{Deserialize, Serialize};

use super::RandomName;

use std::fmt::Display;
use std::fmt;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum DragonbornRace {
    BlackDragonborn,
    BlueDragonborn,
    BrassDragonborn,
    BronzeDragonborn,
    CopperDragonborn,
    GoldDragonborn,
    GreenDragonborn,
    RedDragonborn,
    SilverDragonborn,
    WhiteDragonborn
}

impl Default for DragonbornRace {
    fn default() -> DragonbornRace {
        let choices = [DragonbornRace::BlackDragonborn,
            DragonbornRace::BlueDragonborn,
            DragonbornRace::BrassDragonborn,
            DragonbornRace::BronzeDragonborn,
            DragonbornRace::CopperDragonborn,
            DragonbornRace::GoldDragonborn,
            DragonbornRace::GreenDragonborn,
            DragonbornRace::RedDragonborn,
            DragonbornRace::SilverDragonborn,
            DragonbornRace::WhiteDragonborn];

        let mut rng = thread_rng();
        *choices.choose(&mut rng).expect("Error while choosing random race")
    }
}

impl Display for DragonbornRace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DragonbornRace::BlackDragonborn => write!(f, "Black Dragonborn"),
            DragonbornRace::BlueDragonborn => write!(f, "Blue Dragonborn"),
            DragonbornRace::BrassDragonborn => write!(f, "Brass Dragonborn"),
            DragonbornRace::BronzeDragonborn => write!(f, "Bronze Dragonborn"),
            DragonbornRace::CopperDragonborn => write!(f, "Copper Dragonborn"),
            DragonbornRace::GoldDragonborn => write!(f, "Gold Dragonborn"),
            DragonbornRace::GreenDragonborn => write!(f, "Green Dragonborn"),
            DragonbornRace::RedDragonborn => write!(f, "Red Dragonborn"),
            DragonbornRace::SilverDragonborn => write!(f, "Silver Dragonborn"),
            DragonbornRace::WhiteDragonborn => write!(f, "White Dragonborn")
        }   
    }
}

pub struct Dragonborn {}

impl RandomName for Dragonborn {
    fn random_male_name() -> Result<String, &'static str> {
        let names = ["Adrex", "Arjhan", "Azzakh", "Balasar", "Baradad", "Bharash", "Bidreked", "Dadalan", 
            "Dazzazn", "Direcris", "Donaar", "Fax", "Gargax", "Ghesh", "Gorbundus", "Greethen", "Heskan", 
            "Hirrathak", "Ildrex", "Kaladan", "Kerkad", "Kiirith", "Kriv", "Maagog", "Medrash", "Mehen", 
            "Mozikth", "Mreksh", "Mugrunden", "Nadarr", "Nithther", "Norkruuth", "Nykkan", "Pandjed", "Patrin", 
            "Pijjirik", "Quarethon", "Rathkran", "Rhogar", "Rivaan", "Sethrekar", "Shamash", "Shedinn", 
            "Srorthen", "Tarhun", "Torinn", "Trynnicus", "Valorean", "Vrondiss", "Zedaar"];

        let last_name = Self::random_last_name().expect("Error while retrieving last name");

        let mut rng = thread_rng();
        if let Some(name) = names.choose(&mut rng) {
            Ok(format!("{} {}", name, last_name))
        } else {
            Err("Error retrieving male name")
        }
    }

    fn random_female_name() -> Result<String, &'static str> {
        let names = ["Akra", "Aasathra", "Antrara", "Arava", "Biri", "Blendaeth", "Burana", "Chassath", 
            "Daar", "Dentratha", "Doudra", "Driindar", "Eggren", "Farideh", "Findex", "Furrele", 
            "Gesrethe", "Gilkaas", "Harann", "Havilar", "Hethress", "Hillanot", "Jaxi", "Jezean", 
            "Jheri", "Kadana", "Kava", "Korinn", "Megren", "Mijira", "Mishann", "Nala", "Nuthra", 
            "Perra", "Pogranix", "Pyxrin", "Quespa", "Raiann", "Rezena", "Ruloth", "Saphara", 
            "Savaran", "Sora", "Surina", "Synthrin", "Tatyan", "Thava", "Uadjit", "Vezera", "Zykroff"];

        let last_name = Self::random_last_name().expect("Error while retrieving last name");

        let mut rng = thread_rng();
        if let Some(name) = names.choose(&mut rng) {
            Ok(format!("{} {}", name, last_name))
        } else {
            Err("Error retrieving female name")
        }
    }

    fn random_last_name() -> Result<String, &'static str> {
        let names = ["Akambherylliax", "Argenthrixus", "Baharoosh", "Beryntolthropal", "Bhenkumbyrznaax", "Caavylteradyn", 
            "Chumbyxirinnish", "Clethtinthiallor", "Daardendrian", "Delmirev", "Dhyrktelonis", "Ebynichtomonis", 
            "Esstyrlynn", "Fharngnarthnost", "Ghaallixirn", "Grrrmmballhyst", "Gygazzylyshrift", "Hashphronyxadyn", 
            "Hshhsstoroth", "Imbixtellrhyst", "Jerynomonis", "Jharthraxyn", "Kerrhylon", "Kimbatuul", "Lhamboldennish", 
            "Linxakasendalor", "Mohradyllion", "Mystan", "Nemmonis", "Norixius", "Ophinshtalajiir", "Orexijandilin", 
            "Pfaphnyrennish", "Phrahdrandon", "Pyratallinost", "Qyxpahrgh", "Raghthroknaar", "Shestendeliath", 
            "Skaarzborroosh", "Sumnarghthrysh", "Tiammanthyllish", "Turnuroth", "Umbyrphrael", "Vangdondalor", 
            "Verthisathurgiesh", "Wivvyrholdalphiax", "Wystongjiir", "Xephyrbahnor", "Yarjerit", "Zzzxaaxthroth"];

        let mut rng = thread_rng();
        if let Some(name) = names.choose(&mut rng) {
            Ok(name.to_string())
        } else {
            Err("Error retrieving last name")
        }
    }
}

