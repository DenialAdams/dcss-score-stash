use models::{NewSpecies, NewBackground};
pub const SPECIES: [NewSpecies; 38] = [
    NewSpecies {
        short: "Ba",
        name: "Barachi",
        playable: 1,
    },
    NewSpecies {
        short: "Ce",
        name: "Centaur",
        playable: 1,
    },
    NewSpecies {
        short: "DD",
        name: "Deep Dwarf",
        playable: 1,
    },
    NewSpecies {
        short: "DE",
        name: "Deep Elf",
        playable: 1,
    },
    NewSpecies {
        short: "Dg",
        name: "Demigod",
        playable: 1,
    },
    NewSpecies {
        short: "Dr",
        name: "Draconian",
        playable: 1,
    },
    NewSpecies {
        short: "Ds",
        name: "Demonspawn",
        playable: 1,
    },
    NewSpecies {
        short: "Fe",
        name: "Felid",
        playable: 1,
    },
    NewSpecies {
        short: "Fo",
        name: "Formicid",
        playable: 1,
    },
    NewSpecies {
        short: "Gh",
        name: "Ghoul",
        playable: 1,
    },
    NewSpecies {
        short: "Gn",
        name: "Gnoll",
        playable: 1,
    },
    NewSpecies {
        short: "Gr",
        name: "Gargoyle",
        playable: 1,
    },
    NewSpecies {
        short: "HE",
        name: "High Elf",
        playable: 1,
    },
    NewSpecies {
        short: "HO",
        name: "Hill Orc",
        playable: 1,
    },
    NewSpecies {
        short: "Ha",
        name: "Halfling",
        playable: 1,
    },
    NewSpecies {
        short: "Hu",
        name: "Human",
        playable: 1,
    },
    NewSpecies {
        short: "Ko",
        name: "Kobold",
        playable: 1,
    },
    NewSpecies {
        short: "Mf",
        name: "Merfolk",
        playable: 1,
    },
    NewSpecies {
        short: "Mi",
        name: "Minotaur",
        playable: 1,
    },
    NewSpecies {
        short: "Mu",
        name: "Mummy",
        playable: 1,
    },
    NewSpecies {
        short: "Na",
        name: "Naga",
        playable: 1,
    },
    NewSpecies {
        short: "Op",
        name: "Octopode",
        playable: 1,
    },
    NewSpecies {
        short: "Og",
        name: "Ogre",
        playable: 1,
    },
    NewSpecies {
        short: "Sp",
        name: "Spriggan",
        playable: 1,
    },
    NewSpecies {
        short: "Te",
        name: "Tengu",
        playable: 1,
    },
    NewSpecies {
        short: "Tr",
        name: "Troll",
        playable: 1,
    },
    NewSpecies {
        short: "VS",
        name: "Vine Stalker",
        playable: 1,
    },
    NewSpecies {
        short: "Vp",
        name: "Vampire",
        playable: 1,
    },
    // non-playable
    NewSpecies {
        short: "El",
        name: "Elf",
        playable: 0,
    },
    NewSpecies {
        short: "Gm",
        name: "Gnome",
        playable: 0,
    },
    NewSpecies {
        short: "OM",
        name: "Ogre-Mage",
        playable: 0,
    },
    NewSpecies {
        short: "HD",
        name: "Hill Dwarf",
        playable: 0,
    },
    NewSpecies {
        short: "MD",
        name: "Mountain Dwarf",
        playable: 0,
    },
    NewSpecies {
        short: "GE",
        name: "Grey Elf",
        playable: 0,
    },
    NewSpecies {
        short: "SE",
        name: "Sludge Elf",
        playable: 0,
    },
    NewSpecies {
        short: "LO",
        name: "Lava Orc",
        playable: 0,
    },
    NewSpecies {
        short: "Dj",
        name: "Djinni",
        playable: 0,
    },
    NewSpecies {
        short: "Pl",
        name: "Plutonian",
        playable: 0,
    },
];
pub const BACKGROUNDS: [NewBackground; 33] = [
    NewBackground {
        short: "AE",
        name: "Air Elementalist",
        playable: 1,
    },
    NewBackground {
        short: "AK",
        name: "Abyssal Knight",
        playable: 1,
    },
    NewBackground {
        short: "AM",
        name: "Arcane Marksman",
        playable: 1,
    },
    NewBackground {
        short: "Ar",
        name: "Artificer",
        playable: 1,
    },
    NewBackground {
        short: "As",
        name: "Assassin",
        playable: 1,
    },
    NewBackground {
        short: "Be",
        name: "Berserker",
        playable: 1,
    },
    NewBackground {
        short: "CK",
        name: "Chaos Knight",
        playable: 1,
    },
    NewBackground {
        short: "Cj",
        name: "Conjurer",
        playable: 1,
    },
    NewBackground {
        short: "EE",
        name: "Earth Elementalist",
        playable: 1,
    },
    NewBackground {
        short: "En",
        name: "Enchanter",
        playable: 1,
    },
    NewBackground {
        short: "FE",
        name: "Fire Elementalist",
        playable: 1,
    },
    NewBackground {
        short: "Fi",
        name: "Fighter",
        playable: 1,
    },
    NewBackground {
        short: "Gl",
        name: "Gladiator",
        playable: 1,
    },
    NewBackground {
        short: "Hu",
        name: "Hunter",
        playable: 1,
    },
    NewBackground {
        short: "IE",
        name: "Ice Elementalist",
        playable: 1,
    },
    NewBackground {
        short: "Mo",
        name: "Monk",
        playable: 1,
    },
    NewBackground {
        short: "Ne",
        name: "Necromancer",
        playable: 1,
    },
    NewBackground {
        short: "Sk",
        name: "Skald",
        playable: 1,
    },
    NewBackground {
        short: "Su",
        name: "Summoner",
        playable: 1,
    },
    NewBackground {
        short: "Tm",
        name: "Transmuter",
        playable: 1,
    },
    NewBackground {
        short: "VM",
        name: "Venom Mage",
        playable: 1,
    },
    NewBackground {
        short: "Wn",
        name: "Wanderer",
        playable: 1,
    },
    NewBackground {
        short: "Wr",
        name: "Warper",
        playable: 1,
    },
    NewBackground {
        short: "Wz",
        name: "Wizard",
        playable: 1,
    },
    // non-playable
    NewBackground {
        short: "Cr",
        name: "Crusader",
        playable: 0,
    },
    NewBackground {
        short: "DK",
        name: "Death Knight",
        playable: 0,
    },
    NewBackground {
        short: "He",
        name: "Healer",
        playable: 0,
    },
    NewBackground {
        short: "Pa",
        name: "Paladin",
        playable: 0,
    },
    NewBackground {
        short: "Pr",
        name: "Priest",
        playable: 0,
    },
    NewBackground {
        short: "Re",
        name: "Reaver",
        playable: 0,
    },
    NewBackground {
        short: "St",
        name: "Stalker",
        playable: 0,
    },
    NewBackground {
        short: "Th",
        name: "Thief",
        playable: 0,
    },
    NewBackground {
        short: "Jr",
        name: "Jester",
        playable: 0,
    },
];
