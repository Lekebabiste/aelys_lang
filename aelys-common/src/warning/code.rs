use super::WarningKind;

impl WarningKind {
    pub fn code(&self) -> u16 {
        match self {
            // inline warnings: 100-199
            Self::InlineRecursive => 101,
            Self::InlineMutualRecursion { .. } => 102,
            Self::InlineHasCaptures => 103,
            Self::InlineTooLarge { .. } => 104,
            Self::InlinePublicFunction => 105,
            Self::InlineNativeFunction => 106,

            // unused: 200-299
            Self::UnusedVariable { .. } => 201,
            Self::UnusedFunction { .. } => 202,
            Self::UnusedImport { .. } => 203,

            // deprecation: 300-399
            Self::DeprecatedFunction { .. } => 301,

            // style: 400-499
            Self::ShadowedVariable { .. } => 401,
        }
    }
}
