use std::fmt;

use hdk::prelude::holo_hash::{AgentPubKeyB64, HeaderHashB64};
use hdk::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRecurrentEvent {
    pub agent_pub_key: AgentPubKeyB64,
    pub recurrent_event: RecurrentEvent,
}

/// RecurrentEvent entry definition.
///
/// The recurrent_event must include at a minimum TODO::
///
#[hdk_entry(id = "recurrent_event", visibility = "public")]
#[derive(Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecurrentEvent {
    pub title: String,
    pub event_entry_header: HeaderHashB64,
    pub event_window: TimeFrame,
    pub first_event_ts: u64,
    pub frequency_unit_of_event: Period,
    pub timezone_offset: i8,
    pub min_number_of_occurences: u32,
    pub occurence_restriction: OccurenceRestriction,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeFrame {
    start_ts: u64,
    end_ts: EndPoint,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EndPoint {
    Ongoing,
    FixedTimestamp(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Period {
    Per24(u8),
    Hourly,
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Bimonthly,
    Annually,
    Biannually,
}

#[derive(Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
pub struct UIEnum(pub String);

impl From<UIEnum> for Period {
    fn from(ui_enum: UIEnum) -> Self {
        match ui_enum.0.as_str() {
            "Hourly" => Self::Weekly,
            "Daily" => Self::Weekly,
            "Weekly" => Self::Weekly,
            "Biweekly" => Self::Biweekly,
            "Monthly" => Self::Monthly,
            "Bimonthly" => Self::Bimonthly,
            "Annually" => Self::Monthly,
            "Biannually" => Self::Bimonthly,
            x => match str::parse::<u8>(x) {
                Ok(freq) => Self::Per24(freq),
                Err(_err) => Self::Daily,
            },
        }
    }
}
impl From<Period> for UIEnum {
    fn from(period_description: Period) -> Self {
        Self(period_description.to_string())
    }
}
impl fmt::Display for Period {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OccurenceRestriction {
    OnlyWeekdays,
    OnlyWeekends,
    None,
}

impl From<UIEnum> for OccurenceRestriction {
    fn from(ui_enum: UIEnum) -> Self {
        match ui_enum.0.as_str() {
            "OnlyWeekdays" => Self::OnlyWeekdays,
            "OnlyWeekends" => Self::OnlyWeekends,
            _ => Self::None,
        }
    }
}
impl From<OccurenceRestriction> for UIEnum {
    fn from(restriction: OccurenceRestriction) -> Self {
        Self(restriction.to_string())
    }
}
impl fmt::Display for OccurenceRestriction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
