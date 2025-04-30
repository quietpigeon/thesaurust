use crate::{
    client::parse_response,
    models::app::{App, InputMode},
};
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use tui_input::backend::crossterm::EventHandler;

pub(crate) fn key_handler(app: &mut App, key: KeyEvent) {
    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('q') => {
                App::quit(app);
            }
            KeyCode::Char('j') | KeyCode::Char('k') if !app.results.is_empty() => {
                app.input_mode = InputMode::SelectPartOfSpeech;
            }
            KeyCode::Char('l') | KeyCode::Char('h') if app.part_of_speech_list.items.len() == 1 => {
                app.input_mode = InputMode::SelectDefinition;
            }
            KeyCode::Char('/') => {
                app.input_mode = InputMode::Editing;
                app.input.reset();
            }
            KeyCode::Char(':') => {
                app.input_mode = InputMode::Settings;
            }
            _ => {}
        },
        InputMode::Editing => match key.code {
            KeyCode::Enter => {
                app.input_mode = InputMode::Normal;
                let results =
                    parse_response(app.input.to_string().as_str(), &app.is_spelling_fix_enabled);
                app.results = results.t;
                if let Some(word) = app.results[0].clone().word {
                    if results.is_spelling_suggested {
                        app.suggested_spelling = word;
                        app.input_mode = InputMode::Suggesting;
                    }
                }
                App::update_all(app);
            }
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
            }
            _ => {
                app.input.handle_event(&Event::Key(key));
            }
        },
        InputMode::SelectPartOfSpeech => match key.code {
            KeyCode::Char('j') => {
                app.part_of_speech_list.down();
            }
            KeyCode::Char('k') => {
                app.part_of_speech_list.up();
            }
            KeyCode::Char('q') => {
                app.input_mode = InputMode::Normal;
            }
            KeyCode::Enter => {
                app.input_mode = InputMode::SelectDefinition;
                App::update_definition_list(app);
                App::update_synonym_list(app);
            }
            _ => {}
        },
        InputMode::SelectDefinition => match key.code {
            KeyCode::Char('l') => {
                app.definition_list.down();
                App::update_synonym_list(app);
            }
            KeyCode::Char('h') => {
                app.definition_list.up();
                App::update_synonym_list(app);
            }
            KeyCode::Char('q') => {
                app.input_mode = InputMode::Normal;
                app.definition_list.state.select(Some(0));
                App::update_synonym_list(app);
            }
            KeyCode::Char('/') => {
                app.input_mode = InputMode::Editing;
                app.input.reset();
            }
            _ => {}
        },
        InputMode::Suggesting => match key.code {
            KeyCode::Char('y') | KeyCode::Enter => {
                app.input_mode = InputMode::Normal;
                let results = parse_response(&app.suggested_spelling, &app.is_spelling_fix_enabled);
                // Prevents Serp API from suggesting the same word repeatedly.
                if !results.is_spelling_suggested {
                    app.results = results.t;
                    App::update_all(app);
                }
            }
            KeyCode::Char('n') | KeyCode::Char('q') => {
                app.input_mode = InputMode::Normal;
            }
            _ => {}
        },
        InputMode::Settings => match key.code {
            KeyCode::Char('q') => {
                app.input_mode = InputMode::Editing;
            }
            KeyCode::Char('h') | KeyCode::Char('l') => {
                app.is_spelling_fix_enabled = !app.is_spelling_fix_enabled;
            }
            _ => {}
        },
    }
}
