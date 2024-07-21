use std::fmt::Display;

use crate::types::negative_non_zero_float::PositiveNonZeroF64;
use crate::types::past_naive_date::PastNaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user;

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Id(uuid::Uuid);

impl Id {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn get_value(&self) -> &Uuid {
        &self.0
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<Id> for Uuid {
    fn from(value: Id) -> Self {
        value.0
    }
}
impl From<&Id> for Uuid {
    fn from(value: &Id) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub struct BodyMeasurementsCm {
    pub user_id: user::Id,
    pub date_at: PastNaiveDate,
    pub height: Option<PositiveNonZeroF64>,
    pub left_arm: Option<PositiveNonZeroF64>,
    pub right_arm: Option<PositiveNonZeroF64>,
    pub left_thigh: Option<PositiveNonZeroF64>,
    pub right_thigh: Option<PositiveNonZeroF64>,
    pub left_wrist: Option<PositiveNonZeroF64>,
    pub right_wrist: Option<PositiveNonZeroF64>,
    pub neck: Option<PositiveNonZeroF64>,
    pub left_calf: Option<PositiveNonZeroF64>,
    pub right_calf: Option<PositiveNonZeroF64>,
    pub hips: Option<PositiveNonZeroF64>,
    pub torso: Option<PositiveNonZeroF64>,
    pub waist: Option<PositiveNonZeroF64>,
}
impl BodyMeasurementsCm {
    pub fn builder(user_id: user::Id, date_at: PastNaiveDate) -> BodyMeasurementsCmBuilder {
        BodyMeasurementsCmBuilder::new(user_id, date_at)
    }
}

#[derive(Default)]
pub struct BodyMeasurementsCmBuilder {
    user_id: user::Id,
    date_at: PastNaiveDate,
    height: Option<PositiveNonZeroF64>,
    left_arm: Option<PositiveNonZeroF64>,
    right_arm: Option<PositiveNonZeroF64>,
    left_thigh: Option<PositiveNonZeroF64>,
    right_thigh: Option<PositiveNonZeroF64>,
    left_wrist: Option<PositiveNonZeroF64>,
    right_wrist: Option<PositiveNonZeroF64>,
    neck: Option<PositiveNonZeroF64>,
    left_calf: Option<PositiveNonZeroF64>,
    right_calf: Option<PositiveNonZeroF64>,
    hips: Option<PositiveNonZeroF64>,
    torso: Option<PositiveNonZeroF64>,
    waist: Option<PositiveNonZeroF64>,
}

impl BodyMeasurementsCmBuilder {
    pub fn new(user_id: user::Id, date_at: PastNaiveDate) -> Self {
        Self {
            user_id,
            date_at,
            ..Default::default()
        }
    }
    pub fn build(self) -> BodyMeasurementsCm {
        BodyMeasurementsCm {
            user_id: self.user_id,
            date_at: self.date_at,
            height: self.height,
            left_arm: self.left_arm,
            right_arm: self.right_arm,
            left_thigh: self.left_thigh,
            right_thigh: self.right_thigh,
            left_wrist: self.left_wrist,
            right_wrist: self.right_wrist,
            neck: self.neck,
            left_calf: self.left_calf,
            right_calf: self.right_calf,
            hips: self.hips,
            torso: self.torso,
            waist: self.waist,
        }
    }

    pub fn height(mut self, height: PositiveNonZeroF64) -> Self {
        self.height = Some(height);
        self
    }
    pub fn left_arm(mut self, left_arm: PositiveNonZeroF64) -> Self {
        self.left_arm = Some(left_arm);
        self
    }
    pub fn right_arm(mut self, right_arm: PositiveNonZeroF64) -> Self {
        self.right_arm = Some(right_arm);
        self
    }
    pub fn left_thigh(mut self, left_thigh: PositiveNonZeroF64) -> Self {
        self.left_thigh = Some(left_thigh);
        self
    }
    pub fn right_thigh(mut self, right_thigh: PositiveNonZeroF64) -> Self {
        self.right_thigh = Some(right_thigh);
        self
    }
    pub fn left_wrist(mut self, left_wrist: PositiveNonZeroF64) -> Self {
        self.left_wrist = Some(left_wrist);
        self
    }
    pub fn right_wrist(mut self, right_wrist: PositiveNonZeroF64) -> Self {
        self.right_wrist = Some(right_wrist);
        self
    }
    pub fn neck(mut self, neck: PositiveNonZeroF64) -> Self {
        self.neck = Some(neck);
        self
    }
    pub fn left_calf(mut self, left_calf: PositiveNonZeroF64) -> Self {
        self.left_calf = Some(left_calf);
        self
    }
    pub fn right_calf(mut self, right_calf: PositiveNonZeroF64) -> Self {
        self.right_calf = Some(right_calf);
        self
    }
    pub fn hips(mut self, hips: PositiveNonZeroF64) -> Self {
        self.hips = Some(hips);
        self
    }
    pub fn torso(mut self, torso: PositiveNonZeroF64) -> Self {
        self.torso = Some(torso);
        self
    }
    pub fn waist(mut self, waist: PositiveNonZeroF64) -> Self {
        self.waist = Some(waist);
        self
    }
}
