#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ModStatType {
    None,
    Damage,
    Heat,
    Cold,
    Toxic,
    Shock,
    Magnetic,
    Radiation,
    Multishot,
    CritChance,
    CritDamage,
    FireRate,
    StatusChance,
    ConditionOverload,  // always conditional
    MagazineCapacity,
    ReloadSpeed,
    AcuityBonus,  // crit chance + weak point damage
    StatusDamage,
    PunchThrough,
    AmmoEfficiency,
    Riven
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModData {
    pub stat_type_1: ModStatType,
    pub stat_type_2: ModStatType,
    pub stat_value_1: i16,
    pub stat_value_2: i16
}

pub struct LoadedMods {
    pub mod_names: Vec<String>,
    pub mod_data: Vec<ModData>,
    pub included_mods: [u8; 8],
    pub mod_count: u8,
    pub arcane_count: u8
}

#[derive(Clone)]
pub struct GunModSums {
    pub damage: i16,
    pub ele_damage: i16,
    pub multishot: i16,
    pub crit_chance: i16,
    pub crit_damage: i16,
    pub status: i16,
    pub fire_rate: i16,
    pub magazine: i16,
    pub reload: i16,
    pub ammo_efficiency: i16
}

#[derive(Copy, Clone)]
pub struct HitStats {
    pub damage: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub status: f32
}

pub struct GunData {
    pub name: String,
    pub gun_type: WeaponType,
    pub semi: bool,
    pub gun_stats: GunStats,
}

#[derive(Clone)]
pub struct GunStats {
    pub fire_rate: f32,
    pub multishot: f32,
    pub magazine: f32,
    pub reload: f32,
    pub hit_stats: [HitStats; 2]
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum WeaponType {
    Rifle,
    Shotgun,
    Pistol,
    Bow,
    Riven,
    Primary
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DamageCriteria {
    PerShot,
    BurstDPS,
    SustainedDPS
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ModBehavior {
    Exclude,
    Include,
    Parallel,
    NotExclude,
    NotInclude,
    NotParallel,
    NothingSpecial
}

#[derive(Clone, Eq, PartialEq)]
pub struct ModdingContext {
    pub gun_type: WeaponType,
    pub damage_criteria: DamageCriteria,
    pub kills: bool,
    pub aiming: bool,
    pub semi: bool,
    pub acuity: bool,
    pub prefer_amalgam: bool,
    pub riven: bool
}