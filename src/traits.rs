use crate::types::TextGenerator;

pub trait IntoTextGenerator {
    fn into(self) -> TextGenerator;
}
