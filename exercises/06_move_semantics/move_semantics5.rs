// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// <<<<<<< HEAD:exercises/06_move_semantics/move_semantics5.rs

#[test]
// =======
// >>>>>>> 720eef0 (forgot to fork, we'll figure something out):exercises/move_semantics/move_semantics5.rs
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
