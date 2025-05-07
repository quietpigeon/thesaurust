#[derive(Default, Clone, Debug)]
pub(crate) enum InputMode {
    #[default]
    Normal,
    Insert,
    SelectPartOfSpeech,
    SelectDefinition,
}
