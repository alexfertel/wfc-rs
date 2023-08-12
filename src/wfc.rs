use std::collections::HashSet;

use image;
use rand::seq::IteratorRandom;

use crate::pattern;
use crate::table;

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

        // We start by pushing the observed pattern on the stack.
        stack.push((idx, observed));
        stack_set.insert(idx);

        while let Some(current) = stack.pop() {
            let (idx, pattern) = current;
            let (x, y) = self.etable.idx_to_pos(idx);

            // We get the neighbors of the current pattern.
            let neighbors = self.etable.get_neighbors((x, y));

            // We iterate over the neighbors.
            for (possible_patterns, direction) in neighbors {
                possible_patterns.retain(|p| {
                    // We check if `p` is compatible with the observed pattern
                    // in direction `direction`.
                    let constraints = self.ctable[(pattern.id, p.id)];

                    // We check if the constraints are satisfied.
                    constraints[Into::<usize>::into(direction)]
                });

                // If there are no possible patterns after propagation,
                // we have a contradiction.
                if possible_patterns.is_empty() {
                    panic!("Contradiction");
                }

                // TODO: Figure out borrowing here.
                // let (nx, ny) =
                //     direction.add_pos((x.try_into().unwrap(), y.try_into().unwrap()));
                // let n = self.etable[(nx as usize, ny as usize)].clone();
                // assert_eq!(possible_patterns, n);

                // If there is only one possible pattern, we propagate it.
                // TODO: Check that we only need to propagate when 1 is left and
                // not when there is a change possibilities (the more general case).
                if possible_patterns.len() == 1 {
                    let collapsed = possible_patterns[0];

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
