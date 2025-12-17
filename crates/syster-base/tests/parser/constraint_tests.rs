use pest::Parser;
use syster::parser::{SysMLParser, sysml::Rule};

#[test]
fn test_assert_constraint_with_expression() {
    let source = r#"
        package Test {
            item def Satellite {
                assert constraint { mass < 1000.0 }
            }
        }
    "#;
    let result = SysMLParser::parse(Rule::file, source);
    assert!(result.is_ok(), "Parse failed: {:?}", result.err());
}

#[test]
fn test_assert_constraint_empty() {
    let source = r#"
        package Test {
            item def Satellite {
                assert constraint { true }
            }
        }
    "#;
    assert!(SysMLParser::parse(Rule::file, source).is_ok());
}

#[test]
#[ignore = "Items.sysml has complex expressions not yet supported (arrow invocations, etc.)"]
fn test_stdlib_items_file() {
    let source = std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("sysml.library")
            .join("Systems Library")
            .join("Items.sysml"),
    )
    .unwrap();

    let result = SysMLParser::parse(Rule::file, &source);
    assert!(result.is_ok(), "Parse failed: {:?}", result.err());
}

#[test]
#[ignore]
fn test_stdlib_calculations_file() {
    let source = std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("sysml.library")
            .join("Systems Library")
            .join("Calculations.sysml"),
    )
    .unwrap();

    let result = SysMLParser::parse(Rule::file, &source);
    assert!(
        result.is_ok(),
        "Calculations.sysml parse failed: {:?}",
        result.err()
    );
}
