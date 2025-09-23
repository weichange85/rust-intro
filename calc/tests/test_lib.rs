// unit tests for src/lib.rs

use calc::add;

#[test]
fn test_add() {
    // Arrange
    let test_input_1 = 1;
    let test_input_2 = 2;
    let expected_result = 3;
    // Act
    let actual_result = add(test_input_1, test_input_2);
    // Assert
    assert_eq!(expected_result, actual_result);
}