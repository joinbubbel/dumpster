use super::*;
use ::image::{io::Reader, DynamicImage};
use std::io::Cursor;

pub use ::image::{imageops::FilterType, ImageFormat};

pub struct ImageOperation {
    steps: Vec<ImageOperationStep>,
    output_format: ImageFormat,
}

impl ImageOperation {
    pub fn builder(output_format: ImageFormat) -> Self {
        ImageOperation {
            steps: vec![],
            output_format,
        }
    }

    pub fn add_step(mut self, step: ImageOperationStep) -> Self {
        self.steps.push(step);
        self
    }

    /// This serves no purpose other than to signify that you are done building.
    pub fn build(self) -> Self {
        self
    }
}

impl Operation for ImageOperation {
    fn incoming(&self, _: &str, bytes: Vec<u8>) -> Option<Vec<u8>> {
        //  TODO Add Image Limits.
        let mut image = Reader::new(Cursor::new(bytes))
            .with_guessed_format()
            .ok()?
            .decode()
            .ok()?;

        for step in &self.steps {
            step.run(&mut image)?;
        }

        let mut buffer = Cursor::new(vec![]);
        image.write_to(&mut buffer, self.output_format).ok()?;
        Some(buffer.into_inner())
    }
}

pub enum ImageOperationStep {
    Resize(u32, u32, FilterType),
    Blur(f32),
}

impl ImageOperationStep {
    pub fn run(&self, image: &mut DynamicImage) -> Option<()> {
        *image = match self {
            ImageOperationStep::Resize(width, height, filter) => image.resize(*width, *height, *filter),
            ImageOperationStep::Blur(sigma) => image.blur(*sigma),
        };

        Some(())
    }
}
