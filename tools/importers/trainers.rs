use std::fs::{self, read, write};

use anyhow::{bail, Context, Result};
use convert_case::{Case, Casing};
use serde::Deserialize;

use crate::species::{species_list, species_matcher};
use crate::PersonalArray;

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct Stats {
    hp: u16,
    atk: u16,
    def: u16,
    spa: u16,
    spd: u16,
    spe: u16,
}

fn max_ivs() -> Stats {
    Stats {
        hp: 31,
        atk: 31,
        def: 31,
        spa: 31,
        spd: 31,
        spe: 31,
    }
}

#[derive(Debug, Deserialize)]
struct Move {
    move_id: String,
}

#[derive(Debug, Deserialize)]
struct Pokemon {
    poke_id: String,
    form_id: usize,
    item: String,
    level: u16,
    nature: String,
    gem_type: String,
    move1: Move,
    move2: Move,
    move3: Move,
    move4: Move,
    ability: String,
    #[serde(default = "max_ivs")]
    iv_values: Stats,
    effort_value: Stats,
}

#[derive(Debug, Deserialize)]
struct Trainer {
    tr_id: String,
    change_gem: bool,
    battle_type: String,
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

#[derive(Debug, Deserialize)]
struct TrainerEntry {
    id: String,
    pic: String,
    name: String,
    #[serde(default)]
    pokemon_override: Vec<(usize, Pokemon)>,
}

#[derive(Debug, Deserialize)]
struct TrainerList {
    trainers: Vec<TrainerEntry>,
    partners: Vec<TrainerEntry>,
}

fn stats_to_string(stats: &Stats) -> String {
    format!(
        "{} HP / {} Atk / {} Def / {} SpA / {} SpD / {} Spe",
        stats.hp, stats.atk, stats.def, stats.spa, stats.spd, stats.spe
    )
}

fn pokemon_override(data: &Pokemon) -> Result<String> {
    let id = data.poke_id.to_case(Case::ScreamingSnake);
    let level = data.level;
    let item = data.item.to_case(Case::ScreamingSnake);
    let ivs = stats_to_string(&data.iv_values);
    let evs = stats_to_string(&data.effort_value);
    let nature = &data.nature;
    let tera_type = match data.gem_type.as_str() {
        "Default" => String::new(),
        other => format!("Tera Type: {}\n", other),
    };
    let ability = data.ability.to_case(Case::ScreamingSnake);

    let moves = [&data.move1, &data.move2, &data.move3, &data.move4];
    let moves = moves
        .map(|mov| format!("- MOVE_{}", mov.move_id.to_case(Case::ScreamingSnake)))
        .join("\n");

    Ok(format!(
        "
SPECIES_{id} @ ITEM_{item}
Level: {level}
Ability: ABILITY_{ability}
IVs: {ivs}
EVs: {evs}
Nature: {nature}
{tera_type}{moves}
"
    ))
}

fn pokemon(
    data: &Pokemon,
    species_list: &[String],
    personal_data: &PersonalArray,
    tera_trainer: bool
) -> Result<String> {
    if &data.poke_id == "Egg" {
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
        _ if !tera_trainer => String::new(),
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
    entry: &TrainerEntry,
    species_list: &[String],
    personal_data: &PersonalArray,
    prefix: &str,
) -> Result<String> {
    let id = data.tr_id.to_uppercase();
    let name = entry.name.to_uppercase();
    let pic = entry.pic.as_str();
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

    let mut pokes: Vec<String> = pokes
        .iter()
        .map(|poke| pokemon(poke, species_list, personal_data, data.change_gem))
        .collect::<Result<Vec<_>>>()?;
    for (index, pokemon) in &entry.pokemon_override {
        *pokes.get_mut(*index).context("override")? = pokemon_override(&pokemon)?;
    }
    let pokes = pokes.join("");
    Ok(format!(
        "
=== {prefix}_{id} ===
Name: {name}
Pic: {pic}
Gender: {gender}
Music: {music}
Double Battle: {double_battle}
AI: Basic Trainer / Ace Pokemon

{pokes}
"
    ))
}

pub fn trainers() -> Result<()> {
    let trainers: TrData = serde_json::from_slice(&read("resources/trdata_array.json")?)?;
    let personal: PersonalArray = serde_json::from_slice(&read("resources/personal_array.json")?)?;
    let entries: TrainerList = serde_json::from_slice(&read("resources/trainer_list.json")?)?;
    let base_parties = fs::read_to_string("../../src/data/trainers.base.party")?;
    let base_partners = fs::read_to_string("../../src/data/battle_partners.base.party")?;
    let species = species_list()?;

    let new_parties = (trainers.table)
        .iter()
        .filter_map(|tr| Some((tr, entries.trainers.iter().find(|t| t.id == tr.tr_id)?)))
        .collect::<Vec<_>>();
    let party_defs = new_parties
        .iter()
        .map(|(tr, entry)| trainer(tr, entry, &species, &personal, "TRAINER"))
        .collect::<Result<Vec<_>>>()?
        .join("");
    write("../../src/data/trainers.party", base_parties + &party_defs)?;

    let new_partners = (trainers.table)
        .iter()
        .filter_map(|tr| Some((tr, entries.partners.iter().find(|t| t.id == tr.tr_id)?)))
        .collect::<Vec<_>>();
    let partner_defs = new_partners
        .iter()
        .map(|(tr, entry)| trainer(tr, entry, &species, &personal, "PARTNER"))
        .collect::<Result<Vec<_>>>()?
        .join("");
    write(
        "../../src/data/battle_partners.party",
        base_partners + &partner_defs,
    )?;

    Ok(())
}
