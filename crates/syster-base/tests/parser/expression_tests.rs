#![allow(clippy::unwrap_used)]

use pest::Parser;
use syster::parser::{SysMLParser, sysml::Rule};

#[test]
fn test_chained_member_access() {
    let input = "fn.samples.domainValue";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse chained member access: {:?}",
        result.err()
    );
}

#[test]
fn test_instantiation_expression_with_args() {
    let input = "new SampledFunction(samples = values)";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse instantiation expression: {:?}",
        result.err()
    );
}

#[test]
fn test_instantiation_expression_positional() {
    let input = "new SamplePair(x, y)";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse instantiation with positional args: {:?}",
        result.err()
    );
}

#[test]
fn test_arrow_invocation_with_block() {
    let input = "list->select { in i; true }";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse arrow invocation with block: {:?}",
        result.err()
    );
}

#[test]
fn test_arrow_invocation_with_block_then_index() {
    let input = "list->select { in i; true }#(1)";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse arrow invocation with block followed by indexing: {:?}",
        result.err()
    );
}

#[test]
fn test_typed_parameter_in_lambda() {
    let input = "list->select { in i : Positive; domainValues#(i) <= value }";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse lambda with typed parameter: {:?}",
        result.err()
    );
}

#[test]
fn test_typed_parameter_in_lambda_then_index() {
    let input = "list->select { in i : Positive; domainValues#(i) <= value }#(1)";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse lambda with typed parameter followed by indexing: {:?}",
        result.err()
    );
}

#[test]
fn test_full_sampled_functions_expression() {
    let input =
        "(1..size(domainValues))->select { in i : Positive; domainValues#(i) <= value }#(1)";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse full SampledFunctions expression: {:?}",
        result.err()
    );
}

#[test]
fn test_nested_instantiation_in_collect() {
    let input = "domainValues->collect { in x; new SamplePair(x, calculation(x)) }";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse collect with nested instantiation: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_complex_initializer() {
    let input = "attribute index : Positive[0..1] = (1..size(domainValues))->select { in i : Positive; domainValues#(i) <= value }#(1);";
    let result = SysMLParser::parse(Rule::attribute_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse attribute with complex initializer: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_without_type() {
    let input = "attribute index = (1..size(domainValues))->select { in i : Positive; domainValues#(i) <= value }#(1);";
    let result = SysMLParser::parse(Rule::attribute_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse attribute without type: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_in_package_context() {
    let input =
        "package Test { attribute index = list->select { in i : Positive; vals#(i) <= v }#(1); }";
    let result = SysMLParser::parse(Rule::package, input);

    assert!(
        result.is_ok(),
        "Failed to parse attribute in package: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_simple_type() {
    let input = "attribute index : Positive[0..1];";
    let result = SysMLParser::parse(Rule::attribute_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse attribute with simple type: {:?}",
        result.err()
    );
}

#[test]
fn test_feature_value_with_lambda() {
    use pest::Parser;
    use syster::parser::{SysMLParser, sysml::Rule};

    let input =
        "= (1..size(domainValues))->select { in i : Positive; domainValues#(i) <= value }#(1)";
    let result = SysMLParser::parse(Rule::feature_value, input);

    assert!(
        result.is_ok(),
        "Failed to parse feature_value: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_minimal() {
    let input = "{ vals#(i) }";
    let result = SysMLParser::parse(Rule::calculation_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse minimal calculation body: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_with_parameter_binding() {
    let input = "{ in i; vals#(i) }";
    let result = SysMLParser::parse(Rule::calculation_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse calculation body with parameter binding: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_with_typed_parameter() {
    let input = "{ in i : Positive; vals#(i) }";
    let result = SysMLParser::parse(Rule::calculation_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse calculation body with typed parameter: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_with_parameter_declaration() {
    // This is the failing case from SampledFunctions.sysml line 53
    let input = "{ in fn : SampledFunction; }";
    let result = SysMLParser::parse(Rule::calculation_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse calculation body with parameter declaration: {:?}",
        result.err()
    );
}

#[test]
fn test_expression_body_with_parameter() {
    let input = "{ in i; vals#(i) }";
    let result = SysMLParser::parse(Rule::expression_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse expression body with parameter: {:?}",
        result.err()
    );
}

#[test]
fn test_parameter_binding_simple() {
    let input = "in i";
    let result = SysMLParser::parse(Rule::parameter_binding, input);

    assert!(
        result.is_ok(),
        "Failed to parse simple parameter binding: {:?}",
        result.err()
    );
}

#[test]
fn test_parameter_binding_typed() {
    let input = "in fn : SampledFunction";
    let result = SysMLParser::parse(Rule::parameter_binding, input);

    assert!(
        result.is_ok(),
        "Failed to parse typed parameter binding: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_with_param_and_return() {
    // Exact pattern from SampledFunctions.sysml Domain calc def
    let input = "{ in fn : SampledFunction; return : Anything[0..*] = fn.samples.domainValue; }";
    let result = SysMLParser::parse(Rule::calculation_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse calc body with param and return: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_def_domain() {
    // Full Domain calc def from SampledFunctions.sysml
    let input = r#"calc def Domain {
        in fn : SampledFunction;
        return : Anything[0..*] = fn.samples.domainValue;
    }"#;
    let result = SysMLParser::parse(Rule::calculation_definition, input);

    assert!(
        result.is_ok(),
        "Failed to parse Domain calc def: {:?}",
        result.err()
    );
}

#[test]
fn test_return_parameter_member() {
    let input = "return : Anything[0..*] = fn.samples.domainValue";
    let result = SysMLParser::parse(Rule::return_parameter_member, input);

    assert!(
        result.is_ok(),
        "Failed to parse return_parameter_member: {:?}",
        result.err()
    );
}

#[test]
fn test_return_parameter_member_with_semicolon() {
    let input = "return : Anything[0..*] = fn.samples.domainValue;";
    let result = SysMLParser::parse(Rule::return_parameter_member, input);

    // Should consume everything EXCEPT the semicolon
    match result {
        Ok(mut pairs) => {
            let pair = pairs.next().unwrap();
            let consumed = pair.as_str();
            assert_eq!(consumed, "return : Anything[0..*] = fn.samples.domainValue");
        }
        Err(e) => panic!("Failed to parse: {:?}", e),
    }
}

#[test]
fn test_calculation_body_item_return() {
    let input = "return : Anything[0..*] = fn.samples.domainValue;";
    let result = SysMLParser::parse(Rule::calculation_body_item, input);

    assert!(
        result.is_ok(),
        "Failed to parse calculation_body_item with return: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_part_with_param_and_return() {
    let input = "in fn : SampledFunction; return : Anything[0..*] = fn.samples.domainValue;";
    let result = SysMLParser::parse(Rule::calculation_body_part, input);

    assert!(
        result.is_ok(),
        "Failed to parse calculation_body_part: {:?}",
        result.err()
    );
}

#[test]
fn test_calculation_body_braces() {
    let input = "{ in fn : SampledFunction; return : Anything[0..*] = fn.samples.domainValue; }";
    let result = SysMLParser::parse(Rule::calculation_body, input);

    assert!(
        result.is_ok(),
        "Failed to parse calculation_body with braces: {:?}",
        result.err()
    );
}
