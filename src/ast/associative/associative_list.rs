use std::cmp::Ordering;
use std::fmt::Formatter;
use crate::ast::node::Node;

pub trait AssociativeList {
    /// Main constructor.
    /// Constructor should return normalized node.
    fn from_items(items: &[Node]) -> Node where Self: Sized {
        let this = Self::construct_self(items);
        this.normalize()
    }

    /// For example, for Addition: return Node::Addition(self)
    fn construct_node_variant(self) -> Node;
    /// For example, for Addition: return Addition { items: items.to_vec() }
    fn construct_self(items: &[Node]) -> Self;

    fn items(&self) -> &[Node];

    fn flatten(&mut self);
    fn propagate_constants(&mut self);
    fn ensure_sorted(&mut self);

    /// Normalization should (if applicable):
    /// - Perform flattening (AssociativeList::flatten())
    /// - Perform identities removal
    /// - Perform constant propagation (AssociativeList::propagate_constants())
    /// - Ensure that length of list > 1
    /// - Ensure that the items are sorted according to Node's Ord implementation (AssociativeList::ensure_ordered())
    /// See `NormalizationResult` for details on returned value
    #[must_use]
    fn normalize(self) -> Node;

    // If two lists are of different length, compare them by their length.
    // Otherwise, compare their items in lexicographic order.
    // This method assumes that self.items and other.items are already sorted.
    fn default_cmp(&self, other: &Self) -> Ordering {
        let length_order = self.items().len().cmp(&other.items().len());
        // If two lists are of different len
        if length_order != Ordering::Equal {
            return length_order;
        }

        let is_sorted = |arr: &[Node]| arr.windows(2).all(|w| w[0] <= w[1]);
        debug_assert!(is_sorted(self.items()));
        debug_assert!(is_sorted(other.items()));

        self.items().cmp(other.items())
    }

    fn fmt_associative(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let symbol = self.symbol();
        write!(f, "(")?;
        let items = self.items();
        for i in 0..items.len() {
            write!(f, "{}", items[i])?;
            if i != items.len() - 1 {
                write!(f, " {symbol} ")?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
    fn symbol(&self) -> &'static str;
}
