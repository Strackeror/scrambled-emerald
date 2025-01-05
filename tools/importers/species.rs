use std::cmp::min;

use anyhow::Result;
use convert_case::{Case, Casing};
use tree_sitter::Node;

use crate::{tree_utils::edit_field, Edit, Personal};

pub fn handle_species(node: Node, text: &[u8], entry: &Personal) -> Result<Vec<Edit>> {
    let edits = vec![]
        .into_iter()
        .chain(handle_types(node, text, entry)?)
        .chain(handle_base_stats(node, text, entry)?)
        .chain([handle_abilities(node, text, entry)?])
        .collect();
    Ok(edits)
}

fn handle_types(node: Node, text: &[u8], entry: &Personal) -> Result<Option<Edit>> {
    if entry.type_1 == entry.type_2 {
        let field_value = format!("MON_TYPES(TYPE_{})", entry.type_1.to_uppercase());
        Ok(Some(edit_field(node, text, "types", field_value)?))
    } else {
        let value = format!(
            "MON_TYPES(TYPE_{}, TYPE_{})",
            entry.type_1.to_uppercase(),
            entry.type_2.to_uppercase()
        );
        Ok(Some(edit_field(node, text, "types", value)?))
    }
}

fn handle_base_stats(node: Node, text: &[u8], entry: &Personal) -> Result<Vec<Edit>> {
    Ok(vec![
        edit_field(node, text, "baseHP", min(255, entry.base_stats.HP))?,
        edit_field(node, text, "baseAttack", min(255, entry.base_stats.ATK))?,
        edit_field(node, text, "baseDefense", min(255, entry.base_stats.DEF))?,
        edit_field(node, text, "baseSpeed", min(255, entry.base_stats.SPE))?,
        edit_field(node, text, "baseSpAttack", min(255, entry.base_stats.SPA))?,
        edit_field(node, text, "baseSpDefense", min(255, entry.base_stats.SPD))?,
    ])
}

const ABILITY_OVERRIDES: &[(&str, &str)] = &[
    ("RapidFire", "Triage"),
    ("Brazen", "Iron Fist"),
    ("Amplifier", "Punk Rock"),
    ("Ruthless", "Sniper"),
    ("Headstrong", "Rock Head"),
    ("SixthSense", "Compound Eyes"),
    ("Elegance", "Queenly Majesty"),
    ("Swiftness", "Armor Tail"),
    ("Subterranean", "Earth Eater"),
    ("LiquidVeil", "Water Bubble"),
    ("Instinct", "Download"),
    ("BruteForce", "Gorilla Tactics"),
    ("Expertise", "Parental Bond"),
    ("Hubris", "SoulHeart"),
    ("Proficiency", "Beast Boost"),
    ("Versatility", "Protean"),
    ("EnergyShield", "ShadowShield"),
    ("Permafrost", "Ice Scales"),
    ("TacticalRetreat", "EmergencyExit"),
    ("Resilient", "PrismArmor"),
    ("Appetite", "Ripen"),
    ("Barbed", "Iron Barbs"),
    ("GrassyGuard", "GrassPelt"),
    ("VoltRush", "SurgeSurfer"),
    ("SolarRush", "Chlorophyll"),
    ("GuardingGale", "DeltaStream"),
    ("Camouflage", "Mimicry"),
    ("Hypnotize", "WanderingSpirit"),
    ("WellbakedBody", "WellBakedBody"),
    ("RESERVED_307", "TeraShell"),
    ("RESERVED_308", "TeraShift"),
    ("RESERVED_309", "TeraformZero"),
];

fn handle_abilities(node: Node, text: &[u8], entry: &Personal) -> Result<Edit> {
    let ability_format = |name: &str| {
        let name = ABILITY_OVERRIDES
            .iter()
            .find(|(over, _)| *over == name)
            .map_or(name, |(_, name)| name);
        format!("ABILITY_{}", name.to_case(Case::ScreamingSnake))
    };

    let mut abilities = [None, None, None];
    abilities[0] = Some(ability_format(&entry.ability_1));
    if entry.ability_2 != entry.ability_1 {
        abilities[1] = Some(ability_format(&entry.ability_2));
    }
    if entry.ability_hidden != entry.ability_1 {
        abilities[2] = Some(ability_format(&entry.ability_hidden));
    }

    let ability_list = abilities
        .map(|opt| opt.unwrap_or("ABILITY_NONE".to_string()))
        .join(", ");
    let field_value = format!("{{ {} }}", ability_list);

    Ok(edit_field(node, text, "abilities", field_value)?)
}

