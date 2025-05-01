#[derive(Default, Clone, Debug)]
pub(crate) enum InputMode {
    #[default]
    Normal,
    Editing,
    SelectPartOfSpeech,
    SelectDefinition,
    Suggesting,
    Settings,
}
