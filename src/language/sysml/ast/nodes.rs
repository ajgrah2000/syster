use super::{enums::*, types::*};
use crate::parser::sysml::Rule;
use from_pest::{ConversionError, FromPest, Void};

impl<'pest> FromPest<'pest> for Package {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(
        pest: &mut pest::iterators::Pairs<'pest, Rule>,
    ) -> std::result::Result<Self, ConversionError<Void>> {
        let pair = pest.next().ok_or(ConversionError::NoMatch)?;
        if pair.as_rule() != Rule::package_declaration {
            return Err(ConversionError::NoMatch);
        }
        let name = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::identification)
            .map(|id| id.as_str().to_string());
        Ok(Package {
            name,
            elements: vec![],
        })
    }
}

impl<'pest> FromPest<'pest> for Definition {
    type Rule = Rule;
    type FatalError = Void;

    fn from_pest(
        pest: &mut pest::iterators::Pairs<'pest, Rule>,
    ) -> std::result::Result<Self, ConversionError<Void>> {
        let pair = pest.next().ok_or(ConversionError::NoMatch)?;
        let kind = match pair.as_rule() {
            Rule::part_definition => DefinitionKind::Part,
            Rule::action_definition => DefinitionKind::Action,
            Rule::requirement_definition => DefinitionKind::Requirement,
            Rule::port_definition => DefinitionKind::Port,
            Rule::item_definition => DefinitionKind::Item,
            Rule::attribute_definition => DefinitionKind::Attribute,
            _ => return Err(ConversionError::NoMatch),
        };
        let name = pair
            .clone()
            .into_inner()
            .find(|p| p.as_rule() == Rule::definition_declaration)
            .and_then(|decl| {
                decl.into_inner()
                    .find(|p| p.as_rule() == Rule::identification)
            })
            .map(|id| id.as_str().to_string());
        Ok(Definition {
            kind,
            name,
            body: vec![],
        })
    }
}
