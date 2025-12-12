// SysML v2 Element types

use crate::language::kerml::interfaces::expressions::InvocationExpression;
use crate::language::kerml::interfaces::references::ClassifierReference;
use crate::language::kerml::interfaces::relationships::{
    Conjugation, FeatureMembership, FeatureTyping, Import, Membership, MembershipImport,
    NamespaceImport, OwningMembership, ParameterMembership,
};
use crate::language::kerml::interfaces::{
    AssociationStructure, Behavior, BindingConnector, BooleanExpression, Class, Classifier,
    Connector, DataType, Expression, Feature, Interaction, Invariant, ItemFlow, Metaclass,
    MetadataFeature, Predicate, Step, Structure, Succession, SuccessionItemFlow, SysMLFunction,
};
use crate::language::sysml::types::{PortionKind, RequirementConstraintKind, TriggerKind};

/// Definition extends Classifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    pub classifier: Classifier,
    pub is_variation: bool,
    pub is_individual: bool,
}

/// Usage extends Feature
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Usage {
    pub feature: Feature,
    pub is_variation: bool,
    pub is_reference: bool,
    pub portion_kind: Option<PortionKind>,
    pub is_individual: bool,
}

/// OccurrenceDefinition extends Definition and Class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OccurrenceDefinition {
    pub definition: Definition,
    pub class: Class,
}

/// OccurrenceUsage extends Usage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OccurrenceUsage {
    pub usage: Usage,
}

/// AttributeDefinition extends Definition and DataType
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeDefinition {
    pub definition: Definition,
    pub data_type: DataType,
}

/// AttributeUsage extends Usage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeUsage {
    pub usage: Usage,
}

/// ItemDefinition extends OccurrenceDefinition and Structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemDefinition {
    pub occurrence_definition: OccurrenceDefinition,
    pub structure: Structure,
}

/// ItemUsage extends OccurrenceUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemUsage {
    pub occurrence_usage: OccurrenceUsage,
}

/// PartDefinition extends ItemDefinition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartDefinition {
    pub item_definition: ItemDefinition,
}

/// PartUsage extends ItemUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartUsage {
    pub item_usage: ItemUsage,
}

/// PortDefinition extends OccurrenceDefinition and Structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortDefinition {
    pub occurrence_definition: OccurrenceDefinition,
    pub structure: Structure,
}

/// PortUsage extends OccurrenceUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortUsage {
    pub occurrence_usage: OccurrenceUsage,
}

/// ActionDefinition extends OccurrenceDefinition and Behavior
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionDefinition {
    pub occurrence_definition: OccurrenceDefinition,
    pub behavior: Behavior,
}

/// ActionUsage extends OccurrenceUsage and Step
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionUsage {
    pub occurrence_usage: OccurrenceUsage,
    pub step: Step,
}

/// ReferenceUsage extends Usage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceUsage {
    pub usage: Usage,
}

/// MetadataDefinition extends Metaclass and ItemDefinition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataDefinition {
    pub metaclass: Metaclass,
    pub item_definition: ItemDefinition,
}

/// MetadataUsage extends MetadataFeature and ItemUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataUsage {
    pub metadata_feature: MetadataFeature,
    pub item_usage: ItemUsage,
}

/// IfActionUsage extends ActionUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfActionUsage {
    pub action_usage: ActionUsage,
    pub condition: ParameterMembership,
    pub then: ParameterMembership,
    pub else_: Option<ParameterMembership>,
}

/// StateDefinition extends ActionDefinition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateDefinition {
    pub action_definition: ActionDefinition,
    pub is_parallel: bool,
}

/// StateUsage extends ActionUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateUsage {
    pub action_usage: ActionUsage,
    pub is_parallel: bool,
}

/// ExhibitStateUsage extends StateUsage and PerformActionUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExhibitStateUsage {
    pub state_usage: StateUsage,
    pub perform_action_usage: PerformActionUsage,
}

/// ConstraintDefinition extends OccurrenceDefinition and Predicate
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintDefinition {
    pub occurrence_definition: OccurrenceDefinition,
    pub predicate: Predicate,
}

/// ConstraintUsage extends OccurrenceUsage and BooleanExpression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintUsage {
    pub occurrence_usage: OccurrenceUsage,
    pub boolean_expression: BooleanExpression,
}

/// AssertConstraintUsage extends ConstraintUsage and Invariant
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssertConstraintUsage {
    pub constraint_usage: ConstraintUsage,
    pub invariant: Invariant,
}

/// TransitionUsage extends ActionUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionUsage {
    pub action_usage: ActionUsage,
    pub source: Option<Membership>,
    pub transition_link_source: Option<ParameterMembership>,
    pub payload: Option<ParameterMembership>,
    pub accepter: Option<TransitionFeatureMembership>,
    pub guard: Option<TransitionFeatureMembership>,
    pub effect: Option<TransitionFeatureMembership>,
    pub then: Option<OwningMembership>,
    pub else_: Option<OwningMembership>,
}

/// AcceptActionUsage extends ActionUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptActionUsage {
    pub action_usage: ActionUsage,
    pub payload: ParameterMembership,
    pub receiver: Option<ParameterMembership>,
}

/// RequirementDefinition extends ConstraintDefinition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementDefinition {
    pub constraint_definition: ConstraintDefinition,
}

/// RequirementUsage extends ConstraintUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementUsage {
    pub constraint_usage: ConstraintUsage,
}

/// SatisfyRequirementUsage extends RequirementUsage and AssertConstraintUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SatisfyRequirementUsage {
    pub requirement_usage: RequirementUsage,
    pub assert_constraint_usage: AssertConstraintUsage,
    pub satisfaction_subject: Option<SubjectMembership>,
}

/// ConcernDefinition extends RequirementDefinition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcernDefinition {
    pub requirement_definition: RequirementDefinition,
}

/// ConcernUsage extends RequirementUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcernUsage {
    pub requirement_usage: RequirementUsage,
}

/// CalculationDefinition extends ActionDefinition and SysMLFunction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalculationDefinition {
    pub action_definition: ActionDefinition,
    pub function: SysMLFunction,
}

/// CalculationUsage extends ActionUsage and Expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalculationUsage {
    pub action_usage: ActionUsage,
    pub expression: Expression,
}

/// CaseDefinition extends CalculationDefinition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseDefinition {
    pub calculation_definition: CalculationDefinition,
}

/// CaseUsage extends CalculationUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseUsage {
    pub calculation_usage: CalculationUsage,
}

/// AnalysisCaseDefinition extends CaseDefinition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisCaseDefinition {
    pub case_definition: CaseDefinition,
}

/// AnalysisCaseUsage extends CaseUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisCaseUsage {
    pub case_usage: CaseUsage,
}

/// ConnectorAsUsage extends Usage and Connector
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectorAsUsage {
    pub usage: Usage,
    pub connector: Connector,
}

/// BindingConnectorAsUsage extends ConnectorAsUsage and BindingConnector
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingConnectorAsUsage {
    pub connector_as_usage: ConnectorAsUsage,
    pub binding_connector: BindingConnector,
}

/// ConnectionDefinition extends PartDefinition and AssociationStructure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionDefinition {
    pub part_definition: PartDefinition,
    pub association_structure: AssociationStructure,
}

/// ConnectionUsage extends PartUsage and ConnectorAsUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionUsage {
    pub part_usage: PartUsage,
    pub connector_as_usage: ConnectorAsUsage,
}

/// InterfaceDefinition extends ConnectionDefinition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceDefinition {
    pub connection_definition: ConnectionDefinition,
}

/// InterfaceUsage extends ConnectionUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceUsage {
    pub connection_usage: ConnectionUsage,
}

/// ViewDefinition extends PartDefinition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewDefinition {
    pub part_definition: PartDefinition,
}

/// ViewUsage extends PartUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewUsage {
    pub part_usage: PartUsage,
}

/// ViewpointDefinition extends RequirementDefinition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewpointDefinition {
    pub requirement_definition: RequirementDefinition,
}

/// ViewpointUsage extends RequirementUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewpointUsage {
    pub requirement_usage: RequirementUsage,
}

/// RenderingDefinition extends PartDefinition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderingDefinition {
    pub part_definition: PartDefinition,
}

/// RenderingUsage extends PartUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderingUsage {
    pub part_usage: PartUsage,
}

/// VerificationCaseDefinition extends CaseDefinition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationCaseDefinition {
    pub case_definition: CaseDefinition,
}

/// VerificationCaseUsage extends CaseUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationCaseUsage {
    pub case_usage: CaseUsage,
}

/// EnumerationDefinition extends AttributeDefinition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumerationDefinition {
    pub attribute_definition: AttributeDefinition,
}

/// EnumerationUsage extends AttributeUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumerationUsage {
    pub attribute_usage: AttributeUsage,
}

/// AllocationDefinition extends ConnectionDefinition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocationDefinition {
    pub connection_definition: ConnectionDefinition,
}

/// AllocationUsage extends ConnectionUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocationUsage {
    pub connection_usage: ConnectionUsage,
}

/// UseCaseDefinition extends CaseDefinition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseCaseDefinition {
    pub case_definition: CaseDefinition,
}

/// UseCaseUsage extends CaseUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseCaseUsage {
    pub case_usage: CaseUsage,
}

/// IncludeUseCaseUsage extends UseCaseUsage and PerformActionUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncludeUseCaseUsage {
    pub use_case_usage: UseCaseUsage,
    pub perform_action_usage: PerformActionUsage,
}

/// FlowConnectionDefinition extends ActionDefinition and Interaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowConnectionDefinition {
    pub action_definition: ActionDefinition,
    pub interaction: Interaction,
}

/// FlowConnectionUsage extends ConnectorAsUsage, ActionUsage, and ItemFlow
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowConnectionUsage {
    pub connector_as_usage: ConnectorAsUsage,
    pub action_usage: ActionUsage,
    pub item_flow: ItemFlow,
    pub messages: Vec<ParameterMembership>,
}

/// SuccessionFlowConnectionUsage extends FlowConnectionUsage and SuccessionItemFlow
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuccessionFlowConnectionUsage {
    pub flow_connection_usage: FlowConnectionUsage,
    pub succession_item_flow: SuccessionItemFlow,
}

/// AssignmentActionUsage extends ActionUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignmentActionUsage {
    pub action_usage: ActionUsage,
    pub target_member: Membership,
    pub assigned_value: ParameterMembership,
}

/// TriggerInvocationExpression extends InvocationExpression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggerInvocationExpression {
    pub invocation_expression: InvocationExpression,
    pub kind: TriggerKind,
}

/// EventOccurrenceUsage extends OccurrenceUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventOccurrenceUsage {
    pub occurrence_usage: OccurrenceUsage,
}

/// PerformActionUsage extends ActionUsage and EventOccurrenceUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerformActionUsage {
    pub action_usage: ActionUsage,
    pub event_occurrence_usage: EventOccurrenceUsage,
}

/// LoopActionUsage extends ActionUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopActionUsage {
    pub action_usage: ActionUsage,
}

/// WhileLoopActionUsage extends LoopActionUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileLoopActionUsage {
    pub loop_action_usage: LoopActionUsage,
    pub condition: Option<ParameterMembership>,
    pub body: ParameterMembership,
    pub until: Option<ParameterMembership>,
}

/// ForLoopActionUsage extends LoopActionUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForLoopActionUsage {
    pub loop_action_usage: LoopActionUsage,
    pub variable: FeatureMembership,
    pub sequence: ParameterMembership,
    pub body: ParameterMembership,
}

/// SendActionUsage extends ActionUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendActionUsage {
    pub action_usage: ActionUsage,
    pub payload: ParameterMembership,
    pub sender: Option<ParameterMembership>,
    pub receiver: Option<ParameterMembership>,
}

/// ControlNode extends ActionUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlNode {
    pub action_usage: ActionUsage,
}

/// ForkNode extends ControlNode
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForkNode {
    pub control_node: ControlNode,
}

/// MergeNode extends ControlNode
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeNode {
    pub control_node: ControlNode,
}

/// JoinNode extends ControlNode
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinNode {
    pub control_node: ControlNode,
}

/// DecisionNode extends ControlNode
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionNode {
    pub control_node: ControlNode,
}

/// SuccessionAsUsage extends ConnectorAsUsage and Succession
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuccessionAsUsage {
    pub connector_as_usage: ConnectorAsUsage,
    pub succession: Succession,
}

/// Expose extends Import
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expose {
    pub import: Import,
}

/// ConjugatedPortReference extends ClassifierReference
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConjugatedPortReference {
    pub classifier_reference: ClassifierReference,
}

/// VariantMembership extends OwningMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantMembership {
    pub owning_membership: OwningMembership,
}

/// LifeClass extends Class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifeClass {
    pub class: Class,
}

/// ConjugatedPortDefinition extends PortDefinition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConjugatedPortDefinition {
    pub port_definition: PortDefinition,
}

/// ConjugatedPortTyping extends FeatureTyping
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConjugatedPortTyping {
    pub feature_typing: FeatureTyping,
}

/// PortConjugation extends Conjugation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortConjugation {
    pub conjugation: Conjugation,
}

/// StateSubactionKind enum for StateSubactionMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateSubactionKind {
    Entry,
    Exit,
    Do,
}

/// StateSubactionMembership extends FeatureMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateSubactionMembership {
    pub feature_membership: FeatureMembership,
    pub kind: StateSubactionKind,
}

/// TransitionFeatureKind enum for TransitionFeatureMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitionFeatureKind {
    Accept,
    If,
    Do,
}

/// TransitionFeatureMembership extends FeatureMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionFeatureMembership {
    pub feature_membership: FeatureMembership,
    pub kind: TransitionFeatureKind,
}

/// SubjectMembership extends ParameterMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubjectMembership {
    pub parameter_membership: ParameterMembership,
}

/// ActorMembership extends ParameterMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorMembership {
    pub parameter_membership: ParameterMembership,
}

/// StakeholderMembership extends ParameterMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StakeholderMembership {
    pub parameter_membership: ParameterMembership,
}

/// RequirementConstraintMembership extends FeatureMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementConstraintMembership {
    pub feature_membership: FeatureMembership,
    pub kind: Option<RequirementConstraintKind>,
}

/// FramedConcernMembership extends RequirementConstraintMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FramedConcernMembership {
    pub requirement_constraint_membership: RequirementConstraintMembership,
}

/// RequirementVerificationMembership extends RequirementConstraintMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementVerificationMembership {
    pub requirement_constraint_membership: RequirementConstraintMembership,
}

/// ObjectiveMembership extends FeatureMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectiveMembership {
    pub feature_membership: FeatureMembership,
}

/// ViewRenderingMembership extends FeatureMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewRenderingMembership {
    pub feature_membership: FeatureMembership,
}

/// MembershipExpose extends Expose and MembershipImport
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MembershipExpose {
    pub expose: Expose,
    pub membership_import: MembershipImport,
}

/// NamespaceExpose extends Expose and NamespaceImport
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceExpose {
    pub expose: Expose,
    pub namespace_import: NamespaceImport,
}

/// TerminateActionUsage extends ActionUsage
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminateActionUsage {
    pub action_usage: ActionUsage,
    pub terminated_occurrence: Option<ParameterMembership>,
}
