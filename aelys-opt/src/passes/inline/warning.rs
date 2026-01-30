use aelys_syntax::Span;
use std::fmt;

#[derive(Debug, Clone)]
pub struct InlineWarning {
    pub kind: InlineWarningKind,
    pub function_name: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum InlineWarningKind {
    RecursiveFunction,
    MutualRecursion { cycle: Vec<String> },
    HasCaptures,
    TooLarge { body_size: usize, threshold: usize },
    PublicFunction,
    NativeFunction,
}

impl InlineWarning {
    pub fn new(kind: InlineWarningKind, function_name: String, span: Span) -> Self {
        Self { kind, function_name, span }
    }

    pub fn message(&self) -> String {
        match &self.kind {
            InlineWarningKind::RecursiveFunction => {
                format!(
                    "cannot inline recursive function '{}'\n   \
                     = note: inlining would cause infinite expansion\n   \
                     = hint: remove @inline from this function",
                    self.function_name
                )
            }
            InlineWarningKind::MutualRecursion { cycle } => {
                format!(
                    "cannot inline '{}' due to mutual recursion\n   \
                     = note: call cycle: {}\n   \
                     = hint: break the cycle or remove @inline",
                    self.function_name,
                    cycle.join(" -> ")
                )
            }
            InlineWarningKind::HasCaptures => {
                format!(
                    "inlining '{}' with captured variables may change behavior\n   \
                     = note: function captures variables from enclosing scope\n   \
                     = hint: use @inline_always if you're sure this is safe",
                    self.function_name
                )
            }
            InlineWarningKind::TooLarge { body_size, threshold } => {
                format!(
                    "skipping inline for '{}': body too large ({} statements, threshold {})\n   \
                     = note: inlining would increase code size significantly\n   \
                     = hint: use @inline_always to force, or refactor into smaller functions",
                    self.function_name, body_size, threshold
                )
            }
            InlineWarningKind::PublicFunction => {
                format!(
                    "inlining public function '{}'\n   \
                     = note: the original function will still be emitted for external callers\n   \
                     = hint: this is usually fine, but calls from other modules won't be inlined",
                    self.function_name
                )
            }
            InlineWarningKind::NativeFunction => {
                format!(
                    "cannot inline native function '{}'\n   \
                     = note: native functions have no Aelys body to substitute\n   \
                     = hint: remove @inline from this declaration",
                    self.function_name
                )
            }
        }
    }

    pub fn is_fatal(&self) -> bool {
        matches!(
            self.kind,
            InlineWarningKind::RecursiveFunction
                | InlineWarningKind::MutualRecursion { .. }
                | InlineWarningKind::NativeFunction
        )
    }
}

impl fmt::Display for InlineWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "warning: {}", self.message())
    }
}
