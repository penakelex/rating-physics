#[derive(Clone)]
pub enum CipheredFileType {
    Rpf,
    Zip,
}

impl CipheredFileType {
    pub fn try_from_str(s: &str) -> Option<Self> {
        match s {
            "rpf" => Some(CipheredFileType::Rpf),
            "zip" => Some(CipheredFileType::Zip),
            _ => None,
        }
    }
}

impl CipheredFileType {
    pub fn as_mime_type_str(&self) -> &str {
        match self {
            CipheredFileType::Rpf => "application/octet-stream",
            CipheredFileType::Zip => "application/zip",
        }
    }
}