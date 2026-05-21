use tree_sitter::Node;

use crate::context::RuleContext;
use crate::diagnostic::Severity;

pub trait Rule: Send + Sync {
    fn name(&self) -> &'static str;
    fn default_severity(&self) -> Severity;

    /// Called for each AST node during traversal.
    /// The rule inspects `node.kind()` and reports via `ctx` if needed.
    fn on_node(&self, node: &Node, ctx: &RuleContext);
}
