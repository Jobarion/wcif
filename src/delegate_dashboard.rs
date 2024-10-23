use monostate::MustBe;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::NoneAsEmptyString;
use crate::types::WCAUserId;

// According to spec the id must be com.delegate-dashboard.groups, but that's not what is used in practice
// To reliably identify it this library matches against the spec url, which could potentially break
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupsExtension {
    pub id: MustBe!("undefined.groups"),
    pub spec_url: MustBe!("https://github.com/coder13/delegateDashboard/blob/main/public/wcif-extensions/groups.json"),
    pub data: GroupsConfig,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupsConfig {
    pub groups: u32,
    pub spread_groups_across_all_stages: Option<bool>,
}
