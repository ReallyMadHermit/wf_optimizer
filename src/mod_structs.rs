#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct Mod {
    mod_type: ModType
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
pub enum ModType {
    Primary,
    Rifle,
    Shotgun,
    Pistol,
    Melee
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord)]
pub enum ModCategory {
    Damage,
    Elemental,
    CritChance,
    CritDamage,
    MultiShot,
    FireRate
}