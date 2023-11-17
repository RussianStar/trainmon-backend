use crate::ports::database::Database;
use crate::domain::model::results::general_result::GeneralResult;

pub struct DatabaseAdapter {
    // Database connection details
}

impl DatabaseAdapter {
    pub fn new() -> Self {
        // Initialize database connection
    }
}

impl Database for DatabaseAdapter {
    async fn save_results(&self, user_id: Uuid, results: Vec<Vec<AnalysisResult>>) -> Result<(), Box<dyn std::error::Error>> {
        let mut transaction = self.pool.begin().await?;

        for result in &results {
            let unique_id = generate_uuid(); // Generate UUID for each Vec<AnalysisResult>
            for analysis_result in result {
                match analysis_result {
                    AnalysisResult::Overview(workout_summary) => {
                        let query = sqlx::query!(
                            r#"
                            INSERT INTO workouts (id, user_id, start_time, end_time, duration, sport, distance, tss)
                            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                            "#,
                            unique_id,
                            user_id,
                            workout_summary.start,
                            workout_summary.end,
                            workout_summary.duration,
                            workout_summary.sport,
                            workout_summary.distance,
                            workout_summary.tss
                        );
                        query.execute(&mut transaction).await?;
                    },
                    AnalysisResult::HeartRate(heart_rate_result) => {
                        let query = sqlx::query!(
                            r#"
                            INSERT INTO heart_rate_data (workout_id, average, time_in_zone, average_effective, time_in_zone_effective)
                            VALUES ($1, $2, $3, $4, $5)
                            "#,
                            unique_id,
                            heart_rate_result.average,
                            heart_rate_result.time_in_zone,
                            heart_rate_result.average_effective,
                            heart_rate_result.time_in_zone_effective
                        );
                        query.execute(&mut transaction).await?;
                    },
                    AnalysisResult::Power(power_result) => {
                        let query = sqlx::query!(
                            r#"
                            INSERT INTO power_data (workout_id, average, weighted_average, normalized, time_in_zone, time_in_zone_effective)
                            VALUES ($1, $2, $3, $4, $5, $6)
                            "#,
                            unique_id,
                            power_result.average,
                            power_result.weighted_average,
                            power_result.normalized,
                            power_result.time_in_zone,
                            power_result.time_in_zone_effective
                        );
                        query.execute(&mut transaction).await?;
                    },
                }
            }
        }

        transaction.commit().await?;
        Ok(())
    }
    // Implement other methods
}

use uuid::Uuid;
use sha2::{Sha256, Digest};

fn calculate_hash(result: &GeneralResult) -> String {
    let mut hasher = Sha256::new();
    hasher.update(result.to_string());
    format!("{:x}", hasher.finalize())
}

fn generate_uuid_from_hash(hash: &str) -> Uuid {
    Uuid::new_v5(&Uuid::NAMESPACE_DNS, hash)
}

fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}