#[derive(Clone, Debug)]
pub(crate) enum InputMode {
    Normal(NormalMode),
    Editing(EditingMode),
    SelectPartOfSpeech(SelectPartOfSpeechMode),
    SelectDefinition(SelectDefinitionMode),
    Suggesting(SuggestionMode),
    Settings(SettingsMode),
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Normal(NormalMode::default())
    }
}

#[derive(Default, Clone, Debug)]
pub(crate) struct NormalMode;
#[derive(Default, Clone, Debug)]
pub(crate) struct EditingMode;
#[derive(Default, Clone, Debug)]
pub(crate) struct SelectPartOfSpeechMode;
#[derive(Default, Clone, Debug)]
pub(crate) struct SelectDefinitionMode;
#[derive(Default, Clone, Debug)]
pub(crate) struct SuggestionMode;
#[derive(Default, Clone, Debug)]
pub(crate) struct SettingsMode;
