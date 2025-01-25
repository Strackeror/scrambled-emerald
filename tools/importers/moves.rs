use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::fs::{read, read_to_string, write};

use anyhow::{bail, Context as _, Result};
use convert_case::{Case, Casing as _};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;
use tree_sitter::Node;

use crate::tree_utils::{edit_field, find_entries, get_field_value, GetFieldExt};
use crate::{apply_edits, Edit};

#[derive(Debug, Clone, Deserialize)]
pub struct Move {
    pub move_id: String,
    #[serde(flatten)]
    pub fields: BTreeMap<String, Value>,
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
pub struct WazaArray {
    pub table: Vec<Move>,
}

fn fix_table(mut array: WazaArray, map: &HashMap<String, usize>) -> WazaArray {
    array.table.sort_by_cached_key(|m| map.get(&m.move_id));
    array.table.dedup_by_key(|m| map.get(&m.move_id));
    array
}

pub fn moves() -> Result<()> {
    const MOVE_INFO_PATH: &str = "../../src/data/moves_info.h";
    let modded: WazaArray = serde_json::from_slice(&read("resources/waza_array.json")?)?;
    let vanilla: WazaArray = serde_json::from_slice(&read("resources/waza_array.previous.json")?)?;
    let move_names: Vec<String> = serde_json::from_slice(&read("resources/move_names.json")?)?;
    let move_descs: Vec<String> = serde_json::from_slice(&read("resources/move_descs.json")?)?;
    let move_map: HashMap<String, usize> = serde_json::from_slice(&read("resources/vanilla_move_names.json")?)?;

    let modded = fix_table(modded, &move_map);
    let vanilla = fix_table(vanilla, &move_map);


    let language = tree_sitter_cpp::LANGUAGE.into();
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&language)?;

    let mut move_info_text = read(MOVE_INFO_PATH)?;
    let mut tree = parser
        .parse(&move_info_text, None)
        .context("parsing tree")?;
    let mut edits = vec![];
    let mut fields: BTreeMap<String, Vec<String>> = Default::default();
    let entries = find_entries(tree.root_node(), &move_info_text)?
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
            panic!("desynced move_id {modded:?} \n {vanilla:?}")
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
        edits.append(&mut handle_changes(
            diff_keys,
            &modded,
            move_node,
            &mut move_info_text,
            (&name, &desc),
        )?);
    }
    apply_edits(&mut move_info_text, &mut tree, edits)?;

    println!("fields: {:#?}", fields.keys().collect::<Vec<_>>());
    write(MOVE_INFO_PATH, &move_info_text)?;
    Ok(())
}

#[derive(Clone, Copy)]
struct Context<'a> {
    node: Node<'a>,
    text: &'a [u8],

    diff_keys: &'a [&'a str],
    entry: &'a Move,
}

pub(crate) fn handle_changes(
    diff_keys: Vec<&str>,
    modded: &Move,
    move_node: Node,
    text: &mut Vec<u8>,
    (name, desc): (&str, &str),
) -> Result<Vec<Edit>> {
    //println!("handling {}: {:?}", modded.move_id, diff_keys);

    let context = Context {
        node: move_node,
        text,
        diff_keys: &diff_keys,
        entry: modded,
    };

    let field_name = format!("COMPOUND_STRING({name:?})");
    let desc_name = format!("COMPOUND_STRING({desc:?})");
    let mut edits = vec![
        edit_field(move_node, &text, "name", field_name)?,
        edit_field(move_node, &text, "description", desc_name)?,
    ];
    edits.extend(handle_simple_changes(context)?);
    edits.extend(handle_inverted_flags(context)?);
    edits.extend(handle_recoil(context)?);
    edits.extend(handle_hit_count(context)?);
    edits.extend(handle_effects(context)?);
    edits.extend(handle_accuracy(context)?);
    edits.extend(handle_power(context)?);
    edits.extend(handle_type(context)?);
    edits.extend(handle_category(context)?);
    edits.extend(handle_target(context)?);
    edits.extend(handle_crit_stage(context)?);
    edits.extend(handle_move_category(context)?);
    Ok(edits)
}

fn handle_simple_changes(context: Context) -> Result<Vec<Edit>> {
    const SIMPLE_FIELDS: &[(&str, &str)] = &[
        ("pp", "pp"),
        ("priority", "priority"),
        ("drops_your_stats_flag", "punchingMove"),
        ("flag_defrost", "thawsUser"),
        ("flag_cant_use_twice", "cantUseTwice"),
        ("flag_no_multi_hit", "parentalBondBanned"),
        ("flag_gravity", "gravityBanned"),
        ("sound_flag", "soundMove"),
        ("rapid_fire_flag", "healingMove"),
        ("sharpness_flag", "slicingMove"),
        ("mega_launcher_flag", "pulseMove"),
        ("bulletproof_flag", "ballisticMove"),
        ("wind_flag", "windMove"),
        ("contact", "makesContact"),
    ];

    let Context {
        node,
        text,
        diff_keys,
        entry,
    } = context;
    SIMPLE_FIELDS
        .iter()
        .filter(|(key, _field)| diff_keys.contains(key))
        .map(|(key, field)| edit_field(node, text, field, &entry.field_str(key)?))
        .collect()
}

fn handle_inverted_flags(context: Context) -> Result<Vec<Edit>> {
    const SIMPLE_FIELDS: &[(&str, &str)] = &[("flag_metronome", "metronomeBanned")];
    SIMPLE_FIELDS
        .iter()
        .filter_map(|(key, field)| {
            mapped_field(context, *key, *field, |value: bool| {
                (!value).to_string().to_uppercase()
            })
            .transpose()
        })
        .collect()
}

fn mapped_field<T: DeserializeOwned, U: Display>(
    context: Context,
    field_name: &str,
    c_field: &str,
    value: impl Fn(T) -> U,
) -> Result<Option<Edit>> {
    if !context.diff_keys.contains(&field_name) {
        return Ok(None);
    }

    let field = context.entry.fields.get_field(field_name)?;
    let value = value(field);
    Ok(Some(edit_field(
        context.node,
        context.text,
        c_field,
        value,
    )?))
}

fn handle_power(context: Context) -> Result<Option<Edit>> {
    mapped_field(context, "power", "power", |power: i8| match power {
        power @ 0..=127 => power as i16,
        power @ -128..0 => 256 + power as i16,
    })
}

fn handle_accuracy(context: Context) -> Result<Option<Edit>> {
    mapped_field(
        context,
        "accuracy",
        "accuracy",
        |accuracy: i32| match accuracy {
            101.. => 0,
            acc => acc,
        },
    )
}

fn handle_crit_stage(context: Context) -> Result<Option<Edit>> {
    mapped_field(
        context,
        "crit_stage",
        "criticalHitStage",
        |crit: i16| match crit {
            0 => 0,
            1 => 1,
            2 => 2,
            6 => 3,
            _ => {
                println!("unhandled crit_stage {}", context.entry.move_id);
                0
            }
        },
    )
}

fn handle_type(context: Context) -> Result<Option<Edit>> {
    mapped_field(
        context,
        "type",
        "type",
        |type_: String| format! {"TYPE_{}", type_.to_uppercase()},
    )
}

fn handle_category(context: Context) -> Result<Option<Edit>> {
    mapped_field(
        context,
        "category",
        "category",
        |cat: String| format! {"DAMAGE_CATEGORY_{}", cat.to_uppercase()},
    )
}

fn handle_recoil(context: Context) -> Result<Vec<Edit>> {
    if !context.diff_keys.contains(&"recoil") {
        return Ok(vec![]);
    };

    let Context { node, text, .. } = context;
    let recoil: i16 = context.entry.fields.get_field("recoil")?;
    let quality: String = context.entry.fields.get_field("quality")?;
    let edits = match quality.as_str() {
        "DAMAGEHeal" => vec![
            edit_field(node, text, "effect", "EFFECT_ABSORB")?,
            edit_field(node, text, "argument", recoil)?,
        ],
        _ => vec![edit_field(node, text, "recoil", -recoil)?],
    };
    Ok(edits)
}

fn handle_hit_count(context: Context) -> Result<Vec<Edit>> {
    if !context.diff_keys.contains(&"hit_max") && !context.diff_keys.contains(&"hit_min") {
        return Ok(vec![]);
    }
    let min: u16 = context.entry.fields.get_field("hit_min")?;
    let max: u16 = context.entry.fields.get_field("hit_max")?;
    let Context { node, text, .. } = context;
    let edits = match (min, max) {
        (2, 5) => vec![edit_field(node, text, "effect", "EFFECT_MULTI_HIT")?],
        (min, max) if min == max => vec![edit_field(node, text, "strikeCount", min)?],
        _ => bail!("invalid hit count"),
    };
    Ok(edits)
}

fn handle_target(context: Context) -> Result<Option<Edit>> {
    if !context.diff_keys.contains(&"raw_target") {
        return Ok(None);
    }

    let target = match context
        .entry
        .fields
        .get_field::<String>("raw_target")?
        .as_str()
    {
        "One" => "MOVE_TARGET_SELECTED",
        "All"  => "MOVE_TARGET_FOES_AND_ALLY",
        "AllFoes" => "MOVE_TARGET_BOTH",
        "Self" => "MOVE_TARGET_USER",
        other => bail!("Unhandled raw target: {other}"),
    };
    Ok(Some(edit_field(
        context.node,
        context.text,
        "target",
        target,
    )?))
}

fn handle_move_category(context: Context) -> Result<Option<Edit>> {
    if !context.diff_keys.contains(&"move_category") {
        return Ok(None);
    }

    let Context { node, text, .. } = context;
    match context
        .entry
        .fields
        .get_field::<String>("move_category")?
        .as_str()
    {
        "AlwaysCritical" => Ok(Some(edit_field(node, text, "alwaysCriticalHit", "TRUE")?)),
        "StatusSelf" => {
            println!(
                "StatusSelf {} {:?}",
                context.entry.move_id, context.diff_keys
            );
            Ok(None)
        }
        "Attack" => {
            println!("move_category: Attack: {}", context.entry.move_id);
            Ok(None)
        }
        move_cat => bail!(
            "unhandled move_category {move_cat}, {}",
            context.entry.move_id
        ),
    }
}

#[derive(Deserialize)]
struct InflictStatus {
    status: String,
    chance: u16,
    turn1: u16,
    turn2: u16,
    turn3: u16,
}

#[derive(Deserialize)]
struct StatAmps {
    fstat1: String,
    fstat1_stage: i16,
    fstat1_percent: u16,
    fstat2: String,
    fstat2_stage: i16,
    fstat2_percent: u16,
    fstat3: String,
    fstat3_stage: i16,
    fstat3_percent: u16,
}

fn move_effect_name(stat: &str, count: i16) -> Result<String> {
    let stat = match stat {
        "Attack" => "ATK",
        "Defense" => "DEF",
        "Speed" => "SPD",
        "SpecialAttack" => "SP_ATK",
        "SpecialDefense" => "SP_DEF",
        "Accuracy" => "ACC",
        "Evasion" => "EVS",
        "All" => "ALL",
        _ => bail!("Unexpected stat {stat}"),
    };

    let str = match count {
        count @ 1.. => format!("MOVE_EFFECT_{stat}_PLUS_{count}"),
        count @ ..=-1 => format!("MOVE_EFFECT_{stat}_MINUS_{}", count.abs()),
        0 => bail!("unexpected zero"),
    };
    Ok(str)
}

fn handle_effects(context: Context) -> Result<Vec<Edit>> {
    const KEYS: &[&str] = &[
        "quality",
        "flinch",
        "flag_rechargeg",
        "inflict_status",
        "stat_amps",
    ];

    if !KEYS.iter().any(|key| context.diff_keys.contains(key)) {
        return Ok(vec![]);
    }

    let fields = &context.entry.fields;
    let quality = fields.get_field::<String>("quality")?;
    let mut effects: Vec<Vec<(String, String)>> = vec![];
    if fields.get_field("flag_rechargeg")? {
        effects.push(vec![
            ("moveEffect".into(), "MOVE_EFFECT_RECHARGE".into()),
            ("self".into(), "TRUE".into()),
        ]);
    }

    let flinch = fields.get_field::<u16>("flinch")?;
    if flinch > 0 {
        effects.push(vec![
            ("moveEffect".into(), "MOVE_EFFECT_FLINCH".into()),
            ("chance".into(), format!("{flinch}")),
        ]);
    }

    let InflictStatus {
        status,
        chance,
        turn1,
        turn2,
        turn3,
    } = fields.get_field("inflict_status")?;
    if status != "NONE" {
        let status = match status.as_str() {
            "Poison" => "MOVE_EFFECT_POISON",
            "Burn" => "MOVE_EFFECT_BURN",
            "Paralysis" => "MOVE_EFFECT_PARALYSIS",
            "Toxic" => "MOVE_EFFECT_TOXIC",
            "Freeze" => "MOVE_EFFECT_FREEZE",
            "Sleep" => &format!("MOVE_EFFECT_SLEEP /* {turn1} {turn2} {turn3} */",),
            "Confusion" => "MOVE_EFFECT_CONFUSION",
            "Curse" => "MOVE_EFFECT_CURSE",
            "Bind" => "MOVE_EFFECT_WRAP",
            _ => {
                println!(
                    "Unexpected status {status} in move {}, {:?}",
                    context.entry.move_id, context.diff_keys
                );
                return Ok(vec![]);
            }
        };
        let mut fields = vec![("moveEffect".into(), status.into())];
        if chance != 0 {
            fields.push(("chance".into(), chance.to_string()));
        }
        effects.push(fields);
    }

    let StatAmps {
        fstat1,
        fstat1_stage,
        fstat1_percent,
        fstat2,
        fstat2_stage,
        fstat2_percent,
        fstat3,
        fstat3_stage,
        fstat3_percent,
    } = fields.get_field("stat_amps")?;

    for (stat, stage, chance) in [
        (fstat1, fstat1_stage, fstat1_percent),
        (fstat2, fstat2_stage, fstat2_percent),
        (fstat3, fstat3_stage, fstat3_percent),
    ] {
        if stat == "None" {
            continue;
        }
        let mut fields = vec![("moveEffect".into(), move_effect_name(&stat, stage)?)];
        if chance != 0 {
            fields.push(("chance".into(), chance.to_string()));
        }
        if quality == "DAMAGEUSERStat" {
            fields.push(("self".into(), "TRUE".into()));
        }
        effects.push(fields);
    }

    let mut edits = vec![];
    match quality.as_str() {
        "OnlyDMG"
        | "DAMAGEInflictsStatus"
        | "DAMAGETARGETStat"
        | "DAMAGEUSERStat"
        | "DAMAGEHeal" => {
            if get_field_value(context.node, context.text, "effect") != Some("EFFECT_HIT") {
                println!(
                    "Manual changes needed for {}: {:?}",
                    context.entry.move_id, context.diff_keys
                )
            }
        }

        "OHKO" => println!("OHKO MOVE {}", context.entry.move_id),
        "STATUSStatChange" | "STATUSInflictsStatus" | "STATUSStatChangeANDStatus" => edits.push(
            edit_field(context.node, context.text, "effect", "EFFECT_DO_NOTHING")?,
        ),

        _ => {
            println!(
                "Unhandled move {} quality {}",
                context.entry.move_id, quality
            );
            return Ok(vec![]);
        }
    };

    if !effects.is_empty() {
        let mut add_str = "ADDITIONAL_EFFECTS(".to_string();
        for effect in effects {
            add_str.push_str("{");
            for (field, value) in effect {
                add_str.push_str(&format!(".{field} = {value},"));
            }
            add_str.push_str("},");
        }
        add_str.push_str(")");
        edits.push(edit_field(
            context.node,
            context.text,
            "additionalEffects",
            add_str,
        )?);
    }
    Ok(edits)
}
