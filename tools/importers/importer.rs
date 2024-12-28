use std::collections::BTreeMap;
use std::fs::{read, read_to_string};
use std::sync::OnceLock;
use std::{env, fs};

use anyhow::{Context, Result};
use convert_case::{Case, Casing};
use serde::Deserialize;
use serde_json::Value;
use tree_sitter::{Query, Range};
use tree_utils::replace_range;

mod moves;
mod tree_utils;

const ENTRY_QUERY_STR: &str = "
    (initializer_pair
       designator: (subscript_designator (identifier) @id)
       value: (initializer_list) @entry
    )
";

static ENTRY_QUERY: OnceLock<Query> = OnceLock::new();

const FIELD_QUERY_STR: &str = "
    (initializer_pair
        designator: (field_designator (field_identifier) @field)
        value: (_) @value
    )
";
static FIELD_QUERY: OnceLock<Query> = OnceLock::new();

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

fn main() -> Result<()> {
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

    let move_info = env::args().nth(1).context("arg 1")?;
    let mut move_info_text = read(move_info)?;
    let mut tree = parser
        .parse(&move_info_text, None)
        .context("parsing tree")?;
    let mut edits = vec![];
    let mut fields: BTreeMap<String, Vec<String>> = Default::default();
    let entries = tree_utils::find_entries(tree.root_node(), &move_info_text)?;
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
    println!("applying {} edits", edits.len());
    edits.sort_by_key(|edit| (edit.range.start_byte, edit.range.end_byte));
    for edit in edits.iter().rev() {
        tree.edit(&replace_range(
            &mut move_info_text,
            edit.range,
            &edit.replace,
        )?);
    }

    println!("fields: {:#?}", fields.keys().collect::<Vec<_>>());

    let target = env::args()
        .nth(2)
        .unwrap_or_else(|| "target/output.h".into());
    fs::write(target, &move_info_text)?;
    Ok(())
}
