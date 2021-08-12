use thiserror::Error;

use eyre::Result;

#[test]
fn results() -> Result<()> {
    this_always_fails()?;
    this_also_fails()?;
    Ok(())
}

fn this_always_fails() -> Result<()> {
    Err(CustomError::AlwaysFails.into())
}

fn this_also_fails() -> Result<()> {
    Err(CustomError::OfCourseItFailed(42).into())
}

#[derive(Debug, Error)]
enum CustomError {
    #[error("This always fails, I don't know what you expected")]
    AlwaysFails,
    #[error("This also always fails, but the value it failed with was {0}")]
    OfCourseItFailed(u32),
}
