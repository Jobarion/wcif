use monostate::MustBe;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::NoneAsEmptyString;
use crate::types::WCAUserId;

#[cfg(feature = "parse_activity_code")]
type GroupIdType = crate::types::GroupIdType;
#[cfg(not(feature = "parse_activity_code"))]
type GroupIdType = u32;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityConfigExtension {
    pub id: MustBe!("groupifier.ActivityConfig"),
    pub spec_url: String,
    pub data: ActivityConfig,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityConfig {
    pub capacity: f32,
    pub groups: GroupIdType,
    pub scramblers: u32,
    pub runners: u32,
    pub assign_judges: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub featured_competitors_wca_user_ids: Vec<WCAUserId>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompetitionConfigExtension {
    pub id: MustBe!("groupifier.CompetitionConfig"),
    pub spec_url: String,
    pub data: CompetitionConfig,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompetitionConfig {
    pub local_names_first: bool,
    #[serde_as(as = "NoneAsEmptyString")]
    pub scorecards_background_url: Option<String>,
    pub competitors_sorting_rule: CompetitorsSortingRule,
    pub no_tasks_for_newcomers: bool,
    pub tasks_for_own_events_only: bool,
    pub no_running_for_foreigners: Option<bool>,
    pub print_stations: Option<bool>,
    pub scorecard_paper_size: Option<ScorecardPaperSize>,
    pub scorecard_order: Option<ScorecardOrder>
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompetitorsSortingRule {
    Ranks,
    Balanced,
    Symmetric,
    NameOptimised
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ScorecardPaperSize {
    A4,
    A6,
    Letter
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ScorecardOrder {
    Natural,
    Stacked
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomConfigExtension {
    pub id: MustBe!("groupifier.RoomConfig"),
    pub spec_url: String,
    pub data: RoomConfig,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomConfig {
    pub stations: u32,
}
