use super::utils::{
    extract_flags, extract_relationships, find_name, is_body_rule, is_usage_rule, to_def_kind,
    to_span, to_usage_kind,
};
use super::{
    enums::{DefinitionMember, UsageMember},
    types::{Comment, Definition, Usage},
};
use crate::parser::sysml::Rule;
use from_pest::{ConversionError, Void};
use pest::iterators::Pair;

// ============================================================================
// Body parsing
// ============================================================================

/// Parse definition body members
pub fn parse_def_body(pair: &Pair<Rule>) -> Vec<DefinitionMember> {
    let mut members = Vec::new();
    extract_def_members(pair, &mut members);
    members
}

fn extract_def_members(pair: &Pair<Rule>, members: &mut Vec<DefinitionMember>) {
    if is_usage_rule(pair.as_rule()) {
        members.push(DefinitionMember::Usage(Box::new(parse_usage(pair.clone()))));
    } else {
        for inner in pair.clone().into_inner() {
            extract_def_members(&inner, members);
        }
    }
}

/// Parse usage body members
pub fn parse_usage_body(pair: &Pair<Rule>) -> Vec<UsageMember> {
    let mut members = Vec::new();
    extract_usage_members(pair, &mut members);
    members
}

fn extract_usage_members(pair: &Pair<Rule>, members: &mut Vec<UsageMember>) {
    match pair.as_rule() {
        Rule::documentation | Rule::block_comment => {
            members.push(UsageMember::Comment(Comment {
                content: pair.as_str().to_string(),
                span: Some(to_span(pair.as_span())),
            }));
        }
        _ => {
            for inner in pair.clone().into_inner() {
                extract_usage_members(&inner, members);
            }
        }
    }
}

// ============================================================================
// Main parsers
// ============================================================================

/// Parse a definition from a pest pair
pub fn parse_definition(pair: Pair<Rule>) -> Result<Definition, ConversionError<Void>> {
    let kind = to_def_kind(pair.as_rule())?;
    let span = Some(to_span(pair.as_span()));
    let pairs: Vec<_> = pair.clone().into_inner().collect();

    let body = pairs
        .iter()
        .find(|p| is_body_rule(p.as_rule()))
        .map(parse_def_body)
        .unwrap_or_default();

    Ok(Definition {
        kind,
        name: find_name(pairs.iter().cloned()),
        relationships: extract_relationships(&pair),
        body,
        span,
        is_abstract: false,  // TODO: extract from definition_prefix
        is_variation: false, // TODO: extract from definition_prefix
    })
}

/// Parse a usage from a pest pair
pub fn parse_usage(pair: Pair<Rule>) -> Usage {
    let kind = to_usage_kind(pair.as_rule()).unwrap();
    let span = Some(to_span(pair.as_span()));
    let pairs: Vec<_> = pair.clone().into_inner().collect();

    let body = pairs
        .iter()
        .find(|p| matches!(p.as_rule(), Rule::usage_body | Rule::requirement_body))
        .map(parse_usage_body)
        .unwrap_or_default();

    let (is_derived, is_readonly) = extract_flags(&pairs);

    Usage {
        kind,
        name: find_name(pairs.into_iter()),
        relationships: extract_relationships(&pair),
        body,
        span,
        is_derived,
        is_readonly,
    }
}
