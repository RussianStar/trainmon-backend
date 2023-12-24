use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug,PartialEq)]
pub enum  FileProviderOption{
    RawPath(RawPath),
    Folder(FolderOption)
}

#[derive(Serialize, Deserialize, Clone, Debug,PartialEq)]
pub struct RawPath{
    pub paths: Vec<String>
}

#[derive(Serialize, Deserialize, Clone, Debug,PartialEq)]
pub struct FolderOption {
    pub path: String
}