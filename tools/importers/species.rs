use std::cmp::min;

use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use tree_sitter::Node;

use crate::tree_utils::{edit_field, get_field_value};
use crate::{species_matcher, Edit, Personal};

fn learnset_name(entry: &Personal) -> String {
    format!(
        "s{}{}LevelUpLearnset",
        entry.species.species, entry.species.form
    )
}

fn teachable_name(entry: &Personal) -> String {
    format!(
        "s{}{}TeachableLearnset",
        entry.species.species, entry.species.form
    )
}

pub fn handle_species(
    node: Node,
    text: &[u8],
    entry: &Personal,
    species: &[&str],
    moves: &[&str],
) -> Result<Vec<Edit>> {
    let edits = vec![]
        .into_iter()
        .chain(handle_types(node, text, entry)?)
        .chain(handle_base_stats(node, text, entry)?)
        .chain(handle_evos(node, text, entry, species, moves)?)
        .chain([
            handle_abilities(node, text, entry)?,
            edit_field(node, text, "levelUpLearnset", learnset_name(entry))?,
            edit_field(node, text, "teachableLearnset", teachable_name(entry))?,
        ])
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

const ITEM_MAP: &[(usize, &str)] = &[
    (83, "ITEM_GALARICA_CUFF"),
    (84, "ITEM_GALARICA_WREATH"),
    (324, "ITEM_DUBIOUS_DISC"),
    (252, "ITEM_UPGRADE"),
];

fn handle_evos(
    node: Node,
    text: &[u8],
    entry: &Personal,
    species: &[&str],
    moves: &[&str],
) -> Result<Option<Edit>> {
    let evos = (entry.evo_data)
        .iter()
        .map(|evo_data| {
            let matcher = species_matcher(&evo_data.species);
            let species = species.iter().filter(|id| matcher(id)).nth(evo_data.form)?;

            let param = || match &evo_data.parameter {
                serde_json::Value::Number(num) => num.as_u64().map(|i| i as usize),
                serde_json::Value::String(num) => num.parse().ok(),
                _ => None,
            };
            match evo_data.condition.as_str() {
                "LevelUp" => Some(format!("{{EVO_LEVEL, {}, {}}}", evo_data.level, species)),
                "LevelUp_Female" | "LevelUp_Female_Meowstic" => Some(format!(
                    "{{EVO_LEVEL_FEMALE, {}, {}}}",
                    evo_data.level, species
                )),
                "LevelUp_Male" => Some(format!(
                    "{{EVO_LEVEL_MALE, {}, {}}}",
                    evo_data.level, species
                )),
                "LevelUp_Night" => Some(format!(
                    "{{EVO_LEVEL_NIGHT, {}, {}}}",
                    evo_data.level, species
                )),
                "LevelUp_Day" => Some(format!(
                    "{{EVO_LEVEL_DAY, {}, {}}}",
                    evo_data.level, species
                )),
                "EncryptionConstant_Match2" => Some(format!(
                    "{{EVO_LEVEL_FAMILY_OF_THREE, {}, {}}}",
                    evo_data.level, species
                )),
                "EncryptionConstant_Mismatch2" => Some(format!(
                    "{{EVO_LEVEL_FAMILY_OF_FOUR, {}, {}}}",
                    evo_data.level, species
                )),
                "Spinning" => Some(format!("{{EVO_LEVEL, 0, {species}")),
                "LevelUp_WithMove" => {
                    let move_name = moves.get(param()?)?.to_case(Case::ScreamingSnake);
                    Some(format!(
                        "{{EVO_MOVE, MOVE_{move_name}, {species}, {}}}",
                        evo_data.level
                    ))
                }
                "UseItem" => {
                    let index = param()?;
                    let item = ITEM_MAP.iter().find(|(id, _)| *id == index)?.1;
                    Some(format!(
                        "{{EVO_ITEM, {item}, {species}, {}}}",
                        evo_data.level
                    ))
                }
                "Use_RageFist" => Some(format!(
                    "{{EVO_USE_MOVE_TWENTY_TIMES, MOVE_RAGE_FIST, {}}}",
                    species
                )),
                "LevelUp_DefeatBisharp_HoldingLeadersCrest" => Some(
                    "{EVO_DEFEAT_THREE_WITH_ITEM, ITEM_LEADERS_CREST, SPECIES_KINGAMBIT}"
                        .to_string(),
                ),
                "LevelUp_Dusk_WithOwnTempo" => {
                    Some("{EVO_LEVEL_DUSK, 25, SPECIES_LYCANROC_DUSK}".to_string())
                }

                _ => None,
            }
        })
        .collect::<Option<Vec<_>>>();
    let Some(evos) = evos else {
        println!("Unhandled evo: {:?}", entry.species);
        return Ok(None);
    };
    let value = if evos.is_empty() {
        "NULL".to_string()
    } else {
        format!("EVOLUTION({})", evos.join(", "))
    };
    Ok(Some(edit_field(node, text, "evolutions", value)?))
}

fn build_learnset(entry: &Personal) -> Result<String> {
    let lvl_name = learnset_name(entry);
    let teach_name = teachable_name(entry);
    let c_level = |lvlup: u8| match lvlup {
        253 => 0,
        n => n,
    };

    let mut move_entries = entry.levelup_moves.clone();
    move_entries.sort_by_key(|lvl| c_level(lvl.level));

    let lvlup = move_entries
        .iter()
        .map(|lvlup| {
            format!(
                "    {{.move = MOVE_{}, .level = {}}},\n",
                lvlup.r#move.to_case(Case::ScreamingSnake),
                c_level(lvlup.level)
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let tms = entry
        .tm_moves
        .iter()
        .map(|tm| format!("    MOVE_{},\n", tm.to_case(Case::ScreamingSnake)))
        .collect::<Vec<_>>()
        .join("");
    Ok(format!(
        "
static const struct LevelUpMove {lvl_name}[] = {{
{lvlup}
    {{.move = LEVEL_UP_MOVE_END, .level = 0}}
}};

static const u16 {teach_name}[] = {{
{tms}
    MOVE_UNAVAILABLE
}};"
    ))
}

pub fn build_learnsets(entries: &[Personal]) -> Result<String> {
    let learnsets = entries
        .iter()
        .filter(|entry| entry.is_present)
        .map(build_learnset)
        .collect::<Result<Vec<_>>>()?
        .join("");
    Ok(format!(
        "
static const struct LevelUpMove sNoneLevelUpLearnset[] = {{
    {{.move = MOVE_SYNTHESIS, .level = 1}},
    {{.move = LEVEL_UP_MOVE_END, .level = 0}},
}};

static const u16 sNoneTeachableLearnset[] = {{
    MOVE_UNAVAILABLE,
}};

{learnsets}
    "
    ))
}
