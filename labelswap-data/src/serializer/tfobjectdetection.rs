use super::{FormatSerializer}

pub struct TfObjectDetectionSerializer {}

impl FormatSerializer for TfObjectDetectionSerializer {
    fn init(&mut self, path: impl Into<std::path::PathBuf>) -> anyhow::Result<()> {
        todo!()
    }

    fn push(&mut self, annotation: crate::models::Annotation) -> anyhow::Result<()> {
        todo!()
    }

    fn finish(self) -> anyhow::Result<()> {
        todo!()
    }
}
