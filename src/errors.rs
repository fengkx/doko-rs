use std::io;
#[derive(Debug)]
pub enum DockerRunErr {
    VersionConflict,
    RunErr,
    IoErr(io::Error),
}

impl From<io::Error> for DockerRunErr {
    fn from(error: io::Error) -> Self {
        DockerRunErr::IoErr(error)
    }
}
