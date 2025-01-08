use std::{collections::BTreeMap, env};

use anyhow::{bail, Context, Result};
use moves::moves;
use serde::Deserialize;
use serde_json::Value;
use species::species;
use trainers::trainers;
use tree_sitter::{Node, Range, Tree};
use tree_utils::replace_range;

mod moves;
mod species;
mod trainers;
mod tree_utils;

fn main() -> Result<()> {
    match env::args().nth(1).context("arg")?.as_str() {
        "moves" => moves()?,
        "species" => species()?,
        "trainers" => trainers()?,
        other => bail!("unexpected {other}"),
    };
    Ok(())
}

struct Edit {
    range: Range,
    replace: String,
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
struct EvoData {
    level: u8,
    condition: String,
    parameter: Value,
    species: String,
    form: usize,
}

#[derive(Debug, Clone, Deserialize)]
struct LevelUpMove {
    r#move: String,
    level: u8,
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
    evo_data: Vec<EvoData>,

    #[serde(default)]
    tm_moves: Vec<String>,
    levelup_moves: Vec<LevelUpMove>,

    #[serde(flatten)]
    _fields: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize)]
struct PersonalArray {
    entry: Vec<Personal>,
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
