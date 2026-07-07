//! Generated code! Take a look at the generator-crate for changing this file!
#![allow(clippy::too_many_lines, reason = "Generated code")]
use serde::{Serialize, Deserialize};
use super::super::types::{Coding, CodingInner, CodeableConcept, CodeableConceptInner};
#[doc = "**[AdministrativeGender](http://hl7.org/fhir/ValueSet/administrative-gender)**. The gender of a person used for administrative purposes.\n\nFHIR version: 4.3.0."]
#[derive(PartialEq, Eq, Clone, Hash)]
#[derive(Copy)]
pub enum AdministrativeGender {
    /** **female**

Female. Female. */
    Female,
    /** **male**

Male. Male. */
    Male,
    /** **other**

Other. Other. */
    Other,
    /** **unknown**

Unknown. Unknown. */
    Unknown,
}
impl ::core::str::FromStr for AdministrativeGender {
    type Err = String;
    #[allow(
        clippy::match_single_binding,
        reason = "Generated code; unknown number of variants"
    )]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "female" => Ok(Self::Female),
            "male" => Ok(Self::Male),
            "other" => Ok(Self::Other),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for AdministrativeGender {
    fn as_ref(&self) -> &str {
        match self {
            Self::Female => "female",
            Self::Male => "male",
            Self::Other => "other",
            Self::Unknown => "unknown",
        }
    }
}
impl ::std::fmt::Debug for AdministrativeGender {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for AdministrativeGender {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for AdministrativeGender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for AdministrativeGender {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<AdministrativeGender> for Coding {
    fn from(code: AdministrativeGender) -> Self {
        CodingInner {
            system: Some(
                "http://hl7.org/fhir/ValueSet/administrative-gender".to_owned(),
            ),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<AdministrativeGender> for CodeableConcept {
    fn from(code: AdministrativeGender) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[BundleType](http://hl7.org/fhir/ValueSet/bundle-type)**. Indicates the purpose of a bundle - how it is intended to be used.\n\nFHIR version: 4.3.0."]
#[derive(PartialEq, Eq, Clone, Hash)]
#[derive(Copy)]
pub enum BundleType {
    /** **batch**

Batch. The bundle is a set of actions - intended to be processed by a server as a group of independent actions. */
    Batch,
    /** **batch-response**

Batch Response. The bundle is a batch response. Note that as a batch, some responses may indicate failure and others success. */
    BatchResponse,
    /** **collection**

Collection. The bundle is a set of resources collected into a single package for ease of distribution that imposes no processing obligations or behavioral rules beyond persistence. */
    Collection,
    /** **document**

Document. The bundle is a document. The first resource is a Composition. */
    Document,
    /** **history**

History List. The bundle is a list of resources from a history interaction on a server. */
    History,
    /** **message**

Message. The bundle is a message. The first resource is a MessageHeader. */
    Message,
    /** **searchset**

Search Results. The bundle is a list of resources returned as a result of a search/query interaction, operation, or message. */
    Searchset,
    /** **transaction**

Transaction. The bundle is a transaction - intended to be processed by a server as an atomic commit. */
    Transaction,
    /** **transaction-response**

Transaction Response. The bundle is a transaction response. Because the response is a transaction response, the transaction has succeeded, and all responses are error free. */
    TransactionResponse,
}
impl ::core::str::FromStr for BundleType {
    type Err = String;
    #[allow(
        clippy::match_single_binding,
        reason = "Generated code; unknown number of variants"
    )]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "batch" => Ok(Self::Batch),
            "batch-response" => Ok(Self::BatchResponse),
            "collection" => Ok(Self::Collection),
            "document" => Ok(Self::Document),
            "history" => Ok(Self::History),
            "message" => Ok(Self::Message),
            "searchset" => Ok(Self::Searchset),
            "transaction" => Ok(Self::Transaction),
            "transaction-response" => Ok(Self::TransactionResponse),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for BundleType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Batch => "batch",
            Self::BatchResponse => "batch-response",
            Self::Collection => "collection",
            Self::Document => "document",
            Self::History => "history",
            Self::Message => "message",
            Self::Searchset => "searchset",
            Self::Transaction => "transaction",
            Self::TransactionResponse => "transaction-response",
        }
    }
}
impl ::std::fmt::Debug for BundleType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for BundleType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for BundleType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for BundleType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<BundleType> for Coding {
    fn from(code: BundleType) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/bundle-type".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<BundleType> for CodeableConcept {
    fn from(code: BundleType) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[EncounterStatus](http://hl7.org/fhir/ValueSet/encounter-status)**. Current state of the encounter.\n\nFHIR version: 4.3.0."]
#[derive(PartialEq, Eq, Clone, Hash)]
#[derive(Copy)]
pub enum EncounterStatus {
    /** **arrived**

Arrived. The Patient is present for the encounter, however is not currently meeting with a practitioner. */
    Arrived,
    /** **cancelled**

Cancelled. The Encounter has ended before it has begun. */
    Cancelled,
    /** **entered-in-error**

Entered in Error. This instance should not have been part of this patient's medical record. */
    EnteredInError,
    /** **finished**

Finished. The Encounter has ended. */
    Finished,
    /** **in-progress**

In Progress. The Encounter has begun and the patient is present / the practitioner and the patient are meeting. */
    InProgress,
    /** **onleave**

On Leave. The Encounter has begun, but the patient is temporarily on leave. */
    Onleave,
    /** **planned**

Planned. The Encounter has not yet started. */
    Planned,
    /** **triaged**

Triaged. The patient has been assessed for the priority of their treatment based on the severity of their condition. */
    Triaged,
    /** **unknown**

Unknown. The encounter status is unknown. Note that "unknown" is a value of last resort and every attempt should be made to provide a meaningful value other than "unknown". */
    Unknown,
}
impl ::core::str::FromStr for EncounterStatus {
    type Err = String;
    #[allow(
        clippy::match_single_binding,
        reason = "Generated code; unknown number of variants"
    )]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "arrived" => Ok(Self::Arrived),
            "cancelled" => Ok(Self::Cancelled),
            "entered-in-error" => Ok(Self::EnteredInError),
            "finished" => Ok(Self::Finished),
            "in-progress" => Ok(Self::InProgress),
            "onleave" => Ok(Self::Onleave),
            "planned" => Ok(Self::Planned),
            "triaged" => Ok(Self::Triaged),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for EncounterStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Arrived => "arrived",
            Self::Cancelled => "cancelled",
            Self::EnteredInError => "entered-in-error",
            Self::Finished => "finished",
            Self::InProgress => "in-progress",
            Self::Onleave => "onleave",
            Self::Planned => "planned",
            Self::Triaged => "triaged",
            Self::Unknown => "unknown",
        }
    }
}
impl ::std::fmt::Debug for EncounterStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for EncounterStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for EncounterStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for EncounterStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<EncounterStatus> for Coding {
    fn from(code: EncounterStatus) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/encounter-status".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<EncounterStatus> for CodeableConcept {
    fn from(code: EncounterStatus) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[HTTPVerb](http://hl7.org/fhir/ValueSet/http-verb)**. HTTP verbs (in the HTTP command line). See [HTTP rfc](https://tools.ietf.org/html/rfc7231) for details.\n\nFHIR version: 4.3.0."]
#[derive(PartialEq, Eq, Clone, Hash)]
#[derive(Copy)]
pub enum HTTPVerb {
    /** **DELETE**

DELETE. HTTP DELETE Command. */
    Delete,
    /** **GET**

GET. HTTP GET Command. */
    Get,
    /** **HEAD**

HEAD. HTTP HEAD Command. */
    Head,
    /** **PATCH**

PATCH. HTTP PATCH Command. */
    Patch,
    /** **POST**

POST. HTTP POST Command. */
    Post,
    /** **PUT**

PUT. HTTP PUT Command. */
    Put,
}
impl ::core::str::FromStr for HTTPVerb {
    type Err = String;
    #[allow(
        clippy::match_single_binding,
        reason = "Generated code; unknown number of variants"
    )]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "DELETE" => Ok(Self::Delete),
            "GET" => Ok(Self::Get),
            "HEAD" => Ok(Self::Head),
            "PATCH" => Ok(Self::Patch),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for HTTPVerb {
    fn as_ref(&self) -> &str {
        match self {
            Self::Delete => "DELETE",
            Self::Get => "GET",
            Self::Head => "HEAD",
            Self::Patch => "PATCH",
            Self::Post => "POST",
            Self::Put => "PUT",
        }
    }
}
impl ::std::fmt::Debug for HTTPVerb {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for HTTPVerb {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for HTTPVerb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for HTTPVerb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<HTTPVerb> for Coding {
    fn from(code: HTTPVerb) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/http-verb".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<HTTPVerb> for CodeableConcept {
    fn from(code: HTTPVerb) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[IssueSeverity](http://hl7.org/fhir/ValueSet/issue-severity)**. How the issue affects the success of the action.\n\nFHIR version: 4.3.0."]
#[derive(PartialEq, Eq, Clone, Hash)]
#[derive(Copy)]
pub enum IssueSeverity {
    /** **error**

Error. The issue is sufficiently important to cause the action to fail. */
    Error,
    /** **fatal**

Fatal. The issue caused the action to fail and no further checking could be performed. */
    Fatal,
    /** **information**

Information. The issue has no relation to the degree of success of the action. */
    Information,
    /** **warning**

Warning. The issue is not important enough to cause the action to fail but may cause it to be performed suboptimally or in a way that is not as desired. */
    Warning,
}
impl ::core::str::FromStr for IssueSeverity {
    type Err = String;
    #[allow(
        clippy::match_single_binding,
        reason = "Generated code; unknown number of variants"
    )]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "error" => Ok(Self::Error),
            "fatal" => Ok(Self::Fatal),
            "information" => Ok(Self::Information),
            "warning" => Ok(Self::Warning),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for IssueSeverity {
    fn as_ref(&self) -> &str {
        match self {
            Self::Error => "error",
            Self::Fatal => "fatal",
            Self::Information => "information",
            Self::Warning => "warning",
        }
    }
}
impl ::std::fmt::Debug for IssueSeverity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for IssueSeverity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for IssueSeverity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<IssueSeverity> for Coding {
    fn from(code: IssueSeverity) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/issue-severity".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<IssueSeverity> for CodeableConcept {
    fn from(code: IssueSeverity) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[RequestIntent](http://hl7.org/fhir/ValueSet/request-intent)**. Codes indicating the degree of authority/intentionality associated with a request.\n\nFHIR version: 4.3.0."]
#[derive(PartialEq, Eq, Clone, Hash)]
#[derive(Copy)]
pub enum RequestIntent {
    /** **directive**

Directive. The request represents a legally binding instruction authored by a Patient or RelatedPerson. */
    Directive,
    /** **filler-order**

Filler Order. The request represents the view of an authorization instantiated by a fulfilling system representing the details of the fulfiller's intention to act upon a submitted order. */
    FillerOrder,
    /** **instance-order**

Instance Order. An order created in fulfillment of a broader order that represents the authorization for a single activity occurrence.  E.g. The administration of a single dose of a drug. */
    InstanceOrder,
    /** **option**

Option. The request represents a component or option for a RequestGroup that establishes timing, conditionality and/or other constraints among a set of requests.  Refer to [[[RequestGroup]]] for additional information on how this status is used. */
    Option,
    /** **order**

Order. The request represents a request/demand and authorization for action by a Practitioner. */
    Order,
    /** **original-order**

Original Order. The request represents an original authorization for action. */
    OriginalOrder,
    /** **plan**

Plan. The request represents an intention to ensure something occurs without providing an authorization for others to act. */
    Plan,
    /** **proposal**

Proposal. The request is a suggestion made by someone/something that does not have an intention to ensure it occurs and without providing an authorization to act. */
    Proposal,
    /** **reflex-order**

Reflex Order. The request represents an automatically generated supplemental authorization for action based on a parent authorization together with initial results of the action taken against that parent authorization. */
    ReflexOrder,
}
impl ::core::str::FromStr for RequestIntent {
    type Err = String;
    #[allow(
        clippy::match_single_binding,
        reason = "Generated code; unknown number of variants"
    )]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "directive" => Ok(Self::Directive),
            "filler-order" => Ok(Self::FillerOrder),
            "instance-order" => Ok(Self::InstanceOrder),
            "option" => Ok(Self::Option),
            "order" => Ok(Self::Order),
            "original-order" => Ok(Self::OriginalOrder),
            "plan" => Ok(Self::Plan),
            "proposal" => Ok(Self::Proposal),
            "reflex-order" => Ok(Self::ReflexOrder),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for RequestIntent {
    fn as_ref(&self) -> &str {
        match self {
            Self::Directive => "directive",
            Self::FillerOrder => "filler-order",
            Self::InstanceOrder => "instance-order",
            Self::Option => "option",
            Self::Order => "order",
            Self::OriginalOrder => "original-order",
            Self::Plan => "plan",
            Self::Proposal => "proposal",
            Self::ReflexOrder => "reflex-order",
        }
    }
}
impl ::std::fmt::Debug for RequestIntent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for RequestIntent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for RequestIntent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for RequestIntent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<RequestIntent> for Coding {
    fn from(code: RequestIntent) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/request-intent".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<RequestIntent> for CodeableConcept {
    fn from(code: RequestIntent) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[RequestStatus](http://hl7.org/fhir/ValueSet/request-status)**. Codes identifying the lifecycle stage of a request.\n\nFHIR version: 4.3.0."]
#[derive(PartialEq, Eq, Clone, Hash)]
#[derive(Copy)]
pub enum RequestStatus {
    /** **active**

Active. The request is in force and ready to be acted upon. */
    Active,
    /** **completed**

Completed. The activity described by the request has been fully performed.  No further activity will occur. */
    Completed,
    /** **draft**

Draft. The request has been created but is not yet complete or ready for action. */
    Draft,
    /** **entered-in-error**

Entered in Error. This request should never have existed and should be considered 'void'.  (It is possible that real-world decisions were based on it.  If real-world activity has occurred, the status should be "revoked" rather than "entered-in-error".). */
    EnteredInError,
    /** **on-hold**

On Hold. The request (and any implicit authorization to act) has been temporarily withdrawn but is expected to resume in the future. */
    OnHold,
    /** **revoked**

Revoked. The request (and any implicit authorization to act) has been terminated prior to the known full completion of the intended actions.  No further activity should occur. */
    Revoked,
    /** **unknown**

Unknown. The authoring/source system does not know which of the status values currently applies for this request.  Note: This concept is not to be used for "other" - one of the listed statuses is presumed to apply,  but the authoring/source system does not know which. */
    Unknown,
}
impl ::core::str::FromStr for RequestStatus {
    type Err = String;
    #[allow(
        clippy::match_single_binding,
        reason = "Generated code; unknown number of variants"
    )]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "completed" => Ok(Self::Completed),
            "draft" => Ok(Self::Draft),
            "entered-in-error" => Ok(Self::EnteredInError),
            "on-hold" => Ok(Self::OnHold),
            "revoked" => Ok(Self::Revoked),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for RequestStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Completed => "completed",
            Self::Draft => "draft",
            Self::EnteredInError => "entered-in-error",
            Self::OnHold => "on-hold",
            Self::Revoked => "revoked",
            Self::Unknown => "unknown",
        }
    }
}
impl ::std::fmt::Debug for RequestStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for RequestStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for RequestStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for RequestStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<RequestStatus> for Coding {
    fn from(code: RequestStatus) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/request-status".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<RequestStatus> for CodeableConcept {
    fn from(code: RequestStatus) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[RiskProbability](http://terminology.hl7.org/CodeSystem/risk-probability)**. Codes representing the likelihood of a particular outcome in a risk assessment.\n\nFHIR version: 4.3.0."]
#[derive(PartialEq, Eq, Clone, Hash)]
pub enum RiskProbability {
    /** **certain**

Certain. The specified outcome is effectively guaranteed. */
    Certain,
    /** **high**

High likelihood. The specified outcome is more likely to occur than not. */
    High,
    /** **low**

Low likelihood. The specified outcome is possible but unlikely. */
    Low,
    /** **moderate**

Moderate likelihood. The specified outcome has a reasonable likelihood of occurrence. */
    Moderate,
    /** **negligible**

Negligible likelihood. The specified outcome is exceptionally unlikely. */
    Negligible,
    /// Custom code value.
    _Custom(String),
}
impl ::core::str::FromStr for RiskProbability {
    type Err = String;
    #[allow(
        clippy::match_single_binding,
        reason = "Generated code; unknown number of variants"
    )]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "certain" => Ok(Self::Certain),
            "high" => Ok(Self::High),
            "low" => Ok(Self::Low),
            "moderate" => Ok(Self::Moderate),
            "negligible" => Ok(Self::Negligible),
            _ => Ok(Self::_Custom(s.to_owned())),
        }
    }
}
impl AsRef<str> for RiskProbability {
    fn as_ref(&self) -> &str {
        match self {
            Self::Certain => "certain",
            Self::High => "high",
            Self::Low => "low",
            Self::Moderate => "moderate",
            Self::Negligible => "negligible",
            Self::_Custom(s) => s.as_str(),
        }
    }
}
impl ::std::fmt::Debug for RiskProbability {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for RiskProbability {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for RiskProbability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for RiskProbability {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<RiskProbability> for Coding {
    fn from(code: RiskProbability) -> Self {
        CodingInner {
            system: Some(
                "http://terminology.hl7.org/CodeSystem/risk-probability".to_owned(),
            ),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<RiskProbability> for CodeableConcept {
    fn from(code: RiskProbability) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[SearchComparator](http://hl7.org/fhir/ValueSet/search-comparator)**. What Search Comparator Codes are supported in search.\n\nFHIR version: 4.3.0."]
#[derive(PartialEq, Eq, Clone, Hash)]
#[derive(Copy)]
pub enum SearchComparator {
    /** **ap**

Approximately. the value for the parameter in the resource is approximately the same to the provided value. */
    Ap,
    /** **eb**

Ends Before. the value for the parameter in the resource ends before the provided value. */
    Eb,
    /** **eq**

Equals. the value for the parameter in the resource is equal to the provided value. */
    Eq,
    /** **ge**

Greater or Equals. the value for the parameter in the resource is greater or equal to the provided value. */
    Ge,
    /** **gt**

Greater Than. the value for the parameter in the resource is greater than the provided value. */
    Gt,
    /** **le**

Less of Equal. the value for the parameter in the resource is less or equal to the provided value. */
    Le,
    /** **lt**

Less Than. the value for the parameter in the resource is less than the provided value. */
    Lt,
    /** **ne**

Not Equals. the value for the parameter in the resource is not equal to the provided value. */
    Ne,
    /** **sa**

Starts After. the value for the parameter in the resource starts after the provided value. */
    Sa,
}
impl ::core::str::FromStr for SearchComparator {
    type Err = String;
    #[allow(
        clippy::match_single_binding,
        reason = "Generated code; unknown number of variants"
    )]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "ap" => Ok(Self::Ap),
            "eb" => Ok(Self::Eb),
            "eq" => Ok(Self::Eq),
            "ge" => Ok(Self::Ge),
            "gt" => Ok(Self::Gt),
            "le" => Ok(Self::Le),
            "lt" => Ok(Self::Lt),
            "ne" => Ok(Self::Ne),
            "sa" => Ok(Self::Sa),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for SearchComparator {
    fn as_ref(&self) -> &str {
        match self {
            Self::Ap => "ap",
            Self::Eb => "eb",
            Self::Eq => "eq",
            Self::Ge => "ge",
            Self::Gt => "gt",
            Self::Le => "le",
            Self::Lt => "lt",
            Self::Ne => "ne",
            Self::Sa => "sa",
        }
    }
}
impl ::std::fmt::Debug for SearchComparator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for SearchComparator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for SearchComparator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for SearchComparator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<SearchComparator> for Coding {
    fn from(code: SearchComparator) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/search-comparator".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<SearchComparator> for CodeableConcept {
    fn from(code: SearchComparator) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[SearchEntryMode](http://hl7.org/fhir/ValueSet/search-entry-mode)**. Why an entry is in the result set - whether it's included as a match or because of an _include requirement, or to convey information or warning information about the search process.\n\nFHIR version: 4.3.0."]
#[derive(PartialEq, Eq, Clone, Hash)]
#[derive(Copy)]
pub enum SearchEntryMode {
    /** **include**

Include. This resource is returned because it is referred to from another resource in the search set. */
    Include,
    /** **match**

Match. This resource matched the search specification. */
    Match,
    /** **outcome**

Outcome. An OperationOutcome that provides additional information about the processing of a search. */
    Outcome,
}
impl ::core::str::FromStr for SearchEntryMode {
    type Err = String;
    #[allow(
        clippy::match_single_binding,
        reason = "Generated code; unknown number of variants"
    )]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "include" => Ok(Self::Include),
            "match" => Ok(Self::Match),
            "outcome" => Ok(Self::Outcome),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for SearchEntryMode {
    fn as_ref(&self) -> &str {
        match self {
            Self::Include => "include",
            Self::Match => "match",
            Self::Outcome => "outcome",
        }
    }
}
impl ::std::fmt::Debug for SearchEntryMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for SearchEntryMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for SearchEntryMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for SearchEntryMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<SearchEntryMode> for Coding {
    fn from(code: SearchEntryMode) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/search-entry-mode".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<SearchEntryMode> for CodeableConcept {
    fn from(code: SearchEntryMode) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
