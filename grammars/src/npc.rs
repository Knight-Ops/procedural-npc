use rand::{thread_rng, seq::SliceRandom};
use serde_derive::{Deserialize, Serialize};

use super::races;
use super::races::RandomName;

use crate::stats::StatBlock;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NPC {
    gender: Gender,
    race: Race,
    name: String,
    profession: Profession,
    stats: StatBlock,
    bond: Option<NPCNoBond>,
}

impl Default for NPC {
    fn default() -> NPC {
        let gender = Gender::default();
        let race = Race::default();
        let name = races::get_appropriate_name(gender, race);
        let profession = Profession::default();
        let stats = StatBlock::random();
        let bond = NPC::random_bond();
        NPC {
            gender,
            race,
            name,
            profession,
            stats,
            bond,
        }
    }
}

impl NPC {
    pub fn new(gender: Option<Gender>) -> NPC {
        if gender.is_some() {
            NPC {
                gender: gender.unwrap(),
                ..Default::default()
            }
        } else {
            NPC::default()
        }
    }


    fn random_bond() -> Option<NPCNoBond> {
        let choices = [
            Some(NPCNoBond::default()),
            None
        ];

        let mut rng = thread_rng();
        if let Some(bond) = choices.choose(&mut rng).expect("Error while choosing random gender") {
            Some(bond.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NPCNoBond {
    gender: Gender,
    race: Race,
    name: String,
    profession: Profession,
    stats: StatBlock,
}

impl Default for NPCNoBond {
    fn default() -> NPCNoBond {
        let gender = Gender::default();
        let race = Race::default();
        let name = races::get_appropriate_name(gender, race);
        let profession = Profession::default();
        let stats = StatBlock::random();
        NPCNoBond {
            gender,
            race,
            name,
            profession,
            stats,
        }
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
enum Profession {
    TavernWench,
    Shopkeeper,
    Blacksmith,
    Healer,
    Constable,
    Apocathary,
    Barkeep,
}

impl Default for Profession {
    fn default() -> Profession {
        let choices = [
            Profession::TavernWench,
            Profession::Shopkeeper,
            Profession::Blacksmith,
            Profession::Healer,
            Profession::Constable,
            Profession::Apocathary,
            Profession::Barkeep,
        ];

        let mut rng = thread_rng();
        *choices.choose(&mut rng).expect("Error while choosing random gender")
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum Gender {
    Male,
    Female,
}

impl Default for Gender {
    fn default() -> Gender {
        let choices = [Gender::Male, Gender::Female];

        let mut rng = thread_rng();
        *choices.choose(&mut rng).expect("Error while choosing random gender")
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum Race {
    Dragonborn(races::dragonborn::DragonbornRace),
    Dwarf(races::dwarves::DwarvernRace),
    Elf(races::elves::ElvenRace),
    Gnome(races::gnomes::GnomishRace),
    HalfOrc(races::half_orcs::HalfOrcRace),
    Halfling(races::halflings::HalflingRace),
    Human(races::humans::HumanRace),
    Tiefling(races::tieflings::TieflingRace),
}

impl Default for Race {
    fn default() -> Race {
        let choices = [
            Race::Dragonborn(races::dragonborn::DragonbornRace::default()),
            Race::Dwarf(races::dwarves::DwarvernRace::default()),
            Race::Elf(races::elves::ElvenRace::default()),
            Race::Gnome(races::gnomes::GnomishRace::default()),
            Race::HalfOrc(races::half_orcs::HalfOrcRace::default()),
            Race::Halfling(races::halflings::HalflingRace::default()),
            Race::Human(races::humans::HumanRace::default()),
            Race::Tiefling(races::tieflings::TieflingRace::default())
        ];

        let mut rng = thread_rng();
        *choices.choose(&mut rng).expect("Error while choosing random gender")
    }
}
