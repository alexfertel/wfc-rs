use image;

use crate::pattern;

/// Constraints table.
///
/// This table is used to store the constraints between patterns.
pub struct ConstraintsTable<'p> {
    /// The patterns.
    patterns: Vec<&'p pattern::Pattern<'p>>,
    /// The constraints table.
    table: Vec<bool>,
}

impl<'p> ConstraintsTable<'p> {
    pub fn new(table: Vec<bool>, patterns: Vec<&'p pattern::Pattern<'p>>) -> Self {
        ConstraintsTable { table, patterns }
    }

    /// Implements the CSP solver.
    pub fn generate(&self, size: usize) {
        // let buffer = image::ImageBuffer::new(size as u32, size as u32);
        todo!()
    }
}
