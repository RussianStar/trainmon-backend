use  crate::domain::core::user_model::UserModel;
use crate::domain::model::results::general_result::GeneralResult;
use crate::domain::model::http::file_provider_option::FileProviderOption;
use std::future::Future;
use std::pin::Pin;

pub trait FitFileProcessingCommand {
    fn execute(self, file_provider_options: &FileProviderOption, analysis_modes: Vec<String>, user_profile: UserModel) -> Pin<Box<dyn Future<Output = Vec<GeneralResult>> + Send+ '_>>;
}