use crate::models::GrandCompany;

use chrono::{DateTime, Utc};

use ffxiv_types::World;

use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct FreeCompanySearchItem {
  pub id: u64,
  pub name: String,
  pub world: World,
  #[serde(with = "crate::util::serde::multi_url")]
  pub crest: Vec<Url>,
  pub grand_company: GrandCompany,
  pub active_members: u16,
  pub estate_built: bool,
  pub formed: DateTime<Utc>,
  pub active: Active,
  pub recruitment: RecruitmentStatus,
}

ffxiv_enum!(Active {
  Always => "Always",
  Weekdays => "Weekdays",
  Weekends => "Weekends",
  Never => "Never",
  NotSpecified => "Not specified",
});

ffxiv_enum!(RecruitmentStatus {
  Open => "Open",
  Closed => "Closed",
});