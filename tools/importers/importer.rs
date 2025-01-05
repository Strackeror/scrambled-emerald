use std::collections::{BTreeMap, HashMap};
use std::fs::{read, read_to_string, write};
use std::{env, fs};

use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use serde::Deserialize;
use serde_json::Value;
use species::handle_species;
use tree_sitter::{Node, Range, Tree};
use tree_utils::{find_entries, replace_range};

mod moves;
mod species;
mod tree_utils;

fn main() -> Result<()> {
    if env::args().nth(1).unwrap_or_default() == "moves" {
        moves()?;
    }
    species()?;
    Ok(())
}

struct Edit {
    range: Range,
    replace: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Move {
    move_id: String,
    #[serde(flatten)]
    fields: BTreeMap<String, Value>,
}

impl Move {
    fn field_str(&self, field: &str) -> Result<String> {
        if let Some(bool) = self.fields.get(field).and_then(|field| field.as_bool()) {
            return Ok(bool.to_string().to_uppercase());
        }
        Ok(format!(
            "{}",
            self.fields.get(field).context("getting field")?
        ))
    }
}

#[derive(Deserialize)]
struct WazaArray {
    table: Vec<Move>,
}

fn moves() -> Result<()> {
    const MOVE_INFO_PATH: &str = "../../src/data/moves_info.h";
    let modded: WazaArray = serde_json::from_str(&read_to_string("resources/waza_array.json")?)?;
    let vanilla: WazaArray =
        serde_json::from_str(&read_to_string("resources/waza_array_vanilla.json")?)?;
    let move_names: Vec<String> =
        serde_json::from_str(&read_to_string("resources/move_names.json")?)?;
    let move_descs: Vec<String> =
        serde_json::from_str(&read_to_string("resources/move_descs.json")?)?;

    let language = tree_sitter_cpp::LANGUAGE.into();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&language)?;

    let mut move_info_text = read(MOVE_INFO_PATH)?;
    let mut tree = parser
        .parse(&move_info_text, None)
        .context("parsing tree")?;
    let mut edits = vec![];
    let mut fields: BTreeMap<String, Vec<String>> = Default::default();
    let entries = tree_utils::find_entries(tree.root_node(), &move_info_text)?
        .into_iter()
        .collect::<HashMap<_, _>>();
    for (((modded, vanilla), name), desc) in modded
        .table
        .into_iter()
        .zip(vanilla.table.into_iter())
        .zip(move_names)
        .zip(move_descs)
    {
        if modded.move_id != vanilla.move_id {
            panic!("desynced move_id {modded:?} {vanilla:?}")
        }
        let diff_keys = (modded.fields)
            .keys()
            .filter(|key| modded.fields.get(*key) != vanilla.fields.get(*key))
            .map(|key| key.as_str())
            .collect::<Vec<_>>();

        for key in diff_keys.iter() {
            fields
                .entry(key.to_string())
                .or_default()
                .push(modded.move_id.clone());
        }
        let case = Case::ScreamingSnake;
        let Some(&move_node) = entries.get(&format!("MOVE_{}", modded.move_id.to_case(case)))
        else {
            if !diff_keys.is_empty() {
                println!("ERROR: Missing move: {}", modded.move_id);
            }
            continue;
        };
        edits.append(&mut moves::handle_changes(
            diff_keys,
            &modded,
            move_node,
            &mut move_info_text,
            (&name, &desc),
        )?);
    }
    apply_edits(&mut move_info_text, &mut tree, edits)?;

    println!("fields: {:#?}", fields.keys().collect::<Vec<_>>());
    fs::write(MOVE_INFO_PATH, &move_info_text)?;
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Species {
    species: String,
    form: usize,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Stats {
    HP: u16,
    ATK: u16,
    DEF: u16,
    SPA: u16,
    SPD: u16,
    SPE: u16,
}

#[derive(Debug, Deserialize)]
struct Personal {
    species: Species,
    is_present: bool,

    type_1: String,
    type_2: String,

    ability_1: String,
    ability_2: String,
    ability_hidden: String,

    base_stats: Stats,

    #[serde(flatten)]
    _fields: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize)]
struct PersonalArray {
    entry: Vec<Personal>,
}

const BLACKLIST: &[&str] = &[
    "Vivillon", "Flabebe", "Floette", "Florges", "Minior", "Alcremie",
];
const RENAME: &[(&str, &str)] = &[
    ("Porygon2", "SPECIES_PORYGON2"),
    ("Jangmoo", "SPECIES_JANGMO_O"),
    ("Hakamoo", "SPECIES_HAKAMO_O"),
    ("Kommoo", "SPECIES_KOMMO_O"),
];

fn species() -> Result<()> {
    let species_files = (1..=9)
        .map(|n| format!("../../src/data/pokemon/species_info/gen_{n}_families.h"))
        .map(|path| Ok((path.clone(), read(&path)?)))
        .collect::<Result<Vec<_>>>()?;

    let language = tree_sitter_cpp::LANGUAGE.into();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&language)?;

    let mut trees = species_files
        .into_iter()
        .map(|(path, text)| Some((parser.parse(&text, None)?, text, path)))
        .collect::<Option<Vec<_>>>()
        .context("Failed to parse")?;
    //_dump_nodes(trees[1].1.root_node());

    let entries = trees
        .iter()
        .map(|(tree, text, _)| Ok((text, tree, find_entries(tree.root_node(), text)?)))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .enumerate()
        .flat_map(|(index, (text, tree, nodes))| {
            nodes
                .into_iter()
                .map(|(id, node)| (id, (index, text, tree, node)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let data_array: PersonalArray =
        serde_json::from_slice(&read("resources/personal_array.json")?)?;
    let mut edits: Vec<Vec<Edit>> = trees.iter().map(|_| vec![]).collect();
    for personal in data_array.entry {
        if personal.is_present == false {
            continue;
        }

        if BLACKLIST.contains(&personal.species.species.as_str()) {
            println!("Blacklisted species {:?}", personal.species);
            continue;
        }
        let species_name = match RENAME.iter().find(|(a, _)| *a == &personal.species.species) {
            Some((_, rename)) => rename.to_string(),
            None => {
                let cased = personal.species.species.to_case(Case::ScreamingSnake);
                format!("SPECIES_{cased}")
            }
        };

        let mut matching = entries
            .iter()
            .filter(|(id, _)| id == &species_name || id.starts_with(&(species_name.clone() + "_")));
        let Some((_id, (index, text, _tree, node))) = matching.nth(personal.species.form) else {
            println!("Could not find species {:?}", personal.species);
            continue;
        };

        let species_edits = handle_species(*node, text, &personal)?;
        edits[*index].extend(species_edits);
    }
    for ((tree, text, path), edits) in trees.iter_mut().zip(edits) {
        apply_edits(text, tree, edits)?;
        write(path, text)?;
    }
    Ok(())
}

fn apply_edits(text: &mut Vec<u8>, tree: &mut Tree, mut edits: Vec<Edit>) -> Result<()> {
    println!("applying {} edits", edits.len());
    edits.sort_by_key(|edit| (edit.range.start_byte, edit.range.end_byte));
    for edit in edits.iter().rev() {
        tree.edit(&replace_range(text, edit.range, &edit.replace)?);
    }
    Ok(())
}

fn _dump_nodes(node: Node) {
    let mut cursor = node.walk();
    let mut spacing = String::new();
    'cursor: loop {
        println!(
            "{spacing}{}: {}-{}",
            cursor.node().grammar_name(),
            cursor.node().start_position(),
            cursor.node().end_position()
        );
        if cursor.goto_first_child() {
            spacing += "  ";
            continue;
        }
        if cursor.goto_next_sibling() {
            continue;
        }

        loop {
            if cursor.goto_parent() {
                let len = spacing.len();
                spacing.replace_range(len - 2..len, "");
                if cursor.goto_next_sibling() {
                    continue 'cursor;
                }
                continue;
            }
            break;
        }
        break;
    }
}
