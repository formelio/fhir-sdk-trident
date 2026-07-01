//! Resource definitions.

#[rustfmt::skip] // Too much for rustfmt
#[allow(clippy::doc_lazy_continuation, reason = "Comments from FHIR spec")]
mod generated;

pub use generated::*;

use crate::identifiable_resource::identifiable_resource_code;

identifiable_resource_code!(
	r4b,
	[
		ActivityDefinition,
		Basic,
		CarePlan,
		CareTeam,
		CatalogEntry,
		CodeSystem,
		DeviceUseStatement,
		DocumentManifest,
		Encounter,
		Media,
		MedicationRequest,
		Observation,
		ObservationDefinition,
		Patient,
		PlanDefinition,
		Practitioner,
		PractitionerRole,
		Procedure,
		Questionnaire,
		RequestGroup,
		ResearchDefinition,
		ResearchElementDefinition,
		StructureDefinition,
		Substance,
		Task
	]
);
