use super::{FormatSerializer, SerializerResult};

pub struct TfObjectDetectionSerializer {}

impl FormatSerializer for TfObjectDetectionSerializer {
    fn init(&mut self, path: impl Into<std::path::PathBuf>) -> SerializerResult<()> {
        todo!()
    }

    fn push(&mut self, annotation: crate::models::Annotation) -> SerializerResult<()> {
        todo!()
    }

    fn finish(self) -> SerializerResult<()> {
        todo!()
    }
}
