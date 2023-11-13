use  crate::domain::core::user_model::UserModel;
use crate::domain::model::results::analysis_result::AnalysisResult;

use std::future::Future;
use std::pin::Pin;

pub trait FitFileProcessingCommand {
    fn execute(self, file_paths: &Vec<String>, analysis_modes: Vec<String>, user_profile: UserModel) -> Pin<Box<dyn Future<Output = Vec<AnalysisResult>> + Send+ '_>>;
}