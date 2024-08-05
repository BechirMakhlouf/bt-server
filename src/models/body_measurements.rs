use std::fmt::Display;

use crate::types::past_naive_date::PastNaiveDate;
use crate::types::positive_non_zero_float::PosNonZeroF32;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{user, user_weight::WeightKg};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct BodyMeasurementsCm {
    pub user_id: user::Id,
    pub date_at: PastNaiveDate,

    pub height: Option<PosNonZeroF32>,
    pub left_arm: Option<PosNonZeroF32>,
    pub right_arm: Option<PosNonZeroF32>,
    pub left_thigh: Option<PosNonZeroF32>,
    pub right_thigh: Option<PosNonZeroF32>,
    pub left_wrist: Option<PosNonZeroF32>,
    pub right_wrist: Option<PosNonZeroF32>,
    pub neck: Option<PosNonZeroF32>,
    pub left_calf: Option<PosNonZeroF32>,
    pub right_calf: Option<PosNonZeroF32>,
    pub hips: Option<PosNonZeroF32>,
    pub torso: Option<PosNonZeroF32>,
    pub waist: Option<PosNonZeroF32>,
    pub weight_kg: Option<WeightKg>,
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
    height: Option<PosNonZeroF32>,
    left_arm: Option<PosNonZeroF32>,
    right_arm: Option<PosNonZeroF32>,
    left_thigh: Option<PosNonZeroF32>,
    right_thigh: Option<PosNonZeroF32>,
    left_wrist: Option<PosNonZeroF32>,
    right_wrist: Option<PosNonZeroF32>,
    neck: Option<PosNonZeroF32>,
    left_calf: Option<PosNonZeroF32>,
    right_calf: Option<PosNonZeroF32>,
    hips: Option<PosNonZeroF32>,
    torso: Option<PosNonZeroF32>,
    waist: Option<PosNonZeroF32>,
    weight_kg: Option<WeightKg>,
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
            weight_kg: self.weight_kg,
        }
    }

    pub fn height(mut self, height: Option<PosNonZeroF32>) -> Self {
        self.height = height;
        self
    }
    pub fn left_arm(mut self, left_arm: Option<PosNonZeroF32>) -> Self {
        self.left_arm = left_arm;
        self
    }
    pub fn right_arm(mut self, right_arm: Option<PosNonZeroF32>) -> Self {
        self.right_arm = right_arm;
        self
    }
    pub fn left_thigh(mut self, left_thigh: Option<PosNonZeroF32>) -> Self {
        self.left_thigh = left_thigh;
        self
    }
    pub fn right_thigh(mut self, right_thigh: Option<PosNonZeroF32>) -> Self {
        self.right_thigh = right_thigh;
        self
    }
    pub fn left_wrist(mut self, left_wrist: Option<PosNonZeroF32>) -> Self {
        self.left_wrist = left_wrist;
        self
    }
    pub fn right_wrist(mut self, right_wrist: Option<PosNonZeroF32>) -> Self {
        self.right_wrist = right_wrist;
        self
    }
    pub fn neck(mut self, neck: Option<PosNonZeroF32>) -> Self {
        self.neck = neck;
        self
    }
    pub fn left_calf(mut self, left_calf: Option<PosNonZeroF32>) -> Self {
        self.left_calf = left_calf;
        self
    }
    pub fn right_calf(mut self, right_calf: Option<PosNonZeroF32>) -> Self {
        self.right_calf = right_calf;
        self
    }
    pub fn hips(mut self, hips: Option<PosNonZeroF32>) -> Self {
        self.hips = hips;
        self
    }
    pub fn torso(mut self, torso: Option<PosNonZeroF32>) -> Self {
        self.torso = torso;
        self
    }
    pub fn waist(mut self, waist: Option<PosNonZeroF32>) -> Self {
        self.waist = waist;
        self
    }
    pub fn weight_kg(mut self, weight_kg: Option<WeightKg>) -> Self {
        self.weight_kg = weight_kg;
        self
    }
}
