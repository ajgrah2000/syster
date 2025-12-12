#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::*;
use crate::parser::sysml::{Rule, SysMLParser};
use from_pest::FromPest;
use pest::Parser;

#[test]
fn test_package_from_pest() {
    let source = "package MyPackage;";
    let mut pairs = SysMLParser::parse(Rule::package_declaration, source).unwrap();

    let package = Package::from_pest(&mut pairs).unwrap();

    assert_eq!(package.name, Some("MyPackage".to_string()));
    assert_eq!(package.elements.len(), 0);
}

#[test]
fn test_part_definition_from_pest() {
    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::part_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Part);
    assert_eq!(definition.name, Some("Vehicle".to_string()));
    assert_eq!(definition.body.len(), 0);
}

#[test]
fn test_action_definition_from_pest() {
    let source = "action def Drive;";
    let mut pairs = SysMLParser::parse(Rule::action_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Action);
    assert_eq!(definition.name, Some("Drive".to_string()));
    assert_eq!(definition.body.len(), 0);
}

#[test]
fn test_requirement_definition_from_pest() {
    let source = "requirement def SafetyReq;";
    let mut pairs = SysMLParser::parse(Rule::requirement_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Requirement);
    assert_eq!(definition.name, Some("SafetyReq".to_string()));
    assert_eq!(definition.body.len(), 0);
}

impl AstNode for Package {
    fn node_type(&self) -> &'static str {
        "Package"
    }

    fn has_children(&self) -> bool {
        !self.elements.is_empty()
    }
}

impl Named for Package {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl AstNode for Definition {
    fn node_type(&self) -> &'static str {
        "Definition"
    }
}

impl Named for Definition {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[test]
fn test_ast_node_trait() {
    let pkg = Package {
        name: Some("TestPackage".to_string()),
        elements: vec![],
    };

    assert_eq!(pkg.node_type(), "Package");
    assert_eq!(pkg.name(), Some("TestPackage"));
    assert!(!pkg.has_children());
}

#[test]
fn test_definition_traits() {
    let def = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        body: vec![],
    };

    assert_eq!(def.node_type(), "Definition");
    assert_eq!(def.name(), Some("Vehicle"));
}

struct CountingVisitor {
    packages: usize,
    definitions: usize,
}

impl AstVisitor for CountingVisitor {
    fn visit_package(&mut self, _package: &Package) {
        self.packages += 1;
    }

    fn visit_definition(&mut self, _definition: &Definition) {
        self.definitions += 1;
    }
}

#[test]
fn test_visitor_pattern() {
    let file = SysMLFile {
        namespace: None,
        elements: vec![
            Element::Package(Package {
                name: Some("TestPkg".to_string()),
                elements: vec![],
            }),
            Element::Definition(Definition {
                kind: DefinitionKind::Part,
                name: Some("TestDef".to_string()),
                body: vec![],
            }),
        ],
    };

    let mut visitor = CountingVisitor {
        packages: 0,
        definitions: 0,
    };

    file.accept(&mut visitor);

    assert_eq!(visitor.packages, 1);
    assert_eq!(visitor.definitions, 1);
}
