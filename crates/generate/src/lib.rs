//! Code generation for FHIR types.
#![allow(clippy::expect_used, clippy::print_stdout)] // Just a generator crate.

mod generate;
mod model;
mod parse;

use ::neuer_error::{ConvertResult, Result};
use ::proc_macro2::TokenStream;
use ::std::fs;

use crate::model::StructureDefinitionKind;

const IGNORED_TYPES: &[&str] = &[
	"Account",
	// "ActivityDefinition",
	"ActorDefinition",
	"AdministrableProductDefinition",
	"AdverseEvent",
	"AllergyIntolerance",
	"Appointment",
	"AppointmentResponse",
	"ArtifactAssessment",
	// "Basic",
	"BiologicallyDerivedProduct",
	"BiologicallyDerivedProductDispense",
	"BodyStructure",
	// "CapabilityStatement",
	// "CarePlan",
	// "CareTeam",
	"ChargeItem",
	"ChargeItemDefinition",
	"Citation",
	"Claim",
	"ClaimResponse",
	"ClinicalImpression",
	"ClinicalUseDefinition",
	// "CodeSystem",
	"Communication",
	"CommunicationRequest",
	"Composition",
	"ConceptMap",
	"Condition",
	"ConditionDefinition",
	"Consent",
	"Contract",
	"Coverage",
	"CoverageEligibilityRequest",
	"CoverageEligibilityResponse",
	"DetectedIssue",
	"Device",
	"DeviceAssociation",
	"DeviceDefinition",
	"DeviceDispense",
	"DeviceMetric",
	"DeviceRequest",
	"DeviceUsage",
	"DiagnosticReport",
	"DocumentReference",
	// "Encounter",
	"EncounterHistory",
	"Endpoint",
	"EnrollmentRequest",
	"EnrollmentResponse",
	"EpisodeOfCare",
	"EventDefinition",
	"Evidence",
	"EvidenceReport",
	"EvidenceVariable",
	"ExampleScenario",
	"ExplanationOfBenefit",
	"FamilyMemberHistory",
	"Flag",
	"FormularyItem",
	"GenomicStudy",
	"Goal",
	"GraphDefinition",
	"Group",
	"GuidanceResponse",
	"HealthcareService",
	"ImagingSelection",
	"ImagingStudy",
	"Immunization",
	"ImmunizationEvaluation",
	"ImmunizationRecommendation",
	"ImplementationGuide",
	"InsurancePlan",
	"InventoryItem",
	"InventoryReport",
	"Invoice",
	"Library",
	"List",
	"Location",
	"ManufacturedItemDefinition",
	"Measure",
	"MeasureReport",
	"Medication",
	"MedicationAdministration",
	"MedicationDispense",
	"MedicationKnowledge",
	// "MedicationRequest",
	"MedicationStatement",
	"MedicinalProductDefinition",
	"MessageDefinition",
	"MolecularSequence",
	"NamingSystem",
	"NutritionIntake",
	"NutritionOrder",
	// "Observation",
	"OperationDefinition",
	"Organization",
	"OrganizationAffiliation",
	"PackagedProductDefinition",
	// "Patient",
	"PaymentNotice",
	"PaymentReconciliation",
	"Person",
	// "PlanDefinition",
	// "Practitioner",
	// "PractitionerRole",
	// "Procedure",
	// "Questionnaire",
	// "QuestionnaireResponse",
	"RegulatedAuthorization",
	"RelatedPerson",
	// "RequestOrchestration",
	"Requirements",
	"ResearchStudy",
	"ResearchSubject",
	"RiskAssessment",
	"Schedule",
	"SearchParameter",
	"ServiceRequest",
	"Slot",
	"Specimen",
	// "StructureDefinition",
	"StructureMap",
	"Subscription",
	"SubscriptionTopic",
	// "Substance",
	"SubstanceDefinition",
	"SupplyDelivery",
	"SupplyRequest",
	// "Task",
	"TerminologyCapabilities",
	"TestPlan",
	"TestScript",
	"Transport",
	"ValueSet",
	"VisionPrescription",
];

/// Generate code for a FHIR version. Must match the folder name for the input
/// data and the output folder name.
pub fn generate_code(version_folder: &str) -> Result<()> {
	let base_folder = env!("CARGO_MANIFEST_DIR");

	let codes_file =
		fs::read_to_string(format!("{base_folder}/definitions/{version_folder}/valuesets.json"))?;
	let codes = match version_folder {
		"r4b" => parse::codes::parse_r4b(&codes_file),
		"r5" => parse::codes::parse_r5(&codes_file),
		_ => panic!("Unrecognized version `{version_folder}`"),
	};
	let (generated_code, generated_codes) = generate::generate_codes(codes)?;
	fs::write(
		format!("{base_folder}/../fhir-model/src/{version_folder}/codes/generated.rs"),
		format_code(generated_code)?,
	)?;

	let types_file = fs::read_to_string(format!(
		"{base_folder}/definitions/{version_folder}/profiles-types.json"
	))?;
	let mut types = match version_folder {
		"r4b" => parse::structures::parse_r4b(&types_file),
		"r5" => parse::structures::parse_r5(&types_file),
		_ => panic!("Unrecognized version `{version_folder}`"),
	};
	types.retain(|x| !IGNORED_TYPES.contains(&x.name.as_str()));
	let generated_code = generate::generate_types(types, &generated_codes)?;
	fs::write(
		format!("{base_folder}/../fhir-model/src/{version_folder}/types/generated.rs"),
		format_code(generated_code)?,
	)?;

	let resources_file = fs::read_to_string(format!(
		"{base_folder}/definitions/{version_folder}/profiles-resources.json"
	))?;
	let mut resources = match version_folder {
		"r4b" => parse::structures::parse_r4b(&resources_file),
		"r5" => parse::structures::parse_r5(&resources_file),
		_ => panic!("Unrecognized version `{version_folder}`"),
	};

	resources.retain(|x| !IGNORED_TYPES.contains(&x.name.as_str()));

	let identifiable = resources
		.iter()
		.filter(|ty| !ty.r#abstract)
		.filter(|ty| ty.kind == StructureDefinitionKind::Resource)
		.filter_map(|ty| {
			ty.elements
				.fields
				.iter()
				.any(|field| field.name() == "identifier" && field.is_array())
				.then_some(&ty.name)
		})
		.collect::<Vec<_>>();
	println!("Identifiable resources: {identifiable:?}");

	let generated_code = generate::generate_resources(resources, &generated_codes)?;
	fs::write(
		format!("{base_folder}/../fhir-model/src/{version_folder}/resources/generated.rs"),
		format_code(generated_code)?,
	)?;

	Ok(())
}

/// Convert a TokenStream to formatted Rust code.
fn format_code(code: TokenStream) -> Result<String> {
	let parsed = syn::parse2::<syn::File>(code).context("Parsing generated code to syn::File")?;
	Ok(prettyplease::unparse(&parsed))
}
