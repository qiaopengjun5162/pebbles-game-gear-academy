use gtest::{Program, System};
use pebbles_game_io::*;

#[test]
fn test() {
    let mut program = Program::new(
        r#"
        // This is a comment
        // Another comment
        // This is a comment
        // Another comment
        "#,
    );
    let mut system = System::new();
    system.run_program(&mut program);
    assert_eq!(system.get_output(), "");
    assert_eq!(system.get_error(), "");
}
