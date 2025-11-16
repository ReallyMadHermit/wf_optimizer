use crate::mod_parsing::ModStatType;

const BUFF_STATS: [ModStatType; 11] = [
    ModStatType::Damage,
    ModStatType::Elemental,
    ModStatType::FireRate,
    ModStatType::CritChance,
    ModStatType::CritDamage,
    ModStatType::FlatCritChance,
    ModStatType::FinalCritDamage,
    ModStatType::Bane,
    ModStatType::ReloadSpeed,
    ModStatType::Multishot,
    ModStatType::StatusChance
];

const RIVEN_STATS: [ModStatType; 12] = [
    ModStatType::Damage,
    ModStatType::Multishot,
    ModStatType::CritChance,
    ModStatType::CritDamage,
    ModStatType::Cold,
    ModStatType::Shock,
    ModStatType::Heat,
    ModStatType::Toxic,
    ModStatType::StatusChance,
    ModStatType::FireRate,
    ModStatType::MagazineCapacity,
    ModStatType::ReloadSpeed
];


struct StatScreenApp {
    stat_fields: Vec<(ModStatType, i16)>,
    buffer: String,
    hovered_row: i16,
    selected_field: Option<i16>
} impl StatScreenApp {

    fn edit_buffs(stat_fields: Vec<(ModStatType, i16)>) -> Self {
        Self {
            stat_fields,
            buffer: String::with_capacity(10),
            hovered_row: 0,
            selected_field: None
        }
    }

    fn new_buffs() -> Self {
        Self::new(&BUFF_STATS)
    }

    fn new_riven() -> Self {
        Self::new(&RIVEN_STATS)
    }

    fn new(stat_array: &[ModStatType]) -> Self {
        let mut stat_fields = Vec::with_capacity(stat_array.len());
        for &stat_type in stat_array {
            stat_fields.push((stat_type, 0));
        }
        Self {
            stat_fields,
            buffer: String::with_capacity(10),
            hovered_row: 0,
            selected_field: None
        }
    }

}