use crate::{
    color::{RGBAChannel, RGBA},
    utils::mean,
};

#[cfg(feature = "image")]
extern crate image;
#[cfg(feature = "image")]
use image::GenericImageView;

/// Struct holding an `Vec<RGBA>`.
/// Implements helpful functions for the median cut algorithm.
///
#[derive(Debug, PartialEq)]
pub struct Palette {
    pub(crate) colors: Vec<RGBA>,
}

impl Palette {
    pub fn new() -> Self {
        Palette { colors: vec![] }
    }

    /// Creates a `ColorBucket` based on the colors passed. Returns `None` if passed an empty vector.
    ///
    /// # Arguments
    ///
    /// * `colors` - RGBA vector from which the mean RGBA is created.
    ///
    /// # Examples
    ///
    /// ```
    /// use iris_lib::color::RGBA;
    /// use iris_lib::palette::Palette;
    ///
    /// let data = vec![RGBA { r: 15, g: 131, b: 0, a: 255 }, RGBA { r: 221, g: 11, b: 22, a: 130 }, RGBA { r: 81, g: 11, b: 16, a: 0 }];
    /// let result = Palette::from_pixels(data.clone());
    /// ```
    ///
    pub fn from_pixels(pixels: Vec<RGBA>) -> Self {
        Self { colors: pixels }
    }

    #[cfg(feature = "image")]
    /// Creates a `ColorBucket` based from image files. Returns `None` if unable to load the image.
    ///
    /// # Arguments
    ///
    /// * `image_path` - Path to the target image.
    ///
    /// # Examples
    ///
    /// ```
    /// use iris_lib::palette::Palette;
    ///
    /// if let Some(mut result) = Palette::from_image("peppers.png") {
    ///     // ...
    /// }
    /// ```
    ///
    pub fn from_image(image_path: &str) -> Option<Self> {
        if let Ok(img) = image::open(image_path) {
            let mut colors = Vec::new();

            for p in img.pixels() {
                let rgba = RGBA { r: p.2 .0[0], g: p.2 .0[1], b: p.2 .0[2], a: p.2 .0[3] };

                colors.push(rgba);
            }

            Some(Self { colors })
        } else {
            println!("Unable to load image at: {}", image_path);
            None
        }
    }

    /// Returns the RGBA channel with the highest range.
    /// IMPORTANT: Ignores alpha channel!
    ///
    pub fn highest_range_channel(&self) -> RGBAChannel {
        let ranges = self.color_ranges();
        let mut highest_range_channel = RGBAChannel::R;
        let mut highest_value = ranges.r;

        if ranges.g > highest_value {
            highest_range_channel = RGBAChannel::G;
            highest_value = ranges.g;
        }

        if ranges.b > highest_value {
            highest_range_channel = RGBAChannel::B;
        }

        highest_range_channel
    }

    // Returns the mean RGBA value based on own colors.
    ///
    pub fn color_mean(&self) -> RGBA {
        let r = mean(self.colors.iter().map(|c| c.r));
        let g = mean(self.colors.iter().map(|c| c.g));
        let b = mean(self.colors.iter().map(|c| c.b));
        let a = mean(self.colors.iter().map(|c| c.a));

        RGBA { r, g, b, a }
    }

    /// Returns the ranges for each RGBA channel.
    ///
    pub fn color_ranges(&self) -> RGBA {
        // Unwrap is ok here, because `max_by_key` only returns `None` for empty vectors
        RGBA {
            r: self.colors.iter().max_by_key(|c| c.r).unwrap().r - self.colors.iter().min_by_key(|c| c.r).unwrap().r,
            g: self.colors.iter().max_by_key(|c| c.g).unwrap().g - self.colors.iter().min_by_key(|c| c.g).unwrap().g,
            b: self.colors.iter().max_by_key(|c| c.b).unwrap().b - self.colors.iter().min_by_key(|c| c.b).unwrap().b,
            a: self.colors.iter().max_by_key(|c| c.a).unwrap().a - self.colors.iter().min_by_key(|c| c.a).unwrap().a,
        }
    }

    /// Sort a colors for a specific channel.
    ///
    /// # Arguments
    ///
    /// * `channel` - Target channel. The sorting is performed based on this value.
    ///
    pub fn sort_colors(&mut self, channel: RGBAChannel) {
        self.colors.sort_by_key(|x| x[channel])
    }

    /// Returns median value for a specific `RGBAChannel`.
    ///
    /// # Arguments
    ///
    /// * `channel` - Target channel for which the median is calculated.
    ///
    pub fn color_median(&mut self, channel: RGBAChannel) -> u8 {
        self.sort_colors(channel);

        let mid = self.colors.len() / 2;
        if self.colors.len() % 2 == 0 {
            let palette = Palette::from_pixels(vec![self.colors[mid - 1], self.colors[mid]]);
            palette.channel_mean(channel)
        } else {
            self.channel_value_by_index(mid, channel)
        }
    }

    /// Returns a RGBA value based on the provided channel and index parameters.
    /// TODO: What happens if this is out of bounds?
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the target RGBA in the vector.
    /// * `channel` - RGBA channel of the searched value.
    ///
    pub(crate) fn channel_value_by_index(&self, index: usize, channel: RGBAChannel) -> u8 {
        self.colors[index][channel]
    }

    /// Calculate the mean value for a specific RGBA channel on own vector of `RGBA`.
    ///
    /// # Arguments
    ///
    /// * `channel` - Target channel for which the mean is calculated.
    ///
    ///
    pub fn channel_mean(&self, channel: RGBAChannel) -> u8 {
        mean(self.colors.iter().map(|x| x[channel]))
    }

    // TODO: Docs
    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    // TODO: Docs
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    // TODO: Docs
    pub fn colors(&self) -> Vec<RGBA> {
        self.colors.clone()
    }

    // TODO: Add docs
    pub fn push_rgba(&mut self, rgba: RGBA) {
        self.colors.push(rgba);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::{RGBAChannel, RGBA},
        helpers::generate_unsorted_colors,
        palette::Palette,
    };

    #[test]
    fn from_pixels_ut() {
        let palette = Palette::from_pixels(vec![]);
        assert!(palette.is_empty());

        let data = vec![
            RGBA { r: 15, g: 131, b: 0, a: 255 },
            RGBA { r: 221, g: 11, b: 22, a: 130 },
            RGBA { r: 81, g: 11, b: 16, a: 0 },
        ];
        let palette = Palette::from_pixels(data.clone());
        assert_eq!(palette.len(), 3);
    }

    #[test]
    pub fn sort_colors_ut() {
        let colors = generate_unsorted_colors();
        let mut palette = Palette::from_pixels(colors.clone());
        palette.sort_colors(RGBAChannel::R);

        let colors = palette.colors();

        assert_eq!(colors[0], RGBA { r: 0, g: 2, b: 1, a: 20 });
        assert_eq!(colors[1], RGBA { r: 1, g: 23, b: 16, a: 20 });
        assert_eq!(colors[2], RGBA { r: 3, g: 4, b: 15, a: 2 });
        assert_eq!(colors[3], RGBA { r: 55, g: 17, b: 0, a: 118 });
    }

    #[test]
    pub fn color_median_ut() {
        let colors = generate_unsorted_colors();
        let mut bucket = Palette::from_pixels(colors.clone());
        let result = bucket.color_median(RGBAChannel::R);
        assert_eq!(result, 2);
    }

    #[test]
    fn channel_value_by_index_ut() {
        let colors = vec![
            RGBA { r: 100, g: 22, b: 12, a: 0 },
            RGBA { r: 126, g: 175, b: 137, a: 1 },
            RGBA { r: 221, g: 225, b: 0, a: 113 },
            RGBA { r: 13, g: 226, b: 0, a: 17 },
        ];

        let palette = Palette::from_pixels(colors);

        assert_eq!(100, palette.channel_value_by_index(0, RGBAChannel::R));
        assert_eq!(22, palette.channel_value_by_index(0, RGBAChannel::G));
        assert_eq!(12, palette.channel_value_by_index(0, RGBAChannel::B));
        assert_eq!(0, palette.channel_value_by_index(0, RGBAChannel::A));

        assert_eq!(126, palette.channel_value_by_index(1, RGBAChannel::R));
        assert_eq!(175, palette.channel_value_by_index(1, RGBAChannel::G));
        assert_eq!(137, palette.channel_value_by_index(1, RGBAChannel::B));
        assert_eq!(1, palette.channel_value_by_index(1, RGBAChannel::A));

        assert_eq!(221, palette.channel_value_by_index(2, RGBAChannel::R));
        assert_eq!(225, palette.channel_value_by_index(2, RGBAChannel::G));
        assert_eq!(0, palette.channel_value_by_index(2, RGBAChannel::B));
        assert_eq!(113, palette.channel_value_by_index(2, RGBAChannel::A));

        assert_eq!(13, palette.channel_value_by_index(3, RGBAChannel::R));
        assert_eq!(226, palette.channel_value_by_index(3, RGBAChannel::G));
        assert_eq!(0, palette.channel_value_by_index(3, RGBAChannel::B));
        assert_eq!(17, palette.channel_value_by_index(3, RGBAChannel::A));
    }

    #[test]
    fn channel_mean_ut() {
        let colors = vec![
            RGBA { r: 100, g: 50, b: 12, a: 255 },
            RGBA { r: 100, g: 50, b: 12, a: 255 },
            RGBA { r: 100, g: 50, b: 12, a: 255 },
            RGBA { r: 100, g: 50, b: 12, a: 255 },
        ];

        let palette = Palette::from_pixels(colors);
        let mut result = palette.channel_mean(RGBAChannel::R);
        assert_eq!(100, result);
        result = palette.channel_mean(RGBAChannel::G);
        assert_eq!(50, result);
        result = palette.channel_mean(RGBAChannel::B);
        assert_eq!(12, result);
        result = palette.channel_mean(RGBAChannel::A);
        assert_eq!(255, result);

        // More precise check
        let colors = vec![
            RGBA { r: 100, g: 22, b: 12, a: 0 },
            RGBA { r: 126, g: 175, b: 137, a: 1 },
            RGBA { r: 221, g: 225, b: 0, a: 113 },
            RGBA { r: 13, g: 226, b: 0, a: 17 },
        ];

        let bucket = Palette::from_pixels(colors);

        result = bucket.channel_mean(RGBAChannel::R);
        assert_eq!(115, result);
        result = bucket.channel_mean(RGBAChannel::G);
        assert_eq!(162, result);
        result = bucket.channel_mean(RGBAChannel::B);
        assert_eq!(37, result);
        result = bucket.channel_mean(RGBAChannel::A);
        assert_eq!(32, result);
    }

    #[test]
    fn highest_range_channel_ut() {
        let bucket = Palette::from_pixels(generate_unsorted_colors());
        assert_eq!(RGBAChannel::R, bucket.highest_range_channel());
        assert_ne!(RGBAChannel::G, bucket.highest_range_channel());
        assert_ne!(RGBAChannel::B, bucket.highest_range_channel());
        assert_ne!(RGBAChannel::A, bucket.highest_range_channel());
    }

    #[test]
    fn color_ranges_ut() {
        let palette = Palette::from_pixels(generate_unsorted_colors());
        let expected = RGBA { r: 55, g: 21, b: 16, a: 116 };
        assert_eq!(expected, palette.color_ranges());
    }

    #[test]
    pub fn ut_color_mean() {
        let colors = generate_unsorted_colors();
        let palette = Palette::from_pixels(colors);

        let result = palette.color_mean();
        let expected = RGBA { r: 14, g: 11, b: 8, a: 40 };
        assert_eq!(expected, result);
    }
}
