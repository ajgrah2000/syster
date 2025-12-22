use super::types::{Classifier, Feature};
use super::utils::{extract_direction, extract_flags, find_name, to_classifier_kind, to_span};
use crate::parser::kerml::Rule;
use from_pest::{ConversionError, Void};
use pest::iterators::Pair;

/// Parse a classifier from a pest pair
pub fn parse_classifier(pair: Pair<Rule>) -> Result<Classifier, ConversionError<Void>> {
    let kind = to_classifier_kind(pair.as_rule())?;
    let span = Some(to_span(pair.as_span()));
    let pairs: Vec<_> = pair.into_inner().collect();

    Ok(Classifier {
        kind,
        is_abstract: pairs.iter().any(|p| p.as_rule() == Rule::abstract_marker),
        name: find_name(pairs.into_iter()),
        body: Vec::new(),
        span,
    })
}

/// Parse a feature from a pest pair
pub fn parse_feature(pair: Pair<Rule>) -> Feature {
    let span = Some(to_span(pair.as_span()));
    let pairs: Vec<_> = pair.into_inner().collect();
    let (is_readonly, is_derived) = extract_flags(&pairs);

    Feature {
        name: find_name(pairs.iter().cloned()),
        direction: extract_direction(&pairs),
        is_readonly,
        is_derived,
        body: Vec::new(),
        span,
    }
}
