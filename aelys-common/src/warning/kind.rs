use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WarningKind {
    // Inline-related (W01xx)
    InlineRecursive,
    InlineMutualRecursion { cycle: Vec<String> },
    InlineHasCaptures,
    InlineTooLarge { size: usize, limit: usize },
    InlinePublicFunction,
    InlineNativeFunction,

    // Unused code (W02xx) - for later
    UnusedVariable { name: String },
    UnusedFunction { name: String },
    UnusedImport { module: String },

    // Deprecation (W03xx)
    DeprecatedFunction { name: String, replacement: Option<String> },

    // Style (W04xx)
    ShadowedVariable { name: String },
}

impl WarningKind {
    pub fn is_inline_related(&self) -> bool {
        matches!(
            self,
            WarningKind::InlineRecursive
                | WarningKind::InlineMutualRecursion { .. }
                | WarningKind::InlineHasCaptures
                | WarningKind::InlineTooLarge { .. }
                | WarningKind::InlinePublicFunction
                | WarningKind::InlineNativeFunction
        )
    }

    pub fn category(&self) -> &'static str {
        match self {
            WarningKind::InlineRecursive
            | WarningKind::InlineMutualRecursion { .. }
            | WarningKind::InlineHasCaptures
            | WarningKind::InlineTooLarge { .. }
            | WarningKind::InlinePublicFunction
            | WarningKind::InlineNativeFunction => "inline",

            WarningKind::UnusedVariable { .. }
            | WarningKind::UnusedFunction { .. }
            | WarningKind::UnusedImport { .. } => "unused",

            WarningKind::DeprecatedFunction { .. } => "deprecated",

            WarningKind::ShadowedVariable { .. } => "shadow",
        }
    }
}

impl fmt::Display for WarningKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.category())
    }
}
