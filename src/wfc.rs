use std::collections::HashSet;

use image;
use itertools::iproduct;
use rand::seq::IteratorRandom;

use crate::direction;
use crate::pattern;
use crate::table;

pub fn build_constraints<'p>(patterns: &Vec<&'p pattern::Pattern>) -> table::Table<[bool; 4]> {
    let directions = direction::Direction::all();
    let mut ctable = Vec::with_capacity(patterns.len() * patterns.len() * directions.len());
    for (p1, p2) in iproduct!(patterns.iter(), patterns.iter()) {
        let mut row = [false; 4];
        for (i, d) in directions.iter().enumerate() {
            row[i] = p1.overlaps(p2, d);
        }
        ctable.push(row);
    }

    table::Table::new(ctable, patterns.len())
}

/// Wave Function Collapse.
///
/// It generates arbitrarily sized textures from a given set of patterns.
pub struct Wfc<'p> {
    /// The patterns.
    patterns: Vec<&'p pattern::Pattern<'p>>,
}

impl<'p> Wfc<'p> {
    pub fn new(patterns: Vec<&'p pattern::Pattern<'p>>) -> Self {
        Wfc { patterns }
    }

    /// Implements the CSP solver.
    pub fn generate(&self, ctable: table::Table<[bool; 4]>, width: u32, height: u32) {
        let buffer = image::ImageBuffer::new(width, height);

        let mut entropy = Vec::with_capacity(width as usize * height as usize);
        for _ in 0..width * height {
            entropy.push(self.patterns.clone());
        }
        let etable = table::Table::new(entropy, width as usize);
        let mut solver = WfcI::new(ctable, etable, buffer);

        while let Some(observed_idx) = solver.observe() {
            solver.propagate(observed_idx);
        }

        assert!(solver.etable.iter().all(|x| x.len() == 1));

        for i in 0..width {
            for j in 0..height {
                let idx = i * width + j;
                let pattern = solver.etable[idx as usize][0];
                let color = pattern.pixels[0];
                solver.buffer.put_pixel(i, j, image::Rgb(color.to_slice()));

                println!("{:?}", color.to_slice());
            }
        }
    }
}

/// The internal representation of the WFC solver.
///
/// This is a wrapper around the `Wfc` struct, which contains the constraints
/// table and the input patterns.
pub struct WfcI<'p> {
    /// The constraints table.
    ///
    /// This is a `NxNx4` matrix, where `N` is the number of patterns.
    /// A member of the matrix is true if `p1` overlaps `p2` in the given direction.
    ctable: table::Table<[bool; 4]>,
    /// The entropy table.
    ///
    /// This is a `NxMxP` matrix, where `N` & `M` are the width & the height
    /// of the output image, and `P` is the number of patterns.
    etable: table::Table<Vec<&'p pattern::Pattern<'p>>>,
    /// The output image.
    buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
}

impl<'p> WfcI<'p> {
    fn new(
        ctable: table::Table<[bool; 4]>,
        etable: table::Table<Vec<&'p pattern::Pattern<'p>>>,
        buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    ) -> Self {
        WfcI {
            ctable,
            etable,
            buffer,
        }
    }

    fn observe(&mut self) -> Option<usize> {
        let min = self
            .etable
            .iter()
            .filter(|x| x.len() > 1)
            .map(|x| x.len())
            .min()
            .unwrap();
        let least_entropy = self.etable.iter().filter(|x| x.len() == min);

        let mut rng = rand::thread_rng();
        // It is fine to unwrap because we know that the iterator is not empty.
        let (idx, slot) = least_entropy.enumerate().choose(&mut rng)?;
        let observed = slot.iter().choose(&mut rng)?;

        self.etable[idx] = vec![*observed];

        Some(idx)
    }

    fn propagate(&mut self, idx: usize) {
        let observed = self.etable[idx][0];
        // The upper bound on the stack size is the size of the
        // output image, since at most we can have all the pixels
        // yet to be propagated to on the stack.
        let mut stack = Vec::with_capacity(self.etable.len());
        // We also keep a HashSet of the indices that we already have on the stack.
        let mut stack_set = HashSet::with_capacity(self.etable.len());

        // Start by pushing the observed pattern on the stack.
        stack.push((idx, observed));
        stack_set.insert(idx);

        while let Some(current) = stack.pop() {
            let (idx, pattern) = current;
            let (x, y) = self.etable.idx_to_pos(idx);

            // Get the neighbors of the current pattern.
            let neighbors = self.etable.get_neighbors((x, y));

            // Iterate over the neighbors.
            for (possible_patterns, direction) in neighbors {
                let remaining = possible_patterns
                    .into_iter()
                    .filter(|&&p| {
                        // Check if `p` is compatible with the observed pattern
                        // in direction `direction`.
                        let constraints = self.ctable[(pattern.id, p.id)];

                        // Check if the constraints are satisfied.
                        constraints[usize::from(direction)]
                    })
                    .collect::<Vec<_>>();

                // If there are no possible patterns after propagation,
                // we have a contradiction.
                if remaining.is_empty() {
                    panic!("Contradiction");
                }

                // If there is only one possible pattern, we propagate it.
                // TODO: Check that we only need to propagate when 1 is left and
                // not when there is a change possibilities (the more general case).
                if remaining.len() == 1 {
                    let collapsed = remaining[0];

                    // If the neighbor is not already on the stack, we push it.
                    if !stack_set.contains(&collapsed.id) {
                        stack.push((collapsed.id, collapsed));
                        stack_set.insert(collapsed.id);
                    }
                }
            }
        }
    }
}
