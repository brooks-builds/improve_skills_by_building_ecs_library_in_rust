use eyre::Result;

#[test]
fn converting_options_to_results() -> Result<()> {
    let something = maybe_something().ok_or(CustomError::SomethingWasNothing)?;
    assert_eq!(something, 52);
    Ok(())
}

fn maybe_something() -> Option<u32> {
    Some(52)
}

#[derive(Debug, thiserror::Error)]
enum CustomError {
    #[error("Something wasn't really something, it turned out to be nothing")]
    SomethingWasNothing,
}
