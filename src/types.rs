use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use monostate::MustBe;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::TimeDelta;
use serde_with::{DeserializeFromStr, SerializeDisplay};

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

// Officially recognized by the spec as a string, might be replaced by an enum
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
pub type AttemptResult = attempt_result::AttemptResult;
pub type AttemptResultValue = u32;

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
    pub competitor_limit: Option<u32>,
    pub extensions: Vec<Extension>
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
    #[cfg(feature = "private_properties")]
    pub birthdate: chrono::NaiveDate,
    #[cfg(feature = "private_properties")]
    pub email: String,
    pub avatar: Option<Avatar>,
    pub roles: Vec<Role>,
    pub registration: Option<Registration>,
    pub assignments: Vec<Assignment>,
    pub personal_bests: Vec<PersonalBest>,
    pub extensions: Vec<Extension>
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

#[derive(Clone, PartialEq, Hash, SerializeDisplay, DeserializeFromStr)]
pub struct WCAId {
    pub year: u16,
    pub discriminant: u8,
    pub name: String
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
    #[cfg(feature = "private_properties")]
    pub guests: u32,
    #[cfg(feature = "private_properties")]
    pub comments: String,
    #[cfg(feature = "private_properties")]
    pub administrative_notes: String,
    pub is_competing: bool,
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
    AnyResult,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundResult {
    pub person_id: PersonId,
    pub ranking: Option<u64>,
    pub attempts: Vec<Attempt>,
    pub best: AttemptResult,
    pub average: AttemptResult,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
pub struct Activity {
    pub id: ActivityId,
    pub name: String,
    pub activity_code: ActivityCode,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub child_activities: Vec<Activity>,
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
    #[cfg(feature = "groupifier")]
    #[serde(untagged)]
    GroupifierCompetitionConfig(crate::groupifier::CompetitionConfigExtension),
    #[cfg(feature = "groupifier")]
    #[serde(untagged)]
    GroupifierActivityConfig(crate::groupifier::ActivityConfigExtension),
    #[cfg(feature = "groupifier")]
    #[serde(untagged)]
    GroupifierRoomConfig(crate::groupifier::RoomConfigExtension),
    #[cfg(feature = "delegate_dashboard")]
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
    use serde::{Serializer};
    use serde::de::Error;
    use serde_json::Value;
    use crate::types::AttemptResultValue;

    #[derive(Copy, Clone, PartialEq, Debug, Hash)]
    pub enum AttemptResult {
        Skipped,
        DNF,
        DNS,
        Success(AttemptResultValue),
    }

    impl<'de> serde::Deserialize<'de> for AttemptResult {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = Value::deserialize(d)?;

            Ok(match value.as_i64().ok_or(Error::custom("Not a number"))? {
                -2 => AttemptResult::DNS,
                -1 => AttemptResult::DNF,
                0 => AttemptResult::Skipped,
                x if x > 0 => AttemptResult::Success(x as u32),
                _ => Err(Error::custom("not a valid result"))?,
            })
        }
    }

    impl serde::Serialize for AttemptResult {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
        {
            match &self {
                AttemptResult::Skipped => serializer.serialize_i64(0),
                AttemptResult::DNF => serializer.serialize_i64(-1),
                AttemptResult::DNS => serializer.serialize_i64(-2),
                AttemptResult::Success(x) => serializer.serialize_i64(*x as i64),
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

    #[derive(Clone, Debug, Eq, PartialEq, Hash, SerializeDisplay, DeserializeFromStr)]
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
        Feet333,
        Clock,
        Megaminx,
        Pyraminx,
        Skewb,
        Square1,
        Blind444,
        Blind555,
        MultiBlind333,
        Magic,
        MasterMagic,
        MultiBlindOldStyle333,
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