// Enum types for SysML v2

/// Requirement classification kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequirementKind {
    Objective,
    Verify,
}

/// Parameter classification kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParameterKind {
    Actor,
    Stakeholder,
}

/// Requirement constraint classification kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequirementConstraintKind {
    Assume,
    Require,
}

/// Portion classification kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortionKind {
    Timeslice,
    Snapshot,
}

/// Trigger classification kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerKind {
    When,
    At,
    After,
}

/// State subaction classification kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StateSubactionKind {
    Entry,
    Do,
    Exit,
}

/// Transition feature classification kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransitionFeatureKind {
    Trigger,
    Guard,
    Effect,
}
