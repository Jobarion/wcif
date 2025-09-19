use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use monostate::MustBe;
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use serde_json::Value;
use chrono::TimeDelta;
use serde_with::{DeserializeFromStr, SerializeDisplay, skip_serializing_none};

pub type CompetitionId = String;
pub type SeriesId = String;
pub type PersonId = u32;
pub type VenueId = u32;
pub type ActivityId = u32;
pub type WCAUserId = u64;
pub type WCARegistrationId = u32;
pub type ScrambleSetId = u32;
pub type RoomId = u32;
pub type CountryCode = String;
pub type CurrencyCode = String;
pub type Date = chrono::NaiveDate;
pub type DateTime = chrono::DateTime<chrono::Utc>;

#[cfg(not(feature = "parse_puzzle_type"))]
pub type EventId = String;
#[cfg(feature = "parse_puzzle_type")]
pub type EventId = puzzle_types::OfficialEventId;

#[cfg(feature = "parse_puzzle_type")]
pub type PuzzleType = puzzle_types::OfficialPuzzleType;

#[cfg(not(feature = "parse_activity_code"))]
pub type ActivityCode = String;
#[cfg(not(feature = "parse_activity_code"))]
pub type RoundId = String;
#[cfg(not(feature = "parse_activity_code"))]
pub type GroupId = String;
#[cfg(not(feature = "parse_activity_code"))]
pub type AttemptId = String;

#[cfg(feature = "parse_activity_code")]
pub type ActivityCode = activity_code::ActivityCode;
#[cfg(feature = "parse_activity_code")]
pub type RoundId = activity_code::RoundId<EventId>;
#[cfg(feature = "parse_activity_code")]
pub type RoundIdType = activity_code::RoundIdType;
#[cfg(feature = "parse_activity_code")]
pub type GroupIdType = activity_code::GroupIdType;
#[cfg(feature = "parse_activity_code")]
pub type AttemptIdType = activity_code::AttemptIdType;
#[cfg(feature = "parse_activity_code")]
pub type EventActivityCode = activity_code::EventActivityCode<EventId>;
#[cfg(feature = "parse_activity_code")]
pub type UnofficialActivityCode = activity_code::UnofficialActivityCode;
#[cfg(feature = "parse_activity_code")]
pub type UnofficialEventActivityCode = activity_code::EventActivityCode<String>;


#[cfg(not(feature = "parse_attempt_result"))]
pub type AttemptResult = i32;
#[cfg(feature = "parse_attempt_result")]
pub type CentiSecondsResultValue = attempt_result::CentiSecondAttemptResultValue;
#[cfg(feature = "parse_attempt_result")]
pub type FMCResultValue = attempt_result::FMCAttemptResultValue;
#[cfg(feature = "parse_attempt_result")]
pub type AttemptResult = attempt_result::AttemptResult<CentiSecondsResultValue>;
#[cfg(feature = "parse_attempt_result")]
pub type MultiBlindResultValue = attempt_result::MultiBlindAttemptResultValue;
#[cfg(feature = "parse_attempt_result")]
pub type MultiBlindAttemptResult = attempt_result::AttemptResult<MultiBlindResultValue>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Competition {
    pub format_version: MustBe!("1.0"),
    pub id: CompetitionId,
    pub name: String,
    pub short_name: String,
    pub series: Option<Series>,
    pub persons: Vec<Person>,
    pub events: Vec<Event>,
    pub schedule: Schedule,
    pub registration_info: RegistrationInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competitor_limit: Option<u32>,
    pub extensions: Vec<Extension>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateCompetition {
    pub format_version: MustBe!("1.0"),
    pub id: CompetitionId,
    pub name: String,
    pub short_name: String,
    pub series: Option<Series>,
    pub persons: Vec<PrivatePerson>,
    pub events: Vec<Event>,
    pub schedule: Schedule,
    pub registration_info: RegistrationInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competitor_limit: Option<u32>,
    pub extensions: Vec<Extension>
}

impl From<PrivateCompetition> for Competition {
    fn from(value: PrivateCompetition) -> Self {
        Self {
            format_version: value.format_version,
            id: value.id,
            name: value.name,
            short_name: value.short_name,
            series: value.series,
            persons: value.persons.into_iter()
                .map(|x|Into::<Person>::into(x))
                .collect(),
            events: value.events,
            schedule: value.schedule,
            registration_info: value.registration_info,
            competitor_limit: value.competitor_limit,
            extensions: value.extensions,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub id: SeriesId,
    pub name: String,
    pub short_name: String,
    pub competitions_ids: Vec<CompetitionId>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub registrant_id: Option<PersonId>,
    pub name: String,
    pub wca_user_id: WCAUserId,
    pub wca_id: Option<WCAId>,
    pub country_iso2: CountryCode,
    pub gender: Gender,
    pub avatar: Option<Avatar>,
    pub roles: Vec<Role>,
    pub registration: Option<Registration>,
    pub assignments: Vec<Assignment>,
    pub personal_bests: Vec<PersonalBest>,
    pub extensions: Vec<Extension>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivatePerson {
    pub registrant_id: Option<PersonId>,
    pub name: String,
    pub wca_user_id: WCAUserId,
    pub wca_id: Option<WCAId>,
    pub country_iso2: CountryCode,
    pub gender: Gender,
    pub birthdate: chrono::NaiveDate,
    pub email: String,
    pub avatar: Option<Avatar>,
    pub roles: Vec<Role>,
    pub registration: Option<PrivateRegistration>,
    pub assignments: Vec<Assignment>,
    pub personal_bests: Vec<PersonalBest>,
    pub extensions: Vec<Extension>
}

impl From<PrivatePerson> for Person {
    fn from(value: PrivatePerson) -> Self {
        Self {
            registrant_id: value.registrant_id,
            name: value.name,
            wca_user_id: value.wca_user_id,
            wca_id: value.wca_id,
            country_iso2: value.country_iso2,
            gender: value.gender,
            avatar: value.avatar,
            roles: value.roles,
            registration: value.registration.map(|x|Into::<Registration>::into(x)),
            assignments: value.assignments,
            personal_bests: value.personal_bests,
            extensions: value.extensions,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub enum Gender {
    #[serde(rename = "m")]
    Male,
    #[serde(rename = "f")]
    Female,
    #[serde(rename = "o")]
    Other
}

#[derive(Clone, PartialEq, Eq, Ord, Hash, SerializeDisplay, DeserializeFromStr)]
pub struct WCAId {
    pub year: u16,
    pub discriminant: u8,
    pub name: String
}

impl PartialOrd for WCAId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.year.cmp(&other.year)
            .then(self.name.cmp(&other.name))
            .then(self.discriminant.cmp(&other.discriminant)))
    }
}

impl Display for WCAId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{:0>2}", self.year, self.name, self.discriminant)
    }
}

impl Debug for WCAId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

#[derive(Debug)]
pub enum WCAIdParseError {
    ParseIntError(ParseIntError),
    LengthError(usize)
}

impl Display for WCAIdParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WCAIdParseError::ParseIntError(x) => write!(f, "{x}"),
            WCAIdParseError::LengthError(l) => write!(f, "Invalid WCA ID length {l}")
        }
    }
}

impl FromStr for WCAId {
    type Err = WCAIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(WCAIdParseError::LengthError(s.len()))
        }
        let year = u16::from_str(&s[..4]).map_err(|e|WCAIdParseError::ParseIntError(e))?;
        let name = &s[4..8];
        let discriminant = u8::from_str(&s[8..]).map_err(|e|WCAIdParseError::ParseIntError(e))?;
        Ok(WCAId {
            year,
            name: name.to_string(),
            discriminant,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    #[serde(rename = "delegate")]
    Delegate,
    #[serde(rename = "trainee-delegate")]
    TraineeDelegate,
    #[serde(rename = "organizer")]
    Organizer,
    #[serde(untagged)]
    Other(String),
}

impl Role {
    pub fn is_delegate(&self) -> bool {
        match self {
            Self::Delegate | Self::TraineeDelegate => true,
            _ => false
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
    pub wca_registration_id: WCARegistrationId,
    pub event_ids: Vec<EventId>,
    pub status: RegistrationStatus,
    pub is_competing: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateRegistration {
    pub wca_registration_id: WCARegistrationId,
    pub event_ids: Vec<EventId>,
    pub status: RegistrationStatus,
    pub guests: u32,
    pub comments: String,
    pub administrative_notes: String,
    pub is_competing: bool,
}

impl From<PrivateRegistration> for Registration {
    fn from(value: PrivateRegistration) -> Self {
        Self {
            wca_registration_id: value.wca_registration_id,
            event_ids: value.event_ids,
            status: value.status,
            is_competing: value.is_competing,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RegistrationStatus {
    Accepted,
    Pending,
    Deleted
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationInfo {
    pub open_time: DateTime,
    pub close_time: DateTime,
    pub base_entry_fee: u64,
    pub currency_code: CurrencyCode,
    pub on_the_spot_registration: bool,
    pub use_wca_registration: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub url: String,
    pub thumb_url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Assignment {
    pub activity_id: ActivityId,
    pub assignment_code: AssignmentCode,
    pub station_number: Option<u32>
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, SerializeDisplay, DeserializeFromStr)]
pub enum AssignmentCode {
    Competitor,
    Staff(StaffAssignment),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, SerializeDisplay, DeserializeFromStr)]
pub enum StaffAssignment {
    Judge,
    Scrambler,
    Runner,
    DataEntry,
    Announcer,
    Other(String)
}

impl StaffAssignment {
    pub fn is_competitor_staffing_role(&self) -> bool {
        match self {
            StaffAssignment::Judge => true,
            StaffAssignment::Scrambler => true,
            StaffAssignment::Runner => true,
            _ => false
        }
    }
}

impl Display for AssignmentCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Competitor => write!(f, "competitor"),
            Self::Staff(x) => write!(f, "staff-{x}"),
        }
    }
}

impl Display for StaffAssignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match &self {
            Self::Judge => "judge",
            Self::Scrambler => "scrambler",
            Self::Runner => "runner",
            Self::DataEntry => "dataentry",
            Self::Announcer => "announcer",
            Self::Other(x) => x.as_str(),
        })
    }
}

impl FromStr for AssignmentCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "competitor" => Ok(Self::Competitor),
            x if x.starts_with("staff-") && x.len() > 6 => Ok(Self::Staff(StaffAssignment::from_str(&x[6..])?)),
            _ => Err("invalid staff assignment".to_string())
        }
    }
}

impl FromStr for StaffAssignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "judge" => Ok(Self::Judge),
            "scrambler" => Ok(Self::Scrambler),
            "runner" => Ok(Self::Runner),
            "dataentry" => Ok(Self::DataEntry),
            "announcer" => Ok(Self::Announcer),
            x => Ok(Self::Other(x.to_owned())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalBest {
    pub event_id: EventId,
    pub best: AttemptResult,
    #[serde(rename = "type")]
    pub _type: ResultType,
    pub world_ranking: u64,
    pub continental_ranking: u64,
    pub national_ranking: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: EventId,
    pub rounds: Vec<Round>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub competitor_limit: Option<u32>,
    pub qualification: Option<Qualification>,
    pub extensions: Vec<Extension>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Round {
    pub id: RoundId,
    pub format: RoundFormat,
    pub time_limit: Option<TimeLimit>,
    pub cutoff: Option<Cutoff>,
    pub advancement_condition: Option<AdvancementCondition>,
    pub results: Vec<RoundResult>,
    pub scramble_set_count: u32,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub scramble_sets: Vec<ScrambleSet>,
    pub extensions: Vec<Extension>,
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RoundFormat {
    #[serde(rename = "1")]
    BestOf1,
    #[serde(rename = "2")]
    BestOf2,
    #[serde(rename = "3")]
    BestOf3,
    #[serde(rename = "a")]
    AverageOf5,
    #[serde(rename = "m")]
    MeanOf3,
}

impl RoundFormat {
    pub fn expected_solve_count(&self) -> u8 {
        match self {
            RoundFormat::BestOf1 => 1,
            RoundFormat::BestOf2 => 2,
            RoundFormat::BestOf3 => 3,
            RoundFormat::AverageOf5 => 5,
            RoundFormat::MeanOf3 => 3,
        }
    }

    pub fn sort_by(&self) -> ResultType {
        match self {
            RoundFormat::BestOf1 => ResultType::Single,
            RoundFormat::BestOf2 => ResultType::Single,
            RoundFormat::BestOf3 => ResultType::Single,
            RoundFormat::AverageOf5 => ResultType::Average,
            RoundFormat::MeanOf3 => ResultType::Average,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeLimit {
    pub centiseconds: u32,
    pub cumulative_round_ids: Vec<RoundId>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cutoff {
    pub number_of_attempts: usize,
    pub attempt_result: AttemptResult,
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum AdvancementCondition {
    #[serde(rename = "ranking")]
    Ranking{level: Ranking},
    #[serde(rename = "percent")]
    Percent{level: Percent},
    #[serde(rename = "attemptResult")]
    AttemptResult{level: AttemptResult},
}

pub type Ranking = u64;
pub type Percent = u8;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Qualification {
    pub when_date: Date,
    #[serde(flatten)]
    pub _type: QualificationType,
    pub result_type: ResultType,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", content = "level")]
pub enum QualificationType {
    AttemptResult(AttemptResult),
    Ranking(Ranking),
    #[serde(serialize_with = "serialize_any_result")]
    #[serde(deserialize_with = "deserialize_any_result")]
    AnyResult(),
}

fn serialize_any_result<S: Serializer>(serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_none()
}

fn deserialize_any_result<'de, D: Deserializer<'de>>(_: D) -> Result<(), D::Error> {
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct RoundResult {
    pub person_id: PersonId,
    pub ranking: Option<u64>,
    pub attempts: Vec<Attempt>,
    pub best: AttemptResult,
    pub average: AttemptResult,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Attempt {
    pub result: AttemptResult,
    pub reconstruction: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScrambleSet {
    pub id: ScrambleSetId,
    pub scrambles: Vec<Scramble>,
    pub extra_scrambles: Vec<Scramble>,
}

pub type Scramble = String;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub start_date: Date,
    pub number_of_days: u8,
    pub venues: Vec<Venue>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub id: VenueId,
    pub name: String,
    pub latitude_microdegrees: u32,
    pub longitude_microdegrees: u32,
    pub country_iso2: CountryCode,
    pub timezone: String,
    pub rooms: Vec<Room>,
    pub extensions: Vec<Extension>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub id: RoomId,
    pub name: String,
    pub color: String,
    pub activities: Vec<Activity>,
    pub extensions: Vec<Extension>
}

#[derive(Clone, Debug, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResultType {
    Single,
    Average
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[skip_serializing_none]
pub struct Activity {
    pub id: ActivityId,
    pub name: String,
    pub activity_code: ActivityCode,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub child_activities: Vec<Activity>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scramble_set_id: Option<ScrambleSetId>,
    pub extensions: Vec<Extension>
}

impl Activity {
    pub fn get_duration(&self) -> TimeDelta {
        self.end_time.signed_duration_since(&self.start_time)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Extension {
    #[cfg(feature = "extension_groupifier")]
    #[serde(untagged)]
    GroupifierCompetitionConfig(crate::groupifier::CompetitionConfigExtension),
    #[cfg(feature = "extension_groupifier")]
    #[serde(untagged)]
    GroupifierActivityConfig(crate::groupifier::ActivityConfigExtension),
    #[cfg(feature = "extension_groupifier")]
    #[serde(untagged)]
    GroupifierRoomConfig(crate::groupifier::RoomConfigExtension),
    #[cfg(feature = "extension_delegate_dashboard")]
    #[serde(untagged)]
    DelegateDashboardGroups(crate::delegate_dashboard::GroupsExtension),
    #[serde(untagged)]
    Unknown(UnknownExtension)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnknownExtension {
    pub id: String,
    pub spec_url: String,
    pub data: Value
}

#[cfg(feature = "parse_attempt_result")]
mod attempt_result {
    use std::cmp::Ordering;
    use std::fmt::{Debug, Display, Formatter};
    use std::hash::Hash;
    use serde::{Serializer};
    use serde::de::Error;
    use serde_json::Value;

    #[derive(Copy, Clone, PartialEq, Eq, Ord, Debug, Hash)]
    pub enum AttemptResult<ARV: Ord + Eq + Copy> {
        Skipped,
        DNF,
        DNS,
        Success(ARV),
    }

    impl <ARV: Ord + Eq + Copy> AttemptResult<ARV> {
        pub fn is_success(&self) -> bool {
            if let AttemptResult::Success(_) = self {
                true
            } else {
                false
            }
        }
    }

    #[derive(Copy, Clone, PartialEq, Eq, Ord, Debug, Hash)]
    pub struct MultiBlindAttemptResultValue {
        attempted: u32,
        solved: u32,
        time: CentiSecondAttemptResultValue,
        old_style: bool,
    }

    pub type CentiSecondAttemptResultValue = u32;
    pub type FMCAttemptResultValue = u16;

    impl MultiBlindAttemptResultValue {
        pub fn attempted(&self) -> u32 {
            self.attempted
        }

        pub fn solved(&self) -> u32 {
            self.solved
        }

        pub fn failed(&self) -> u32 {
            self.attempted - self.solved
        }

        pub fn points(&self) -> u32 {
            self.solved() - self.failed()
        }

        pub fn seconds(&self) -> CentiSecondAttemptResultValue {
            self.time
        }

        pub fn is_old_style(&self) -> bool {
            self.old_style
        }
    }

    impl AttemptResult<CentiSecondAttemptResultValue> {
        pub fn to_multi_blind(&self) -> AttemptResult<MultiBlindAttemptResultValue> {
            (*self).into()
        }

        pub fn to_fmc(&self) -> AttemptResult<FMCAttemptResultValue> {
            (*self).into()
        }
    }

    impl<'de> serde::Deserialize<'de> for MultiBlindAttemptResultValue {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            CentiSecondAttemptResultValue::deserialize(d).map(Into::<MultiBlindAttemptResultValue>::into)
        }
    }

    impl serde::Serialize for MultiBlindAttemptResultValue {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
        {
            let value = if self.is_old_style() {
                1000000000 +
                    (99 - self.solved()) * 10000000 +
                    self.attempted() * 100000 +
                    self.seconds()
            } else {
                (99 - (self.solved() - self.failed())) * 10000000 +
                    self.seconds() * 100 +
                    self.failed()
            };
            serializer.serialize_i64(value as i64)
        }
    }

    impl AttemptResult<CentiSecondAttemptResultValue> {
        pub fn as_multi_blind(&self) -> AttemptResult<MultiBlindAttemptResultValue> {
            (*self).into()
        }
    }

    impl <ARV: Copy + Eq + Ord + From<u32>> TryFrom<i64> for AttemptResult<ARV> {
        type Error = ();

        fn try_from(value: i64) -> Result<Self, Self::Error> {
            Ok(match value {
                -2 => AttemptResult::DNS,
                -1 => AttemptResult::DNF,
                0 => AttemptResult::Skipped,
                x if x > 0 => AttemptResult::Success(ARV::from(x as u32)),
                _ => return Err(())
            })
        }
    }

    impl From<AttemptResult<CentiSecondAttemptResultValue>> for AttemptResult<MultiBlindAttemptResultValue> {
        fn from(value: AttemptResult<CentiSecondAttemptResultValue>) -> Self {
            match value {
                AttemptResult::Skipped => Self::Skipped,
                AttemptResult::DNF => Self::DNF,
                AttemptResult::DNS => Self::DNS,
                AttemptResult::Success(ast) => Self::Success(MultiBlindAttemptResultValue::from(ast))
            }
        }
    }

    impl From<AttemptResult<CentiSecondAttemptResultValue>> for AttemptResult<FMCAttemptResultValue> {
        fn from(value: AttemptResult<CentiSecondAttemptResultValue>) -> Self {
            match value {
                AttemptResult::Skipped => Self::Skipped,
                AttemptResult::DNF => Self::DNF,
                AttemptResult::DNS => Self::DNS,
                AttemptResult::Success(ast) => Self::Success(ast as FMCAttemptResultValue)
            }
        }
    }

    impl From<CentiSecondAttemptResultValue> for MultiBlindAttemptResultValue {
        fn from(value: CentiSecondAttemptResultValue) -> Self {
            if value < 1000000000 {
                let missed = value % 100;
                let value = value / 100;
                let time = value % 100000;
                let value = value / 100000;
                let difference = 99 - value;
                let solved = difference + missed;
                let attempted = solved + missed;
                Self {
                    attempted,
                    solved,
                    time: CentiSecondAttemptResultValue::from(time),
                    old_style: false
                }
            } else {
                let time = value % 100000;
                let value = value / 100000;
                let attempted = value % 100;
                let value = value / 100;
                let solved = 99 - (value % 100);
                Self {
                    attempted,
                    solved,
                    time: CentiSecondAttemptResultValue::from(time),
                    old_style: true
                }
            }
        }
    }

    impl PartialOrd for MultiBlindAttemptResultValue {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let cmp = self.points().cmp(&other.points())
                .then(self.time.cmp(&other.time).reverse())
                .then(self.failed().cmp(&other.failed()).reverse());
            Some(cmp)
        }
    }

    impl<'de, ARV: Ord + Eq + Copy + From<u32>> serde::Deserialize<'de> for AttemptResult<ARV> {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = Value::deserialize(d)?;
            Self::try_from(value.as_i64().ok_or(Error::custom("Not a number"))?)
                .map_err(|_|Error::custom("not a valid result"))
        }
    }

    impl <ARV: Ord + Eq + Into<u32> + Copy> serde::Serialize for AttemptResult<ARV> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
        {
            match &self {
                AttemptResult::Skipped => serializer.serialize_i64(0),
                AttemptResult::DNF => serializer.serialize_i64(-1),
                AttemptResult::DNS => serializer.serialize_i64(-2),
                AttemptResult::Success(x) => serializer.serialize_i64(Into::<u32>::into(*x) as i64),
            }
        }
    }

    impl <ARV: Ord + Eq + Copy> PartialOrd for AttemptResult<ARV> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(match (*self, *other) {
                (Self::Success(a), Self::Success(b)) => a.cmp(&b).reverse(),
                (Self::Success(_), _) => Ordering::Greater,
                (_, Self::Success(_)) => Ordering::Less,
                _ => Ordering::Equal
            })
        }
    }

    impl <ARV: Ord + Eq + Copy> AttemptResult<ARV> {
        pub fn ok(&self) -> Option<ARV> {
            match *self {
                AttemptResult::Success(a) => Some(a),
                _ => None,
            }
        }
    }

    impl Display for AttemptResult<CentiSecondAttemptResultValue> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            self.fmt_or_else(f, &|arv, f|{
                if arv < 100 * 60 {
                    write!(f, "{}.{:0>2}", arv / 100, arv % 100)
                } else if arv < 100 * 60 * 60 {
                    write!(f, "{}:{:0>2}.{:0>2}", arv / 60 / 100, (arv / 100) % 60, arv % 100)
                } else {
                    write!(f, "{}:{:0>2}:{:0>2}.{:0>2}", arv / 3600 / 100, (arv / 60 / 100) % 60, (arv / 100) % 60, arv % 100)
                }
            })
        }
    }

    impl Display for AttemptResult<FMCAttemptResultValue> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            self.fmt_or_else(f, &|arv, f|{
                if arv > 80 {
                    write!(f, "{}.{:0<2}", arv / 100, arv % 100)
                } else {
                    write!(f, "{arv}")
                }
            })
        }
    }

    impl Display for AttemptResult<MultiBlindAttemptResultValue> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            self.fmt_or_else(f, &|arv, f|{
                write!(f, "{}/{} ", arv.solved(), arv.attempted())?;
                if arv.seconds() >= 3600 {
                    write!(f, "{}:{:0>2}:{:0>2}", arv.seconds() / 3600, (arv.seconds() % 3600) / 60, arv.seconds() % 60)
                } else {
                    write!(f, "{:0>2}:{:0>2}", arv.seconds() / 60, arv.seconds() % 60)
                }
            })
        }
    }

    impl <ARV: Ord + Eq + Copy> AttemptResult<ARV> {
        fn fmt_or_else<F: Fn(ARV, &mut Formatter<'_>) -> std::fmt::Result>(&self, f: &mut Formatter<'_>, success: &F) -> std::fmt::Result {
            match *self {
                AttemptResult::Skipped => write!(f, ""),
                AttemptResult::DNF => write!(f, "DNF"),
                AttemptResult::DNS => write!(f, "DNS"),
                AttemptResult::Success(arv) => success(arv, f)
            }
        }
    }
}

#[cfg(feature = "parse_activity_code")]
mod activity_code {
    use std::fmt::{Debug, Display, Formatter};
    use std::str::FromStr;
    use serde_with::{DeserializeFromStr, SerializeDisplay};
    use std::cmp::Ordering;

    use crate::types::EventId;
    #[cfg(feature = "parse_puzzle_type")]
    use crate::types::puzzle_types::OfficialEventId;

    #[derive(Clone, Debug, PartialEq, Eq, Hash, SerializeDisplay, DeserializeFromStr)]
    pub enum ActivityCode {
        Official(EventActivityCode<EventId>),
        Unofficial(UnofficialActivityCode)
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash, SerializeDisplay, DeserializeFromStr)]
    pub struct RoundId<EventId: Debug + Display + Clone + FromStr> {
        pub event: EventId,
        pub round: RoundIdType,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct EventActivityCode<EventId: Debug + Display + Clone + FromStr> {
        pub event: EventId,
        pub round: Option<RoundIdType>,
        pub group: Option<GroupIdType>,
        pub attempt: Option<AttemptIdType>,
    }

    impl ActivityCode {
        pub fn is_official(&self) -> bool {
            if let Self::Official(_) = self {
                return true
            }
            return false
        }
    }

    impl <EventId: PartialEq + Debug + Display + Clone + FromStr> PartialEq<EventActivityCode<EventId>> for RoundId<EventId> {
        fn eq(&self, other: &EventActivityCode<EventId>) -> bool {
            let evc: EventActivityCode<EventId> = self.into();
            evc.eq(other)
        }
    }

    impl <EventId: Eq + Debug + Display + Clone + FromStr> PartialOrd for EventActivityCode<EventId> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.event != other.event {
                return None;
            }
            let mut self_more_specific = None;
            if self.round != other.round {
                self_more_specific = match (self.round.is_some(), other.round.is_some()) {
                    (true, true) => return None,
                    (true, false) => Some(true),
                    (false, true) => Some(false),
                    (false, false) => unreachable!()
                }
            }
            if self.group != other.group {
                self_more_specific = match (self.group.is_some(), other.group.is_some(), self_more_specific) {
                    (true, true, _) => return None,
                    (true, false, Some(false)) => return None,
                    (false, true, Some(true)) => return None,
                    (true, false, _) => Some(true),
                    (false, true, _) => Some(false),
                    (false, false, _) => unreachable!()
                }
            }
            if self.attempt != other.attempt {
                match (self.attempt.is_some(), other.attempt.is_some(), self_more_specific) {
                    (true, true, _) => return None,
                    (true, false, Some(false)) => return None,
                    (false, true, Some(true)) => return None,
                    (true, false, _) => return Some(Ordering::Less),
                    (false, true, _) => return Some(Ordering::Greater),
                    (false, false, _) => unreachable!()
                }
            }
            Some(Ordering::Equal)
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub enum UnofficialActivityCode {
        Registration,
        Checkin,
        Tutorial,
        MultiSubmission,
        Breakfast,
        Lunch,
        Dinner,
        Awards,
        Event(EventActivityCode<String>),
        Misc(Option<String>),
        #[deprecated]
        Other(String), //The spec only recommends using misc, but it doesn't require it
    }

    pub type RoundIdType = u32;
    pub type GroupIdType = u32;
    pub type AttemptIdType = u8;

    impl Display for ActivityCode {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match &self {
                ActivityCode::Official(x) => write!(f, "{x}"),
                ActivityCode::Unofficial(x) => write!(f, "other-{x}")
            }
        }
    }

    impl FromStr for ActivityCode {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.starts_with("other-") {
                Ok(ActivityCode::Unofficial(UnofficialActivityCode::from_str(&s[6..])?))
            } else {
                Ok(ActivityCode::Official(EventActivityCode::<EventId>::from_str(s)?))
            }
        }
    }

    impl Display for UnofficialActivityCode {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match &self {
                UnofficialActivityCode::Registration => write!(f, "registration"),
                UnofficialActivityCode::Checkin => write!(f, "checkin"),
                UnofficialActivityCode::Tutorial => write!(f, "tutorial"),
                UnofficialActivityCode::MultiSubmission => write!(f, "multi"),
                UnofficialActivityCode::Breakfast => write!(f, "breakfast"),
                UnofficialActivityCode::Lunch => write!(f, "lunch"),
                UnofficialActivityCode::Dinner => write!(f, "dinner"),
                UnofficialActivityCode::Awards => write!(f, "awards"),
                UnofficialActivityCode::Event(e) => write!(f, "unofficial-{e}"),
                UnofficialActivityCode::Misc(Some(x)) => write!(f, "misc-{x}"),
                UnofficialActivityCode::Misc(None) => write!(f, "misc"),
                #[allow(deprecated)]
                UnofficialActivityCode::Other(x) => write!(f, "{x}"),
            }
        }
    }

    impl FromStr for UnofficialActivityCode {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "registration" => Ok(UnofficialActivityCode::Registration),
                "checkin" => Ok(UnofficialActivityCode::Checkin),
                "tutorial" => Ok(UnofficialActivityCode::Tutorial),
                "multi" => Ok(UnofficialActivityCode::MultiSubmission),
                "breakfast" => Ok(UnofficialActivityCode::Breakfast),
                "lunch" => Ok(UnofficialActivityCode::Lunch),
                "dinner" => Ok(UnofficialActivityCode::Dinner),
                "awards" => Ok(UnofficialActivityCode::Awards),
                "misc" => Ok(UnofficialActivityCode::Misc(None)),
                x if x.starts_with("unofficial-") => Ok(UnofficialActivityCode::Event(EventActivityCode::from_str(&x[11..])?)),
                x if x.starts_with("misc-") => Ok(UnofficialActivityCode::Misc(Some((&x[5..]).to_string()))),
                #[allow(deprecated)]
                x => Ok(UnofficialActivityCode::Other(x.to_string())),
            }
        }
    }

    impl <EventId: Debug + Display + Clone + FromStr> Display for EventActivityCode<EventId> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.event)?;
            if let Some(id) = self.round.as_ref() {
                write!(f, "-r{id}")?;
            }
            if let Some(id) = self.group.as_ref() {
                write!(f, "-g{id}")?;
            }
            if let Some(id) = self.attempt.as_ref() {
                write!(f, "-a{id}")?;
            }
            Ok(())
        }
    }

    impl <EventId: Debug + Display + Clone + FromStr> FromStr for EventActivityCode<EventId> where <EventId as FromStr>::Err: ToString {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.split("-").peekable();
            let event_id = match parts.next() {
                None => return Err("Missing event id".to_string())?,
                Some(x) => EventId::from_str(x).map_err(|x|x.to_string())?
            };

            let round_id = match parts.peek() {
                None => None,
                Some(x) if x.starts_with("r") => Some(RoundIdType::from_str(&(parts.next().unwrap())[1..])
                    .map_err(|x|x.to_string())?),
                _ => None
            };

            let group_id = match parts.peek() {
                None => None,
                Some(x) if x.starts_with("g") => Some(GroupIdType::from_str(&(parts.next().unwrap())[1..])
                    .map_err(|x|x.to_string())?),
                _ => None
            };

            let attempt_id = match parts.next() {
                None => None,
                Some(x) if x.starts_with("a") => Some(AttemptIdType::from_str(&x[1..])
                    .map_err(|x|x.to_string())?),
                _ => None
            };

            Ok(Self {
                event: event_id,
                round: round_id,
                group: group_id,
                attempt: attempt_id,
            })
        }
    }

    impl <EventId: Debug + Display + Clone + FromStr> Display for RoundId<EventId> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}-r{}", self.event, self.round)
        }
    }

    impl <EventId: Debug + Display + Clone + FromStr> FromStr for RoundId<EventId> where <EventId as FromStr>::Err: ToString {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (event, round) = s.split_once("-").ok_or("invalid format".to_string())?;
            let event_id = EventId::from_str(event).map_err(|x|x.to_string())?;
            if !round.starts_with("r") {
                return Err("missing round prefix".to_string());
            }
            let round_id = RoundIdType::from_str(&round[1..])
                .map_err(|x|x.to_string())?;

            Ok(Self {
                event: event_id,
                round: round_id,
            })
        }
    }

    impl <EventId: Debug + Display + Clone + FromStr> From<&RoundId<EventId>> for EventActivityCode<EventId> {
        fn from(value: &RoundId<EventId>) -> Self {
            Self {
                event: value.event.clone(),
                round: Some(value.round.clone()),
                group: None,
                attempt: None,
            }
        }
    }

    impl <EventId: Debug + Display + Clone + FromStr> From<&EventId> for EventActivityCode<EventId> {
        fn from(value: &EventId) -> Self {
            Self {
                event: value.clone(),
                round: None,
                group: None,
                attempt: None,
            }
        }
    }

    #[cfg(feature = "parse_puzzle_type")]
    impl From<&EventActivityCode<OfficialEventId>> for OfficialEventId {
        fn from(value: &EventActivityCode<OfficialEventId>) -> Self {
            value.event.clone()
        }
    }
}

#[cfg(feature = "parse_puzzle_type")]
mod puzzle_types {
    use std::cmp::Ordering;
    use std::fmt::{Debug, Display, Formatter};
    use std::str::FromStr;

    use serde_with::{DeserializeFromStr, SerializeDisplay};

    #[derive(Clone, Debug, Eq, PartialEq, Hash)]
    pub enum OfficialPuzzleType {
        Cube333,
        Cube222,
        Cube444,
        Cube555,
        Cube666,
        Cube777,
        Clock,
        Megaminx,
        Pyraminx,
        Skewb,
        Square1,
        Magic,
        MasterMagic,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Hash, SerializeDisplay, DeserializeFromStr)]
    pub enum OfficialEventId {
        Cube333,
        Cube222,
        Cube444,
        Cube555,
        Cube666,
        Cube777,
        Blind333,
        FewestMoves333,
        OneHanded333,
        Clock,
        Megaminx,
        Pyraminx,
        Skewb,
        Square1,
        Blind444,
        Blind555,
        MultiBlind333,
        Feet333,
        Magic,
        MasterMagic,
        MultiBlindOldStyle333,
    }

    impl OfficialEventId {
        fn ordinal(&self) -> usize {
            match self {
                OfficialEventId::Cube333 => 0,
                OfficialEventId::Cube222 => 1,
                OfficialEventId::Cube444 => 2,
                OfficialEventId::Cube555 => 3,
                OfficialEventId::Cube666 => 4,
                OfficialEventId::Cube777 => 5,
                OfficialEventId::Blind333 => 6,
                OfficialEventId::FewestMoves333 => 7,
                OfficialEventId::OneHanded333 => 8,
                OfficialEventId::Clock => 9,
                OfficialEventId::Megaminx => 10,
                OfficialEventId::Pyraminx => 11,
                OfficialEventId::Skewb => 12,
                OfficialEventId::Square1 => 13,
                OfficialEventId::Blind444 => 14,
                OfficialEventId::Blind555 => 15,
                OfficialEventId::MultiBlind333 => 16,
                OfficialEventId::Feet333 => 17,
                OfficialEventId::Magic => 18,
                OfficialEventId::MasterMagic => 19,
                OfficialEventId::MultiBlindOldStyle333 => 20,
            }
        }
    }

    impl Ord for OfficialEventId {
        fn cmp(&self, other: &Self) -> Ordering {
            self.ordinal().cmp(&other.ordinal())
        }
    }

    impl OfficialEventId {
        pub fn is_official(&self) -> bool {
            match self {
                OfficialEventId::Feet333 => false,
                OfficialEventId::Magic => false,
                OfficialEventId::MasterMagic => false,
                OfficialEventId::MultiBlindOldStyle333 => false,
                _ => true,
            }
        }

        pub fn all() -> [Self; 21] {
            [
                Self::Cube333,
                Self::Cube222,
                Self::Cube444,
                Self::Cube555,
                Self::Cube666,
                Self::Cube777,
                Self::Blind333,
                Self::FewestMoves333,
                Self::OneHanded333,
                Self::Clock,
                Self::Megaminx,
                Self::Pyraminx,
                Self::Skewb,
                Self::Square1,
                Self::Blind444,
                Self::Blind555,
                Self::MultiBlind333,
                Self::Feet333,
                Self::Magic,
                Self::MasterMagic,
                Self::MultiBlindOldStyle333,
            ]
        }

        pub fn all_official() -> [Self; 17] {
            [
                Self::Cube333,
                Self::Cube222,
                Self::Cube444,
                Self::Cube555,
                Self::Cube666,
                Self::Cube777,
                Self::Blind333,
                Self::FewestMoves333,
                Self::OneHanded333,
                Self::Clock,
                Self::Megaminx,
                Self::Pyraminx,
                Self::Skewb,
                Self::Square1,
                Self::Blind444,
                Self::Blind555,
                Self::MultiBlind333,
            ]
        }

        pub fn get_official_name(&self) -> &'static str {
            match &self {
                OfficialEventId::Cube333 => "3x3x3 Cube",
                OfficialEventId::Cube222 => "2x2x2 Cube",
                OfficialEventId::Cube444 => "4x4x4 Cube",
                OfficialEventId::Cube555 => "5x5x5 Cube",
                OfficialEventId::Cube666 => "6x6x6 Cube",
                OfficialEventId::Cube777 => "7x7x7 Cube",
                OfficialEventId::Blind333 => "3x3x3 Blindfolded",
                OfficialEventId::FewestMoves333 => "3x3x3 Fewest Moves",
                OfficialEventId::OneHanded333 => "3x3x3 One-Handed",
                OfficialEventId::Feet333 => "3x3x3 With Feet",
                OfficialEventId::Clock => "Clock",
                OfficialEventId::Megaminx => "Megaminx",
                OfficialEventId::Pyraminx => "Pyraminx",
                OfficialEventId::Skewb => "Skewb",
                OfficialEventId::Square1 => "Square-1",
                OfficialEventId::Blind444 => "4x4x4 Blindfolded",
                OfficialEventId::Blind555 => "5x5x5 Blindfolded",
                OfficialEventId::MultiBlind333 | OfficialEventId::MultiBlindOldStyle333 => "3x3x3 Multi-Blind",
                OfficialEventId::Magic => "Magic",
                OfficialEventId::MasterMagic => "Master Magic",
            }
        }

        pub fn has_average_or_mean(&self) -> bool {
            match self {
                OfficialEventId::MultiBlind333 => false,
                OfficialEventId::MultiBlindOldStyle333 => false,
                _ => true,
            }
        }

        pub fn has_average(&self) -> bool {
            match self {
                OfficialEventId::Cube666 | OfficialEventId::Cube777 => false,
                OfficialEventId::Blind333 | OfficialEventId::Blind444 | OfficialEventId::Blind555 => false,
                OfficialEventId::FewestMoves333 => false,
                OfficialEventId::MultiBlind333 | OfficialEventId::MultiBlindOldStyle333 => false,
                _ => true,
            }
        }

        pub fn has_mean(&self) -> bool {
            match self {
                OfficialEventId::Cube666 | OfficialEventId::Cube777 => true,
                OfficialEventId::Blind333 | OfficialEventId::Blind444 | OfficialEventId::Blind555 => true,
                OfficialEventId::FewestMoves333 => true,
                _ => false,
            }
        }
    }

    impl FromStr for OfficialEventId {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "333" => Ok(OfficialEventId::Cube333),
                "222" => Ok(OfficialEventId::Cube222),
                "444" => Ok(OfficialEventId::Cube444),
                "555" => Ok(OfficialEventId::Cube555),
                "666" => Ok(OfficialEventId::Cube666),
                "777" => Ok(OfficialEventId::Cube777),
                "333bf" => Ok(OfficialEventId::Blind333),
                "333fm" => Ok(OfficialEventId::FewestMoves333),
                "333oh" => Ok(OfficialEventId::OneHanded333),
                "333ft" => Ok(OfficialEventId::Feet333),
                "clock" => Ok(OfficialEventId::Clock),
                "minx" => Ok(OfficialEventId::Megaminx),
                "pyram" => Ok(OfficialEventId::Pyraminx),
                "skewb" => Ok(OfficialEventId::Skewb),
                "sq1" => Ok(OfficialEventId::Square1),
                "444bf" => Ok(OfficialEventId::Blind444),
                "555bf" => Ok(OfficialEventId::Blind555),
                "333mbf" => Ok(OfficialEventId::MultiBlind333),
                "magic" => Ok(OfficialEventId::Magic),
                "mmagic" => Ok(OfficialEventId::MasterMagic),
                "333mbo" => Ok(OfficialEventId::MultiBlindOldStyle333),
                _ => Err("Not a valid event".to_string())
            }
        }
    }

    impl Display for OfficialEventId {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match &self {
                OfficialEventId::Cube333 => "333",
                OfficialEventId::Cube222 => "222",
                OfficialEventId::Cube444 => "444",
                OfficialEventId::Cube555 => "555",
                OfficialEventId::Cube666 => "666",
                OfficialEventId::Cube777 => "777",
                OfficialEventId::Blind333 => "333bf",
                OfficialEventId::FewestMoves333 => "333fm",
                OfficialEventId::OneHanded333 => "333oh",
                OfficialEventId::Feet333 => "333ft",
                OfficialEventId::Clock => "clock",
                OfficialEventId::Megaminx => "minx",
                OfficialEventId::Pyraminx => "pyram",
                OfficialEventId::Skewb => "skewb",
                OfficialEventId::Square1 => "sq1",
                OfficialEventId::Blind444 => "444bf",
                OfficialEventId::Blind555 => "555bf",
                OfficialEventId::MultiBlind333 => "333mbf",
                OfficialEventId::Magic => "magic",
                OfficialEventId::MasterMagic => "mmagic",
                OfficialEventId::MultiBlindOldStyle333 => "333mbo",
            })
        }
    }


    impl OfficialEventId {
        pub fn is_blind(&self) -> bool {
            match self {
                Self::Blind333 | Self::Blind444 | Self::Blind555 | Self::MultiBlind333 | Self::MultiBlindOldStyle333 => true,
                _ => false
            }
        }

        pub fn get_puzzle_type(&self) -> OfficialPuzzleType {
            match self {
                Self::Cube333 | Self::OneHanded333 | Self::Blind333 | Self::Feet333 | Self::FewestMoves333 | Self::MultiBlind333 | Self::MultiBlindOldStyle333 => OfficialPuzzleType::Cube333,
                Self::Cube222 => OfficialPuzzleType::Cube222,
                Self::Cube444 | Self::Blind444 => OfficialPuzzleType::Cube444,
                Self::Cube555 | Self::Blind555 => OfficialPuzzleType::Cube555,
                Self::Cube666 => OfficialPuzzleType::Cube666,
                Self::Cube777 => OfficialPuzzleType::Cube777,
                Self::Clock => OfficialPuzzleType::Clock,
                Self::Megaminx => OfficialPuzzleType::Megaminx,
                Self::Pyraminx => OfficialPuzzleType::Pyraminx,
                Self::Skewb => OfficialPuzzleType::Skewb,
                Self::Square1 => OfficialPuzzleType::Square1,
                Self::Magic => OfficialPuzzleType::Magic,
                Self::MasterMagic => OfficialPuzzleType::MasterMagic
            }
        }
    }
}