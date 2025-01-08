use std::fs::{self, read, write};

use anyhow::{bail, Context, Result};
use convert_case::{Case, Casing};
use serde::Deserialize;

use crate::species::{species_list, species_matcher};
use crate::PersonalArray;

const TRAINER_LIST: &[(&str, &str, &str)] = &[
    ("rival_02_hono", "May", "May"),
    ("sister_01_01", "Leaf", "Leaf"),
    ("brother_01_01", "Wally", "Wally"),
    ("gym_mushi_leader_01", "Leader Roxanne", "Roxanne"),
    // TITAN_KLAWF
    ("gym_kusa_leader_01", "Leader Brawly", "Brawly"),
    ("brother_01_02", "Wally", "Wally"),
    // TITAN_BOMBIRDIER
    ("dan_aku_boss_01", "Aqua Admin M", "Matt"),
    ("rival_04_hono", "May", "May"),
    ("gym_denki_leader_01", "Leader Wattson", "Wattson"),
    ("dan_hono_boss_01", "Magma Admin", "Tabitha"),
    ("sister_01_02", "Leaf", "Leaf"),
    ("brother_01_03", "Wally", "Wally"),
    // TITAN ORTHWORM
    ("gym_mizu_leader_01", "Leader Flannery", "Flannery"),
    ("dan_doku_boss_01", "Aqua Admin F", "Shelly"),
    ("brother_01_04", "Wally", "Wally"),
    ("gym_normal_leader_01", "Leader Norman", "Norman"),
    ("rival_05_hono", "May", "May"),
    // TITAN_LOYAL_3
    ("gym_ghost_leader_01", "Leader Winona", "Winona"), // Special effects ?
    // TITAN_WEIRD_DONFAN
    ("brother_01_05", "Wally", "Wally"),
    // TITAN_OGERPONS
    ("sister_01_03", "Leaf", "Leaf"),
    ("gym_esper_leader_01", "Leader Tate And Liza", "Tate&Liza"),
    ("gym_koori_leader_01", "Leader Juan", "Juan"),
    ("dan_fairy_boss_01", "Aqua Leader Archie", "Archie"),
    ("dan_kakutou_boss_01", "Magma Leader Maxie", "Maxie"),
    ("pepper_01", "Factory Head Noland", "Arven"),
    ("clavel_01_hono", "Gentleman", "Clavell"),
    ("botan_01", "Arena Tycoon Greta", "Penny"),
    ("e4_jimen_01", "Elite Four Sidney", "Rika"),
    ("e4_hagane_01", "Elite Four Phoebe", "Poppy"),
    ("e4_hikou_01", "Elite Four Glacia", "Larry"),
    ("e4_dragon_01", "Elite Four Drake", "Hassel"),
    ("chairperson_01", "Champion Wallace", "Geeta"),
];

#[derive(Debug, Deserialize)]
struct Stats {
    hp: u16,
    atk: u16,
    def: u16,
    spa: u16,
    spd: u16,
    spe: u16,
}

#[derive(Debug, Deserialize)]
struct Move {
    move_id: String,
    point_up: u8,
}

#[derive(Debug, Deserialize)]
struct Pokemon {
    poke_id: String,
    form_id: usize,
    sex: String,
    item: String,
    level: u16,
    nature: String,
    gem_type: String,
    move1: Move,
    move2: Move,
    move3: Move,
    move4: Move,
    ability: String,
    iv_values: Stats,
    effort_value: Stats,
}

#[derive(Debug, Deserialize)]
struct Trainer {
    tr_id: String,
    trainer_type: String,
    battle_type: String,
    change_gem: bool,
    poke1: Pokemon,
    poke2: Pokemon,
    poke3: Pokemon,
    poke4: Pokemon,
    poke5: Pokemon,
    poke6: Pokemon,
}

#[derive(Debug, Deserialize)]
struct TrData {
    table: Vec<Trainer>,
}

fn stats_to_string(stats: &Stats) -> String {
    format!(
        "{} HP / {} Atk / {} Def / {} SpA / {} SpD / {} Spe",
        stats.hp, stats.atk, stats.def, stats.spa, stats.spd, stats.spe
    )
}

fn pokemon(
    data: &Pokemon,
    species_list: &[String],
    personal_data: &PersonalArray,
) -> Result<String> {
    if data.level == 0 {
        return Ok(String::new());
    }

    let personal = (personal_data.entry)
        .iter()
        .find(|personal| {
            personal.species.species == data.poke_id && personal.species.form == data.form_id
        })
        .context("pokemon")?;
    let ability = match data.ability.as_str() {
        "Set_1" => &personal.ability_1,
        "Set_2" => &personal.ability_2,
        "Set_3" => &personal.ability_hidden,
        _ => bail!("Unexpected ability {}", data.ability),
    }
    .to_case(Case::ScreamingSnake);

    let matcher = species_matcher(&data.poke_id);
    let species_id = species_list
        .iter()
        .filter(|str| matcher(str))
        .nth(data.form_id)
        .cloned()
        .unwrap_or_else(|| {
            format!(
                "SPECIES_{} /* {} */",
                data.poke_id.to_case(Case::ScreamingSnake),
                data.poke_id
            )
        });
    let level = data.level;
    let item = data.item.to_case(Case::ScreamingSnake);
    let ivs = stats_to_string(&data.iv_values);
    let evs = stats_to_string(&data.effort_value);
    let nature = &data.nature;
    let tera_type = match data.gem_type.as_str() {
        "Default" => String::new(),
        other => format!("Tera Type: {}\n", other),
    };

    let moves = [&data.move1, &data.move2, &data.move3, &data.move4];
    let moves = moves
        .map(|mov| format!("- MOVE_{}", mov.move_id.to_case(Case::ScreamingSnake)))
        .join("\n");
    Ok(format!(
        "
{species_id} @ ITEM_{item}
Level: {level}
Ability: ABILITY_{ability}
IVs: {ivs}
EVs: {evs}
Nature: {nature}
{tera_type}{moves}
"
    ))
}

fn trainer(
    data: &Trainer,
    pic: &str,
    name: &str,
    species_list: &[String],
    personal_data: &PersonalArray,
) -> Result<String> {
    let id = data.tr_id.to_uppercase();
    let name = name.to_uppercase();
    let gender = "Male";
    let double_battle = match data.battle_type.as_str() {
        "Single" => "No",
        "Double" => "Yes",
        _ => "No",
    };
    let music = "Female";

    let pokes = [
        &data.poke1,
        &data.poke2,
        &data.poke3,
        &data.poke4,
        &data.poke5,
        &data.poke6,
    ];

    let pokes: Vec<String> = pokes
        .iter()
        .map(|poke| pokemon(poke, species_list, personal_data))
        .collect::<Result<Vec<_>>>()?;
    let pokes = pokes.join("");
    Ok(format!(
        "
=== TRAINER_{id} ===
Name: {name}
Pic: {pic}
Gender: {gender}
Music: {music}
Double Battle: {double_battle}
AI: Basic Trainer

{pokes}
"
    ))
}

pub fn trainers() -> Result<()> {
    let trainers: TrData = serde_json::from_slice(&read("resources/trdata_array.json")?)?;
    let personal: PersonalArray = serde_json::from_slice(&read("resources/personal_array.json")?)?;
    let base_parties = fs::read_to_string("../../src/data/trainers.base.party")?;
    let species = species_list()?;

    let new_parties = (trainers.table)
        .iter()
        .filter_map(|tr| Some((tr, TRAINER_LIST.iter().find(|(id, ..)| *id == tr.tr_id)?)))
        .collect::<Vec<_>>();
    let party_defs = new_parties
        .iter()
        .map(|(tr, (_, pic, name))| trainer(tr, pic, name, &species, &personal))
        .collect::<Result<Vec<_>>>()?
        .join("");

    write("../../src/data/trainers.party", base_parties + &party_defs)?;
    Ok(())
}
