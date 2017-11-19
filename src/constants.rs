use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
#[repr(i64)]
pub enum Species {
    Barachi = 0,
    Centaur,
    DeepDwarf,
    DeepElf,
    Demigod,
    Demonspawn,
    Draconian,
    RedDraconian,
    WhiteDraconian,
    GreenDraconian,
    YellowDraconian,
    GreyDraconian,
    BlackDraconian,
    PurpleDraconian,
    MottledDraconian,
    PaleDraconian,
    Felid,
    Formicid,
    Gargoyle,
    Ghoul,
    Gnoll,
    Halfling,
    HighElf,
    HillOrc,
    Human,
    Kobold,
    Merfolk,
    Minotaur,
    Mummy,
    Naga,
    Ocotopode,
    Ogre,
    Spriggan,
    Tengu,
    Troll,
    Vampire,
    VineStalker,
}

impl FromStr for Species {
    type Err = ();
    fn from_str(s: &str) -> Result<Species, ()> {
        if s.ends_with("Draconian") {
            return Ok(Species::Draconian);
        }
        match s {
            "Barachi" => Ok(Species::Barachi),
            "Centaur" => Ok(Species::Centaur),
            "Deep Dwarf" => Ok(Species::DeepDwarf),
            "Deep Elf" => Ok(Species::DeepElf),
            "Demigod" => Ok(Species::Demigod),
            "Demonspawn" => Ok(Species::Demonspawn),
            "Draconian" => Ok(Species::Draconian),
            // These are impossible to hit for now,
            // eventually sub races will be a possibility? TOOD
            "Red Draconian" => Ok(Species::RedDraconian),
            "White Draconian" => Ok(Species::WhiteDraconian),
            "Green Draconian" => Ok(Species::GreenDraconian),
            "Yellow Draconian" => Ok(Species::YellowDraconian),
            "Grey Draconian" => Ok(Species::GreyDraconian),
            "Black Draconian" => Ok(Species::BlackDraconian),
            "Purple Draconian" => Ok(Species::PurpleDraconian),
            "Mottled Draconian" => Ok(Species::MottledDraconian),
            "Pale Draconian" => Ok(Species::PaleDraconian),
            // -----------
            "Felid" => Ok(Species::Felid),
            "Formicid" => Ok(Species::Formicid),
            "Gargoyle" => Ok(Species::Gargoyle),
            "Ghoul" => Ok(Species::Ghoul),
            "Gnoll" => Ok(Species::Gnoll),
            "Halfling" => Ok(Species::Halfling),
            "High Elf" => Ok(Species::HighElf),
            "Hill Orc" => Ok(Species::HillOrc),
            "Human" => Ok(Species::Human),
            "Kobold" => Ok(Species::Kobold),
            "Merfolk" => Ok(Species::Merfolk),
            "Minotaur" => Ok(Species::Minotaur),
            "Mummy" => Ok(Species::Mummy),
            "Naga" => Ok(Species::Naga),
            "Octopode" => Ok(Species::Ocotopode),
            "Ogre" => Ok(Species::Ogre),
            "Spriggan" => Ok(Species::Spriggan),
            "Tengu" => Ok(Species::Tengu),
            "Troll" => Ok(Species::Troll),
            "Vampire" => Ok(Species::Vampire),
            "Vine Stalker" => Ok(Species::VineStalker),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(i64)]
pub enum Background {
    Fighter = 0,
    Gladiator,
    Monk,
    Hunter,
    Assassin,
    Berserker,
    AbyssalKnight,
    ChaosKnight,
    Skald,
    Enchanter,
    Transmuter,
    ArcaneMarksman,
    Warper,
    Wizard,
    Conjurer,
    Summoner,
    Necromancer,
    FireElementalist,
    IceElementalist,
    AirElementalist,
    EarthElementalist,
    VenomMage,
    Artificer,
    Wanderer,
}

impl FromStr for Background {
    type Err = ();
    fn from_str(s: &str) -> Result<Background, ()> {
        match s {
            "Fighter" => Ok(Background::Fighter),
            "Gladiator" => Ok(Background::Gladiator),
            "Monk" => Ok(Background::Monk),
            "Hunter" => Ok(Background::Hunter),
            "Assassin" => Ok(Background::Assassin),
            "Berserker" => Ok(Background::Berserker),
            "Abyssal Knight" => Ok(Background::AbyssalKnight),
            "Chaos Knight" => Ok(Background::ChaosKnight),
            "Skald" => Ok(Background::Skald),
            "Enchanter" => Ok(Background::Enchanter),
            "Transmuter" => Ok(Background::Transmuter),
            "Arcane Marksman" => Ok(Background::ArcaneMarksman),
            "Warper" => Ok(Background::Warper),
            "Wizard" => Ok(Background::Wizard),
            "Conjurer" => Ok(Background::Conjurer),
            "Summoner" => Ok(Background::Summoner),
            "Necromancer" => Ok(Background::Necromancer),
            "Fire Elementalist" => Ok(Background::FireElementalist),
            "Ice Elementalist" => Ok(Background::IceElementalist),
            "Air Elementalist" => Ok(Background::AirElementalist),
            "Earth Elementalist" => Ok(Background::EarthElementalist),
            "Venom Mage" => Ok(Background::VenomMage),
            "Artificer" => Ok(Background::Artificer),
            "Wanderer" => Ok(Background::Wanderer),
            _ => Err(()),
        }
    }
}
