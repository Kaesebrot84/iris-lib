
use crate::palette::Palette;

pub trait MedianCut {
    // TODO: Muss self mutable sein?
    fn make_palette(&mut self, iter_count: u8) -> Palette;
    fn recurse(&mut self, iter_count: u8, result: &mut Palette);
    fn median_cut(&mut self) -> (Palette, Palette);
}

impl MedianCut for Palette {
    /// Creates a RGBA palette from own pixels.
    ///
    /// # Arguments
    ///
    /// * `iter_count` - number of iterations to be performed on the bucket.
    ///
    /// # Example
    ///
    /// ```
    /// use iris_lib::palette::Palette;
    /// use iris_lib::color::RGBA;
    /// use crate::iris_lib::median_cut::median_cut::MedianCut; // TODO: Better import would be Palette::Mediancut
    ///
    /// let data = vec![RGBA { r: 15, g: 131, b: 0, a: 255 }, RGBA { r: 221, g: 11, b: 22, a: 130 }, RGBA { r: 81, g: 11, b: 16, a: 0 }];
    /// let mut bucket = Palette::from_pixels(data.clone());
    /// let result = bucket.make_palette(3);
    /// ```
    ///
    fn make_palette(&mut self, iter_count: u8) -> Palette {
        let mut palette = Palette::new();
        self.recurse(iter_count, &mut palette);
        palette
    }

    /// Performs the median cut on a own vector (bucket) of `RGBA`.
    /// Returns two `RGBA` vectors representing the colors above and colors below median value.
    ///
    fn median_cut(&mut self) -> (Palette, Palette) {
        let highest_range_channel = self.highest_range_channel();
        let median = self.color_median(highest_range_channel);
        let mut above_median = vec![];
        let mut below_median = vec![];
        for rgba in &self.colors {
            if rgba[highest_range_channel] > median {
                above_median.push(*rgba);
            } else {
                below_median.push(*rgba);
            }
        }

        (Palette::from_pixels(above_median), Palette::from_pixels(below_median))
    }

    /// Recursively performs the median cut algorithm on self if iteration has not reached 0 yet.
    /// Creates two new buckets based on own colors. One bucket with values above and one bucket with value below the median, then performs the algorithm on them again.
    ///
    /// If iteration has reached 0 the color mean for self is pushed to the result vector.
    ///
    /// # Arguments
    ///
    /// * `iter_count` - Iteration index is used as termination criteria. Recursion stop when 0 is reached.
    /// * `result` - Vector holding color means for each bucket in the iteration.
    ///
    fn recurse(&mut self, iter_count: u8, result: &mut Palette) {
        if iter_count == 0 {
            result.push_rgba(self.color_mean())
        } else {
            let mut new_palettes = self.median_cut();
            if !new_palettes.0.is_empty() {
                new_palettes.0.recurse(iter_count - 1, result);
            }
            if !new_palettes.1.is_empty() {
                new_palettes.1.recurse(iter_count - 1, result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::RGBA, helpers::generate_unsorted_colors};

    use super::*;

    #[test]
    fn recurse_ut() {
        let pixels = vec![RGBA { r: 255, g: 0, b: 0, a: 255 }, RGBA { r: 0, g: 255, b: 0, a: 255 }];
        let mut palette = Palette::from_pixels(pixels.clone());
        let mut result = Palette::new();
        palette.recurse(1, &mut result);
        assert_eq!(result.colors, pixels);
    }

    #[test]
    fn make_palette_ut() {
        let pixels = vec![
            RGBA { r: 100, g: 120, b: 120, a: 0 },
            RGBA { r: 150, g: 150, b: 150, a: 0 },
            RGBA { r: 255, g: 255, b: 255, a: 0 },
        ];
        let mut palette = Palette::from_pixels(pixels.clone());

        let colors = palette.make_palette(3);
        let expected = Palette::from_pixels(vec![
            RGBA { r: 255, g: 255, b: 255, a: 0 },
            RGBA { r: 150, g: 150, b: 150, a: 0 },
            RGBA { r: 100, g: 120, b: 120, a: 0 },
        ]);
        assert_eq!(colors, expected);
    }

    #[test]
    fn median_cut_ut() {
        let mut palette = Palette::from_pixels(generate_unsorted_colors());
        let result = palette.median_cut();
        assert_eq!(result.0, Palette::from_pixels(vec![RGBA { r: 3, g: 4, b: 15, a: 2 }, RGBA { r: 55, g: 17, b: 0, a: 118 }]));
        assert_eq!(result.1, Palette::from_pixels(vec![RGBA { r: 0, g: 2, b: 1, a: 20 }, RGBA { r: 1, g: 23, b: 16, a: 20 }]));

        let mut palette = Palette::from_pixels(vec![RGBA { r: 0, g: 0, b: 0, a: 0 }]);
        let result = palette.median_cut();
        assert!(result.0.is_empty());
        assert_eq!(result.1, Palette::from_pixels(vec![RGBA { r: 0, g: 0, b: 0, a: 0 }]));
    }
}
