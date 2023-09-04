use super::*;
use ::image::{io::Reader, DynamicImage};
use std::io::Cursor;

pub use ::image::ImageFormat;

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
    fn incoming(&self, bytes: Vec<u8>) -> Result<Vec<u8>, OperationReject> {
        //  TODO Add Image Limits.
        let mut image = Reader::new(Cursor::new(bytes))
            .with_guessed_format()
            .map_err(|e| OperationReject::DataCorrupt {
                reason: e.to_string(),
            })?
            .decode()
            .map_err(|e| OperationReject::DataCorrupt {
                reason: e.to_string(),
            })?;

        for step in &self.steps {
            step.run(&mut image)?;
        }

        let mut buffer = Cursor::new(vec![]);
        image
            .write_to(&mut buffer, self.output_format)
            .map_err(|e| OperationReject::DataConstraint {
                reason: e.to_string(),
            })?;
        Ok(buffer.into_inner())
    }
}

pub enum ImageOperationStep {
    MaxSize(u32, u32),
    Resize(u32, u32),
    Blur(f32),
}

impl ImageOperationStep {
    pub fn run(&self, image: &mut DynamicImage) -> Result<(), OperationReject> {
        *image = match self {
            ImageOperationStep::MaxSize(max_width, max_height) => (image.width() < *max_width
                && image.height() < *max_height)
                .then_some(image.clone())
                .ok_or(OperationReject::DataConstraint {
                    reason: "Max Size Exceeded".to_owned(),
                })?,
            ImageOperationStep::Resize(width, height) => image.thumbnail(*width, *height),
            ImageOperationStep::Blur(sigma) => image.blur(*sigma),
        };

        Ok(())
    }
}
