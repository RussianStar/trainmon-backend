use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum  FileProviderOption{
    RawPath(RawPath),
    Folder(FolderOption)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RawPath{
    pub paths: Vec<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FolderOption {
    pub path: String
}