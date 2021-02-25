use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing KeysetRoot")]
    MissingKeysetRoot,

    #[error("Missing Device")]
    MissingDevice,
}