use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("open file error: {err}")]
    OpenFileError { err: std::io::Error },
    #[error("can not read metadata: {err}")]
    MetadataError { err: std::io::Error },
    #[error("is not file")]
    NotFileError,
    #[error("fail to create directory {dir}: {err}")]
    CreateDirError { dir: String, err: std::io::Error },
    #[error("fail to find home directory")]
    NoHomeDir,
    #[error("fail to read directory {dir}: {err}")]
    ReadDirError { dir: String, err: std::io::Error },
}

#[derive(Debug, Error)]
pub enum MustacheError {
    #[error("mustache compile: {msg}")]
    CompileError { msg: String },
    #[error("mustache render cannot find field: {missing_field}")]
    DataNotFoundError { missing_field: String },
}

#[derive(Debug, Error)]
pub enum CipherError {
    #[error("password validation fail")]
    PasswordError,
}
