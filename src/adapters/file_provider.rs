use crate::domain::model::http::file_provider_option::FileProviderOption;
use anyhow::{Result,Context};

pub fn provide_files(option: &FileProviderOption) -> Result<Vec<String>> {
    match option {
        FileProviderOption::RawPath(paths) => Ok(paths.paths.clone()),
        FileProviderOption::Folder(folder) => {
            let entries = std::fs::read_dir(&folder.path)
                .context("Failed to read directory")?;

            let mut files = Vec::new();


            for entry in entries {
                let entry = entry
                    .context("Failed to read directory entry")?;

                let path = entry.path();

                if path.is_file() && path.extension().map_or(false, |ext| ext.to_string_lossy().to_lowercase() == "fit") {
                    files.push(path.to_string_lossy().into_owned());
                }
            }

            Ok(files)
        }
    }
}