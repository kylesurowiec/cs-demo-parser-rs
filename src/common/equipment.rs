use crate::sendtables::entity::{Entity, Vector};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EquipmentClass {
    #[default]
    Unknown = 0,
    Pistols = 1,
    Smg = 2,
    Heavy = 3,
    Rifle = 4,
    Equipment = 5,
    Grenade = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
#[derive(Default)]
pub enum EquipmentType {
    #[default]
    Unknown = 0,
    // Pistols
    P2000 = 1,
    Glock = 2,
    P250 = 3,
    Deagle = 4,
    FiveSeven = 5,
    DualBerettas = 6,
    Tec9 = 7,
    Cz75 = 8,
    Usp = 9,
    Revolver = 10,
    // SMGs
    Mp7 = 101,
    Mp9 = 102,
    Bizon = 103,
    Mac10 = 104,
    Ump45 = 105,
    P90 = 106,
    Mp5 = 107,
    // Heavy
    SawedOff = 201,
    Nova = 202,
    Mag7 = 203,
    Xm1014 = 204,
    M249 = 205,
    Negev = 206,
    // Rifles
    Galil = 301,
    Famas = 302,
    Ak47 = 303,
    M4A4 = 304,
    M4A1 = 305,
    Ssg08 = 306,
    Sg553 = 307,
    Aug = 308,
    Awp = 309,
    Scar20 = 310,
    G3Sg1 = 311,
    // Equipment
    Zeus = 401,
    Kevlar = 402,
    Helmet = 403,
    Bomb = 404,
    Knife = 405,
    DefuseKit = 406,
    World = 407,
    ZoneRepulsor = 408,
    Shield = 409,
    HeavyAssaultSuit = 410,
    NightVision = 411,
    HealthShot = 412,
    TacticalAwarenessGrenade = 413,
    Fists = 414,
    BreachCharge = 415,
    Tablet = 416,
    Axe = 417,
    Hammer = 418,
    Wrench = 419,
    Snowball = 420,
    BumpMine = 421,
    // Grenades
    Decoy = 501,
    Molotov = 502,
    Incendiary = 503,
    Flash = 504,
    Smoke = 505,
    He = 506,
}


impl EquipmentType {
    pub fn class(self) -> EquipmentClass {
        let val = self as i32;
        let class_denominator = 100;
        EquipmentClass::from((((val + class_denominator - 1) / class_denominator)))
    }

    pub fn as_str(self) -> &'static str {
        match self {
            | EquipmentType::P2000 => "P2000",
            | EquipmentType::Glock => "Glock-18",
            | EquipmentType::P250 => "P250",
            | EquipmentType::Deagle => "Desert Eagle",
            | EquipmentType::FiveSeven => "Five-SeveN",
            | EquipmentType::DualBerettas => "Dual Berettas",
            | EquipmentType::Tec9 => "Tec-9",
            | EquipmentType::Cz75 => "CZ75 Auto",
            | EquipmentType::Usp => "USP-S",
            | EquipmentType::Revolver => "R8 Revolver",
            | EquipmentType::Mp7 => "MP7",
            | EquipmentType::Mp9 => "MP9",
            | EquipmentType::Bizon => "PP-Bizon",
            | EquipmentType::Mac10 => "MAC-10",
            | EquipmentType::Ump45 => "UMP-45",
            | EquipmentType::P90 => "P90",
            | EquipmentType::Mp5 => "MP5-SD",
            | EquipmentType::SawedOff => "Sawed-Off",
            | EquipmentType::Nova => "Nova",
            | EquipmentType::Mag7 => "MAG-7",
            | EquipmentType::Xm1014 => "XM1014",
            | EquipmentType::M249 => "M249",
            | EquipmentType::Negev => "Negev",
            | EquipmentType::Galil => "Galil AR",
            | EquipmentType::Famas => "FAMAS",
            | EquipmentType::Ak47 => "AK-47",
            | EquipmentType::M4A4 => "M4A4",
            | EquipmentType::M4A1 => "M4A1",
            | EquipmentType::Ssg08 => "SSG 08",
            | EquipmentType::Sg553 => "SG 553",
            | EquipmentType::Aug => "AUG",
            | EquipmentType::Awp => "AWP",
            | EquipmentType::Scar20 => "SCAR-20",
            | EquipmentType::G3Sg1 => "G3SG1",
            | EquipmentType::Zeus => "Zeus x27",
            | EquipmentType::Kevlar => "Kevlar Vest",
            | EquipmentType::Helmet => "Kevlar + Helmet",
            | EquipmentType::Bomb => "C4",
            | EquipmentType::Knife => "Knife",
            | EquipmentType::DefuseKit => "Defuse Kit",
            | EquipmentType::World => "World",
            | EquipmentType::ZoneRepulsor => "Zone Repulsor",
            | EquipmentType::Shield => "Shield",
            | EquipmentType::HeavyAssaultSuit => "Heavy Assault Suit",
            | EquipmentType::NightVision => "Night Vision",
            | EquipmentType::HealthShot => "Medi-Shot",
            | EquipmentType::TacticalAwarenessGrenade => "TA Grenade",
            | EquipmentType::Fists => "Fists",
            | EquipmentType::BreachCharge => "Breach Charge",
            | EquipmentType::Tablet => "Tablet",
            | EquipmentType::Axe => "Axe",
            | EquipmentType::Hammer => "Hammer",
            | EquipmentType::Wrench => "Wrench",
            | EquipmentType::Snowball => "Snowball",
            | EquipmentType::BumpMine => "Bump Mine",
            | EquipmentType::Decoy => "Decoy Grenade",
            | EquipmentType::Molotov => "Molotov",
            | EquipmentType::Incendiary => "Incendiary Grenade",
            | EquipmentType::Flash => "Flashbang",
            | EquipmentType::Smoke => "Smoke Grenade",
            | EquipmentType::He => "HE Grenade",
            | EquipmentType::Unknown => "UNKNOWN",
        }
    }
}

impl EquipmentClass {
    fn from(v: i32) -> Self {
        match v {
            | 1 => EquipmentClass::Pistols,
            | 2 => EquipmentClass::Smg,
            | 3 => EquipmentClass::Heavy,
            | 4 => EquipmentClass::Rifle,
            | 5 => EquipmentClass::Equipment,
            | 6 => EquipmentClass::Grenade,
            | _ => EquipmentClass::Unknown,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Equipment {
    pub equipment_type: EquipmentType,
    pub entity: Option<Entity>,
    pub original_string: String,
    pub unique_id: i64,
    pub position: Vector,
}

/// Maps a weapon or equipment name to [`EquipmentType`].
pub fn map_equipment(name: &str) -> EquipmentType {
    let mut n = name.to_lowercase();
    if let Some(stripped) = n.strip_prefix("weapon_") {
        n = stripped.to_string();
    }

    if n.contains("knife") || n.contains("bayonet") {
        return EquipmentType::Knife;
    }

    match n.as_str() {
        | "ak47" => EquipmentType::Ak47,
        | "aug" => EquipmentType::Aug,
        | "awp" => EquipmentType::Awp,
        | "bizon" => EquipmentType::Bizon,
        | "c4" | "planted_c4" => EquipmentType::Bomb,
        | "deagle" => EquipmentType::Deagle,
        | "decoy" | "decoygrenade" | "decoyprojectile" | "decoy_projectile" => EquipmentType::Decoy,
        | "elite" => EquipmentType::DualBerettas,
        | "famas" => EquipmentType::Famas,
        | "fiveseven" => EquipmentType::FiveSeven,
        | "flashbang" => EquipmentType::Flash,
        | "g3sg1" => EquipmentType::G3Sg1,
        | "galil" | "galilar" => EquipmentType::Galil,
        | "glock" => EquipmentType::Glock,
        | "hegrenade" => EquipmentType::He,
        | "hkp2000" => EquipmentType::P2000,
        | "incgrenade" | "incendiarygrenade" => EquipmentType::Incendiary,
        | "m249" => EquipmentType::M249,
        | "m4a1" => EquipmentType::M4A4,
        | "mac10" => EquipmentType::Mac10,
        | "mag7" => EquipmentType::Mag7,
        | "molotov" | "molotovgrenade" | "molotovprojectile" | "molotov_projectile" => {
            EquipmentType::Molotov
        },
        | "mp7" => EquipmentType::Mp7,
        | "mp5sd" => EquipmentType::Mp5,
        | "mp9" => EquipmentType::Mp9,
        | "negev" => EquipmentType::Negev,
        | "nova" => EquipmentType::Nova,
        | "p250" => EquipmentType::P250,
        | "p90" => EquipmentType::P90,
        | "sawedoff" => EquipmentType::SawedOff,
        | "scar20" => EquipmentType::Scar20,
        | "sg556" => EquipmentType::Sg553,
        | "smokegrenade" | "smokegrenadeprojectile" | "smokegrenade_projectile" => {
            EquipmentType::Smoke
        },
        | "ssg08" => EquipmentType::Ssg08,
        | "taser" => EquipmentType::Zeus,
        | "tec9" => EquipmentType::Tec9,
        | "ump45" => EquipmentType::Ump45,
        | "xm1014" => EquipmentType::Xm1014,
        | "m4a1_silencer" | "m4a1_silencer_off" => EquipmentType::M4A1,
        | "cz75a" => EquipmentType::Cz75,
        | "usp" | "usp_silencer" | "usp_silencer_off" => EquipmentType::Usp,
        | "world" | "worldspawn" => EquipmentType::World,
        | "inferno" => EquipmentType::Incendiary,
        | "revolver" => EquipmentType::Revolver,
        | "vest" => EquipmentType::Kevlar,
        | "vesthelm" => EquipmentType::Helmet,
        | "defuser" => EquipmentType::DefuseKit,
        | "scar17" | "sensorgrenade" | "mp5navy" | "p228" | "scout" | "sg550" | "sg552" | "tmp" => {
            EquipmentType::Unknown
        },
        | _ => EquipmentType::Unknown,
    }
}
