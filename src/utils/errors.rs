use failure::Fail;

#[derive(Debug, Fail)]
pub enum ReadError {
    #[fail(display = "open file error: {}", err)]
    OpenFileError { err: std::io::Error },
    #[fail(display = "can not read metadata: {}", err)]
    MetadataError { err: std::io::Error },
    #[fail(display = "is not file")]
    NotFileError,
    #[fail(display = "fail to create directory {}: {}", dir, err)]
    CreateDirError { dir: String, err: std::io::Error },
    #[fail(display = "fail to find home directory")]
    NoHomeDir,
    #[fail(display = "fail to read directory {}: {}", dir, err)]
    ReadDirError { dir: String, err: std::io::Error },
}

#[derive(Debug, Fail)]
pub enum MustacheError {
    #[fail(display = "mustache compile: {}", msg)]
    CompileError { msg: String },
    #[fail(display = "mustache render cannot find field: {}", missing_field)]
    DataNotFoundError { missing_field: String },
}
