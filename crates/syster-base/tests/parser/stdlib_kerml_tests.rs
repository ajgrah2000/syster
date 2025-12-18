#![allow(clippy::unwrap_used)]

use rstest::rstest;
use std::path::PathBuf;
use syster::project::file_loader;

/// Test that each KerML stdlib file can be parsed successfully
///
/// This test suite creates individual test cases for each KerML file in the standard library.
/// When a file fails to parse, the test name clearly indicates which file has the issue.
///
/// NOTE: Many KerML stdlib files currently fail to parse due to incomplete grammar support.
/// These tests are included to track progress on KerML parser implementation.

#[rstest]
// Kernel Data Type Library - Partially supported
#[case("Kernel Libraries/Kernel Data Type Library/Collections.kerml")]
#[case("Kernel Libraries/Kernel Data Type Library/ScalarValues.kerml")]
#[case("Kernel Libraries/Kernel Data Type Library/VectorValues.kerml")]
// Kernel Function Library - Most tests ignored due to parser limitations
#[case("Kernel Libraries/Kernel Function Library/BaseFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/BooleanFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/CollectionFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/ComplexFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/ControlFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/DataFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/IntegerFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/NaturalFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/NumericalFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/OccurrenceFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/RationalFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/RealFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/ScalarFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/SequenceFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/StringFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/TrigFunctions.kerml")]
#[case("Kernel Libraries/Kernel Function Library/VectorFunctions.kerml")]
// Kernel Semantic Library - Partially supported
#[case("Kernel Libraries/Kernel Semantic Library/Base.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Clocks.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/ControlPerformances.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/FeatureReferencingPerformances.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/KerML.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Links.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Metaobjects.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Objects.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Observation.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Occurrences.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Performances.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/SpatialFrames.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/StatePerformances.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Transfers.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/TransitionPerformances.kerml")]
#[case("Kernel Libraries/Kernel Semantic Library/Triggers.kerml")]
fn test_parse_stdlib_kerml_file(#[case] relative_path: &str) {
    let mut path = PathBuf::from("sysml.library");
    path.push(relative_path);

    let result = file_loader::load_and_parse(&path);

    assert!(
        result.is_ok(),
        "Failed to parse {}: {}",
        relative_path,
        result.err().unwrap()
    );
}
