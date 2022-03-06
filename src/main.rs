use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
	 #[error("unknown error")]
	 UnknownError
}

fn main() -> Result<(), AppError> {

    println!("Hello, world!");
	 Ok(())
}
