use super::{Gender, Race};

pub mod dragonborn;
pub mod dwarves;
pub mod elves;
pub mod gnomes;
pub mod half_orcs;
pub mod halflings;
pub mod humans;
pub mod tieflings;

pub trait RandomName {
    fn random_male_name() -> Result<String, &'static str>;

    fn random_female_name() -> Result<String, &'static str>;

    fn random_last_name() -> Result<String, &'static str>;
}

pub fn get_appropriate_name(gender: Gender, race: Race) -> String {
    match race {
        Race::Dragonborn(_) => {
            match gender {
                Gender::Male => {
                    dragonborn::Dragonborn::random_male_name().expect("Error retrieveing dragonborn male name")
                }
                Gender::Female => {
                    dragonborn::Dragonborn::random_female_name().expect("Error retrieveing dragonborn female name")
                }
            }
        }
        Race::Dwarf(_) => {
            match gender {
                Gender::Male => {
                    dwarves::Dwarf::random_male_name().expect("Error retrieving dwarvern male name")
                }
                Gender::Female => {
                    dwarves::Dwarf::random_female_name().expect("Error retrieving dwarvern female name")
                }
            }
        }
        Race::Elf(_) => {
            match gender {
                Gender::Male => {
                    elves::Elf::random_male_name().expect("Error retrieving elven male name")
                }
                Gender::Female => {
                    elves::Elf::random_female_name().expect("Error retrieving elven female name")
                }
            }
        }
        Race::Gnome(_) => {
            match gender {
                Gender::Male => {
                    gnomes::Gnome::random_male_name().expect("Error retrieving gnomish male name")
                }
                Gender::Female => {
                    gnomes::Gnome::random_female_name().expect("Error retrieving gnomish female name")
                }
            }
        }
        Race::HalfOrc(_) => {
            match gender {
                Gender::Male => {
                    half_orcs::HalfOrc::random_male_name().expect("Error retrieving half-orc male name")
                }
                Gender::Female => {
                    half_orcs::HalfOrc::random_female_name().expect("Error retrieving half-orc female name")
                }
            }
        }
        Race::Halfling(_) => {
            match gender {
                Gender::Male => {
                    halflings::Halfling::random_male_name().expect("Error retrieving halfling male name")
                }
                Gender::Female => {
                    halflings::Halfling::random_female_name().expect("Error retrieving halfling female name")
                }
            }
        }
        Race::Human(_) => {
            match gender {
                Gender::Male => {
                    humans::Human::random_male_name().expect("Error retrieving human male name")
                }
                Gender::Female => {
                    humans::Human::random_female_name().expect("Error retrieving human female name")
                }
            }
        }
        Race::Tiefling(_) => {
            match gender {
                Gender::Male => {
                    tieflings::Tiefling::random_male_name().expect("Error retrieving tiefling male name")
                }
                Gender::Female => {
                    tieflings::Tiefling::random_female_name().expect("Error retrieving tiefling female name")
                }
            }
        }
    }
}