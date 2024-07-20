use super::user;

#[derive(Debug)]
pub struct WeightLog {
    user_id: user::Id,
    weight_kg: f32,
    date: chrono::NaiveDate,
}
