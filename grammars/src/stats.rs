use serde_derive::{Deserialize, Serialize};
use rand::thread_rng;
use rand::Rng;

#[derive(Debug, Default, Copy, Clone, Deserialize, Serialize)]
pub struct StatBlock {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8
}

impl StatBlock {

    pub fn random() -> StatBlock {
        StatBlock {
            strength: StatBlock::random_stat(),
            dexterity: StatBlock::random_stat(),
            constitution: StatBlock::random_stat(),
            intelligence: StatBlock::random_stat(),
            wisdom: StatBlock::random_stat(),
            charisma: StatBlock::random_stat(),
        }
    }

    fn random_stat() -> u8 {
        let mut rolls : [u8; 4] = [0; 4];

        let mut rng = thread_rng();

        for x in  0..rolls.len() {
            rolls[x] = rng.gen_range(1, 7);
        }

        rolls.iter().sum::<u8>() - rolls.iter().min().expect("Error while retrieving minimum from stat rolls")
    }
}
