use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

use image;
use itertools::iproduct;
use itertools::Itertools;
use rand::seq::IteratorRandom;

use crate::direction;
use crate::direction::Direction;
use crate::pattern;
use crate::table;
use crate::Image;

type CTable = HashMap<(usize, usize), u8>;
type ETable<'p> = table::Table<Vec<&'p pattern::Pattern<'p>>>;

/// Wave Function Collapse.
///
/// It generates arbitrarily sized textures from a given set of patterns.
pub struct Wfc<'p> {
    /// The patterns.
    patterns: Vec<&'p pattern::Pattern<'p>>,
    /// The constraints table.
    ///
    /// This is a `NxNx4` matrix, where `N` is the number of patterns.
    /// A member of the matrix is true if `p1` overlaps `p2` in the given direction.
    ctable: CTable,
}

impl<'p> Wfc<'p> {
    pub fn new(patterns: Vec<&'p pattern::Pattern<'p>>) -> Self {
        let ctable = Wfc::build_constraints(&patterns);
        Wfc { patterns, ctable }
    }

    pub fn build_constraints(patterns: &Vec<&'p pattern::Pattern<'p>>) -> CTable {
        let directions = direction::Direction::all();
        let mut ctable = HashMap::default();
        for (p1, p2) in iproduct!(patterns.iter(), patterns.iter()) {
            let mut row = 0u8;
            for d in directions {
                row = row | (u8::from(p1.overlaps(p2, &d)) << u8::from(d));
            }
            ctable.insert((p1.id, p2.id), row);
        }

        ctable
    }

    /// Implements the CSP solver.
    pub fn generate(&self, width: u32, height: u32) -> Image {
        let buffer = image::ImageBuffer::new(width, height);

        let mut entropy = Vec::with_capacity(width as usize * height as usize);
        for _ in 0..width * height {
            entropy.push(self.patterns.clone());
        }
        let etable = table::Table::new(entropy, width as usize);
        let mut solver = WfcI::new(&self.ctable, etable, buffer);

        while let Some(observed_idx) = solver.observe() {
            solver.propagate(observed_idx);
        }

        assert!(solver.etable.iter().all(|x| x.len() == 1));

        for i in 0..height {
            for j in 0..width {
                let idx = i * width + j;
                let pattern = solver.etable[idx as usize][0];
                let color = pattern.pixels[0];
                solver.buffer.put_pixel(i, j, image::Rgb(color.to_slice()));
            }
        }

        solver.buffer
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
    ctable: &'p CTable,
    /// The entropy table.
    ///
    /// This is a `NxMxP` matrix, where `N` & `M` are the width & the height
    /// of the output image, and `P` is the number of patterns.
    etable: ETable<'p>,
    /// The output image.
    buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
}

impl<'p> WfcI<'p> {
    fn new(
        ctable: &'p CTable,
        etable: ETable<'p>,
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
            .map(|x| x.len())
            .filter(|&x| x > 1)
            .min()?;

        let least_entropy = self
            .etable
            .iter()
            .enumerate()
            .filter(|(_, x)| x.len() == min);

        let mut rng = rand::thread_rng();
        let (idx, slot) = least_entropy.choose(&mut rng)?;
        let observed = slot.iter().choose(&mut rng)?;

        self.etable[idx] = vec![*observed];

        Some(idx)
    }

    fn propagate(&mut self, start_idx: usize) {
        // The upper bound on the stack size is the size of the
        // output image, since at most we can have all the pixels
        // yet to be propagated to on the stack.
        let mut stack = Vec::with_capacity(self.etable.len());
        // We also keep a HashSet of the indices that we already have on the stack.
        let mut stack_set = HashSet::default();

        // Start by pushing the observed pattern onto the stack.
        stack.push(start_idx);
        stack_set.insert(start_idx);

        while let Some(current_idx) = stack.pop() {
            stack_set.remove(&current_idx);
            let (x, y) = self.etable.idx_to_pos(current_idx);

            // Get the neighbors of the current pattern.

            for (nx, ny) in self.etable.get_neighbors((x, y)) {
                let neighbor_possibilities = self.etable.get((nx, ny));

                let mut remaining_set = HashSet::default();
                let direction = Direction::from_neighbors((x, y), (nx, ny));
                for possibility in self.etable.get((x, y)) {
                    let remaining = neighbor_possibilities
                        .iter()
                        .filter(|&p| {
                            // Check if `p` is compatible with the observed pattern
                            // in direction `direction`. It's okay to unwrap since we
                            // have to assume the table is populated correctly.
                            let constraints = self.ctable.get(&(possibility.id, p.id)).unwrap();

                            // Check if the constraints are satisfied.
                            (constraints >> u8::from(direction) & 1) != 0
                        })
                        .collect_vec();

                    // Add the possible slots that this possibility enables.
                    remaining_set.extend(remaining);
                }

                // If there are no possible patterns after propagation,
                // we have a contradiction.
                if remaining_set.is_empty() {
                    panic!("Contradiction");
                }

                // If there was a change in possibilities we propagate that
                // position as well.
                //
                // This is needed because if we remove possibilities from a
                // slot S we might end up in a situation where a neighbor of
                // S gets observed with a pattern that has no overlap with
                // any of the possible patterns in S.
                if neighbor_possibilities.len() != remaining_set.len() {
                    let idx = nx * self.etable.width() + ny;

                    // If the neighbor is not already on the stack, we push it.
                    if stack_set.insert(idx) {
                        stack.push(idx)
                    }
                }

                // Collapse the neighboring slot.
                self.etable[(nx, ny)] = remaining_set.into_iter().collect();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rustc_hash::FxHashMap as HashMap;

    use image::{Rgb, RgbImage};
    use itertools::Itertools;

    use crate::test_utils::p;

    #[test]
    fn build_constraints() {
        // [0, 1, 2]
        // [1, 2, 3]
        // [2, 3, 4]
        let mut texture = RgbImage::new(4, 4);
        for x in 0..4 {
            for y in 0..4 {
                texture.put_pixel(x, y, Rgb([(x + y) as u8, 0, 0]));
            }
        }

        let patterns = vec![p(0, 2, &texture, (0, 0)), p(1, 2, &texture, (1, 0))];

        let mut expected = HashMap::default();
        expected.insert((0, 0), 0b0000);
        expected.insert((0, 1), 0b0110);
        expected.insert((1, 0), 0b1001);
        expected.insert((1, 1), 0b0000);
        let actual = super::Wfc::new(patterns.iter().collect_vec()).ctable;
        assert_eq!(expected, actual);

        // [0, 1, 2]
        // [1, 2, 3]
        // [2, 3, 4]
        let mut texture = RgbImage::new(4, 4);
        for x in 0..4 {
            for y in 0..4 {
                texture.put_pixel(x, y, Rgb([(x + y) as u8, 0, 0]));
            }
        }

        let patterns = vec![p(0, 3, &texture, (0, 0)), p(1, 3, &texture, (1, 0))];

        let mut expected = HashMap::default();
        expected.insert((0, 0), 0b0000);
        expected.insert((0, 1), 0b0110);
        expected.insert((1, 0), 0b1001);
        expected.insert((1, 1), 0b0000);
        let actual = super::Wfc::new(patterns.iter().collect_vec()).ctable;
        assert_eq!(expected, actual);
    }
}
