use std::ops::{Range, RangeBounds};

/// A selection in the text, represented by start and end byte indices.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Selection {
    pub start: usize,
    pub end: usize,
}

impl Selection {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Clears the selection, setting start and end to 0.
    pub fn clear(&mut self) {
        self.start = 0;
        self.end = 0;
    }

    /// Checks if the given offset is within the selection range.
    pub fn contains(&self, offset: usize) -> bool {
        offset >= self.start && offset < self.end
    }
}

impl From<Range<usize>> for Selection {
    fn from(value: Range<usize>) -> Self {
        Self::new(value.start, value.end)
    }
}
impl From<Selection> for Range<usize> {
    fn from(value: Selection) -> Self {
        value.start..value.end
    }
}
impl RangeBounds<usize> for Selection {
    fn start_bound(&self) -> std::ops::Bound<&usize> {
        std::ops::Bound::Included(&self.start)
    }

    fn end_bound(&self) -> std::ops::Bound<&usize> {
        std::ops::Bound::Excluded(&self.end)
    }
}

/// An ordered, non-overlapping set of selections with one primary selection.
///
/// A selection set is the coordinate model used by code editors that support
/// multiple carets. Its ranges are always sorted, and overlapping or touching
/// ranges are coalesced. The primary selection remains identifiable after
/// normalization, so editor commands can preserve the user's active caret.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionSet {
    selections: Vec<Selection>,
    primary_index: usize,
}

impl SelectionSet {
    /// Creates a set containing one primary selection.
    pub fn singleton(primary: Selection) -> Self {
        Self {
            selections: vec![primary],
            primary_index: 0,
        }
    }

    /// Creates a normalized selection set.
    ///
    /// `primary_index` identifies the primary selection in `selections` before
    /// normalization. An empty input is represented by a single caret at zero.
    pub fn new(mut selections: Vec<Selection>, primary_index: usize) -> Self {
        if selections.is_empty() {
            return Self::singleton(Selection::default());
        }

        let primary = selections[primary_index.min(selections.len() - 1)];
        let primary = Selection::new(
            primary.start.min(primary.end),
            primary.start.max(primary.end),
        );
        for selection in &mut selections {
            *selection = Selection::new(
                selection.start.min(selection.end),
                selection.start.max(selection.end),
            );
        }
        selections.sort_unstable_by_key(|selection| (selection.start, selection.end));

        let mut normalized = Vec::with_capacity(selections.len());
        for selection in selections {
            match normalized.last_mut() {
                Some(previous) if selection.start <= previous.end => {
                    previous.end = previous.end.max(selection.end);
                }
                _ => normalized.push(selection),
            }
        }

        let primary_index = normalized
            .iter()
            .position(|selection| {
                selection.start <= primary.start.min(primary.end)
                    && selection.end >= primary.start.max(primary.end)
            })
            .expect("the normalized selection set contains its primary selection");

        Self {
            selections: normalized,
            primary_index,
        }
    }

    /// Returns the selections in ascending document order.
    pub fn selections(&self) -> &[Selection] {
        &self.selections
    }

    /// Returns the selection that receives primary-caret behavior.
    pub fn primary(&self) -> Selection {
        self.selections[self.primary_index]
    }

    /// Returns the primary selection's index in [`Self::selections`].
    pub fn primary_index(&self) -> usize {
        self.primary_index
    }

    /// Returns whether the set contains more than one selection.
    pub fn is_multi_cursor(&self) -> bool {
        self.selections.len() > 1
    }
}

#[cfg(test)]
mod tests {
    use super::{Selection, SelectionSet};
    use crate::input::Position;

    #[test]
    fn test_line_column_from_to() {
        assert_eq!(
            Position::new(1, 2),
            Position {
                line: 1,
                character: 2
            }
        );
    }

    #[test]
    fn selection_set_sorts_and_coalesces_overlapping_ranges() {
        let selections = SelectionSet::new(
            vec![
                Selection::new(12, 16),
                Selection::new(4, 8),
                Selection::new(7, 12),
            ],
            0,
        );

        assert_eq!(selections.selections(), &[Selection::new(4, 16)]);
        assert_eq!(selections.primary(), Selection::new(4, 16));
    }

    #[test]
    fn selection_set_coalesces_duplicate_carets_and_preserves_primary() {
        let selections = SelectionSet::new(
            vec![
                Selection::new(10, 10),
                Selection::new(2, 2),
                Selection::new(10, 10),
            ],
            1,
        );

        assert_eq!(
            selections.selections(),
            &[Selection::new(2, 2), Selection::new(10, 10)]
        );
        assert_eq!(selections.primary_index(), 0);
        assert_eq!(selections.primary(), Selection::new(2, 2));
        assert!(selections.is_multi_cursor());
    }

    #[test]
    fn selection_set_represents_empty_input_as_a_single_caret() {
        let selections = SelectionSet::new(Vec::new(), 0);

        assert_eq!(selections.selections(), &[Selection::default()]);
        assert_eq!(selections.primary_index(), 0);
        assert!(!selections.is_multi_cursor());
    }
}
