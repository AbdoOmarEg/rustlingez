// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
<<<<<<< HEAD:exercises/13_error_handling/errors4.rs
        // Hmm... Why is this always returning an Ok value?
        Ok(PositiveNonzeroInteger(value as u64))
=======
        // Hmm...? Why is this only returning an Ok value?
        match value {
            1.. => Ok(PositiveNonzeroInteger(value as u64)),
            0 => Err(CreationError::Zero),
            _ => Err(CreationError::Negative),
        }
        // Ok(PositiveNonzeroInteger(value as u64))
>>>>>>> 720eef0 (forgot to fork, we'll figure something out):exercises/error_handling/errors4.rs
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
