use dua::{ByteFormat, WalkOptions};
use std::fmt;

#[derive(Clone, Copy)]
pub enum ByteVisualization {
    Percentage,
    Bar,
    LongBar,
    PercentageAndBar,
}

pub struct DisplayByteVisualization {
    format: ByteVisualization,
    percentage: f32,
}

impl Default for ByteVisualization {
    fn default() -> Self {
        ByteVisualization::PercentageAndBar
    }
}

impl ByteVisualization {
    pub fn cycle(&mut self) {
        use ByteVisualization::*;
        *self = match self {
            Bar => LongBar,
            LongBar => PercentageAndBar,
            PercentageAndBar => Percentage,
            Percentage => Bar,
        }
    }
    pub fn display(self, percentage: f32) -> DisplayByteVisualization {
        DisplayByteVisualization {
            format: self,
            percentage,
        }
    }
}

impl fmt::Display for DisplayByteVisualization {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use ByteVisualization::*;
        let Self { format, percentage } = self;

        let percentage = if percentage.is_nan() {
            0.0
        } else {
            *percentage
        };
        const BAR_SIZE: usize = 25;
        match format {
            Percentage => Self::make_percentage(f, percentage),
            PercentageAndBar => {
                Self::make_percentage(f, percentage)?;
                f.write_str(" ")?;
                Self::make_bar(f, percentage, BAR_SIZE)
            }
            Bar => Self::make_bar(f, percentage, BAR_SIZE),
            LongBar => Self::make_bar(f, percentage, 19),
        }
    }
}

impl DisplayByteVisualization {
    fn make_bar(f: &mut fmt::Formatter, percentage: f32, length: usize) -> Result<(), fmt::Error> {
        // Print the filled part of the bar
        let block_length = (length as f32 * percentage).floor() as usize;
        f.write_str("[")?;
        for _ in 0..block_length {
            f.write_str("#")?;
        }

        // Bar is done if full length is already used, continue working if not
        if block_length < length {
            let block_sections = [" "," "," "," ","#","#","#","#","#"];
            // Get the index based on how filled the remaining part is
            let index =
                (((length as f32 * percentage) - block_length as f32) * 8f32).round() as usize;
            f.write_str(block_sections[index])?;

            // Remainder of the bar should be empty
            for _ in 0..length - block_length - 1 {
                f.write_str(" ")?;
            }
        }
        f.write_str("]")?;
        Ok(())
    }
    fn make_percentage(f: &mut fmt::Formatter, percentage: f32) -> Result<(), fmt::Error> {
        write!(f, " {:>5.01}% ", percentage * 100.0)
    }
}

/// Options to configure how we display things
#[derive(Clone, Copy)]
pub struct DisplayOptions {
    pub byte_format: ByteFormat,
    pub byte_vis: ByteVisualization,
}

impl From<WalkOptions> for DisplayOptions {
    fn from(WalkOptions { byte_format, .. }: WalkOptions) -> Self {
        DisplayOptions {
            byte_format,
            byte_vis: ByteVisualization::default(),
        }
    }
}
