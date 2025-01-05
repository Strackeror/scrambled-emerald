use super::Edit;

use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde_json::Value;
use streaming_iterator::StreamingIterator;
use tree_sitter::{InputEdit, Node, Point, Query, QueryCursor, Range};

use std::collections::BTreeMap;
use std::fmt::Display;
use std::sync::OnceLock;

const ENTRY_QUERY_STR: &str = "
    (initializer_pair
       designator: (subscript_designator (identifier) @id)
       value: (initializer_list) @entry
    )
    
    (assignment_expression
        left: (subscript_expression indices: (
            subscript_argument_list (identifier) @id))
        right: (initializer_list) @entry
    )
    (init_declarator
        declarator: (structured_binding_declarator (identifier) @id)
        value: (initializer_list) @entry
    )
    (init_declarator
        declarator: (array_declarator size: (identifier) @id)
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
pub trait GetFieldExt {
    fn get_field<T: DeserializeOwned>(&self, name: &str) -> Result<T>;
}

impl GetFieldExt for BTreeMap<String, Value> {
    fn get_field<T: DeserializeOwned>(&self, name: &str) -> Result<T> {
        let field = self.get(name).context("getting field")?;
        Ok(serde_json::from_value(field.clone())?)
    }
}

pub(crate) fn find_entries<'a>(
    root_node: Node<'a>,
    text: &[u8],
) -> Result<Vec<(String, Node<'a>)>> {
    let query = ENTRY_QUERY.get_or_init(|| {
        tree_sitter::Query::new(&tree_sitter_cpp::LANGUAGE.into(), ENTRY_QUERY_STR).unwrap()
    });

    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(&query, root_node, text);
    matches
        .map_deref(|match_| {
            Ok((
                match_.captures[0].node.utf8_text(text)?.to_string(),
                match_.captures[1].node,
            ))
        })
        .collect()
}

// pub(crate) fn find_entry<'a>(root_node: Node<'a>, text: &[u8], move_id: &str) -> Result<Node<'a>> {
//     let query = ENTRY_QUERY.get_or_init(|| {
//         tree_sitter::Query::new(&tree_sitter_cpp::LANGUAGE.into(), ENTRY_QUERY_STR).unwrap()
//     });
//     let move_enum_name = format!("MOVE_{}", move_id.to_case(convert_case::Case::UpperSnake));

//     let mut cursor = QueryCursor::new();
//     let mut matches = cursor.matches(&query, root_node, text);
//     let match_ = matches
//         .find(|it| it.captures[0].node.utf8_text(text) == Ok(&move_enum_name))
//         .with_context(|| format!("find entry: {move_enum_name}"))?;

//     Ok(match_.captures[1].node)
// }

fn get_field<'a>(node: Node<'a>, text: &[u8], field_name: &str) -> Option<(Node<'a>, Node<'a>)> {
    let query = FIELD_QUERY.get_or_init(|| {
        tree_sitter::Query::new(&tree_sitter_cpp::LANGUAGE.into(), FIELD_QUERY_STR).unwrap()
    });
    QueryCursor::new()
        .matches(&query, node.clone(), text)
        .map_deref(|matche| (matche.captures[0].node, matche.captures[1].node))
        .find(|(field, _value)| field.utf8_text(text) == Ok(field_name))
}

pub fn get_field_value<'a>(node: Node, text: &'a [u8], field_name: &str) -> Option<&'a str> {
    let (_, value) = get_field(node, text, field_name)?;
    value.utf8_text(text).ok()
}

pub(crate) fn edit_field<T: Display>(
    node: Node,
    text: &[u8],
    field_name: &str,
    field_value: T,
) -> Result<Edit> {
    match get_field(node, text, field_name) {
        Some(field) => Ok(Edit {
            range: field.1.range(),
            replace: field_value.to_string(),
        }),
        None => {
            let end_token = node.child(node.child_count() - 1).context("last child")?;
            let range = Range {
                start_byte: end_token.start_byte(),
                end_byte: end_token.start_byte(),
                start_point: end_token.start_position(),
                end_point: end_token.start_position(),
            };
            Ok(Edit {
                range,
                replace: format!("    .{field_name} = {field_value},\n    "),
            })
        }
    }
}

pub fn replace_range(target: &mut Vec<u8>, range: Range, replace: &str) -> Result<InputEdit> {
    target.splice(range.start_byte..range.end_byte, replace.bytes());

    let lines = replace.split('\n');
    let line_count = lines.clone().count();
    let last_line_len = lines.last().unwrap_or("").bytes().len();
    let new_end_pos = Point {
        row: range.start_point.row + line_count - 1,
        column: match line_count {
            2.. => last_line_len,
            _ => range.start_point.column + last_line_len,
        },
    };

    let input_edit = InputEdit {
        start_byte: range.start_byte,
        old_end_byte: range.end_byte,
        new_end_byte: range.start_byte + replace.bytes().len(),
        start_position: range.start_point,
        old_end_position: range.end_point,
        new_end_position: new_end_pos,
    };
    Ok(input_edit)
}
