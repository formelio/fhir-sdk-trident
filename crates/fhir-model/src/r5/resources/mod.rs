//! Resource definitions.

#[rustfmt::skip] // Too much for rustfmt
#[allow(clippy::doc_lazy_continuation, reason = "Comments from FHIR spec")]
mod generated;

pub use generated::*;

use crate::identifiable_resource::identifiable_resource_code;

identifiable_resource_code!(
	r5,
	[
		ActivityDefinition,
		Basic,
		CapabilityStatement,
		CarePlan,
		CareTeam,
		CodeSystem,
		Encounter,
		MedicationRequest,
		Observation,
		Patient,
		PlanDefinition,
		Practitioner,
		PractitionerRole,
		Procedure,
		Questionnaire,
		QuestionnaireResponse,
		RequestOrchestration,
		StructureDefinition,
		Substance,
		Task
	]
);
