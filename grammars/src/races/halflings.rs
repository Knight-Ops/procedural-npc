use rand::{thread_rng, seq::SliceRandom};
use serde_derive::{Deserialize, Serialize};

use super::RandomName;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum HalflingRace {
    LightfootHalfling,
    StoutHalfling,
}

impl Default for HalflingRace {
    fn default() -> HalflingRace {
        let choices = [
            HalflingRace::LightfootHalfling,
            HalflingRace::StoutHalfling,
        ];

        let mut rng = thread_rng();
        *choices.choose(&mut rng).expect("Error while choosing random race")
    }
}

pub struct Halfling {}

impl RandomName for Halfling {
    fn random_male_name() -> Result<String, &'static str> {
        let names = [
            "Alton", "Ander", "Bernie", "Bobbin", "Cade", "Callus", "Corrin", "Dannad", "Danniel", 
            "Eddie", "Egart", "Eldon", "Errich", "Fildo", "Finnan", "Franklin", "Garret", "Garth", 
            "Gilbert", "Gob", "Harol", "Igor", "Jasper", "Keith", "Kevin", "Lazam", "Lerry", "Lindal", 
            "Lyle", "Meric", "Mican", "Milo", "Morrin", "Nebin", "Nevil", "Osborn", "Ostran", "Oswalt", 
            "Perrin", "Poppy", "Reed", "Roscoe", "Sam", "Shardon", "Tye", "Ulmo", "Wellby", "Wendel", 
            "Wenner", "Wes"
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
            "Alain", "Andry", "Anne", "Bella", "Blossom", "Bree", "Callie", "Chenna", "Cora", 
            "Dee", "Dell", "Dida", "Eran", "Euphemia", "Georgina", "Gynnie", "Harriet", "Jasmine", 
            "Jillian", "Jo", "Kithri", "Lavinia", "Lidda", "Maegan", "Marigold", "Merla", "Myria", 
            "Nedda", "Nikki", "Nora", "Olivia", "Paela", "Pearl", "Pennie", "Philomena", "Portia", 
            "Robbie", "Rose", "Saral", "Seraphina", "Shaena", "Stacee", "Tawna", "Thea", "Trym", 
            "Tyna", "Vani", "Verna", "Wella", "Willow"
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
            "Appleblossom", "Bigheart", "Brightmoon", "Brushgather", "Cherrycheeks", "Copperkettle", 
            "Deephollow", "Elderberry", "Fastfoot", "Glenfellow", "Goldfound", "Goodbarrel", "Goodearth", 
            "Greenbottle", "Greenleaf", "High-hill", "Hilltopple", "Hogcollar", "Honeypot", "Jamjar", 
            "Kettlewhistle", "Leagallow", "Littlefoot", "Nimblefingers", "Porridgepot", "Quickstep", 
            "Reedfellow", "Shadowquick", "Silvereyes", "Smoothhands", "Stonebridge", "Stoutbridge", "Stoutman", 
            "Sunmeadow", "Swiftwhistle", "Tallfellow", "Tealeaf", "Tenpenny", "Thistletop", "Thorngage", 
            "Tosscobble", "Underbough", "Underfoot", "Warmwater", "Whispermouse", "Wildcloak", "Wildheart", "Wiseacre"
        ];

        let mut rng = thread_rng();
        if let Some(name) = names.choose(&mut rng) {
            Ok(name.to_string())
        } else {
            Err("Error retrieving last name")
        }
    }
}

