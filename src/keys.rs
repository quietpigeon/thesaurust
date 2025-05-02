use crate::{
    client::parse_response,
    models::{app::App, input_mode::InputMode},
};
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use tui_input::backend::crossterm::EventHandler;

pub(crate) fn key_handler(app: &mut App, key: KeyEvent) {
    match app.input_mode {
        InputMode::Normal => handle_normal(app, &key),
        InputMode::Editing => handle_editing(app, &key),
        InputMode::SelectPartOfSpeech => handle_select_pos(app, &key),
        InputMode::SelectDefinition => handle_select_definition(app, &key),
        InputMode::Suggesting => handle_suggesting(app, &key),
        InputMode::Settings => handle_settings(app, &key),
    }
}

fn handle_normal(app: &mut App, key: &KeyEvent) {
    match key.code {
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
    }
}

fn handle_editing(app: &mut App, key: &KeyEvent) {
    match key.code {
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
            app.input.handle_event(&Event::Key(*key));
        }
    }
}

fn handle_select_pos(app: &mut App, key: &KeyEvent) {
    match key.code {
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
    }
}

fn handle_select_definition(app: &mut App, key: &KeyEvent) {
    match key.code {
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
    }
}

fn handle_suggesting(app: &mut App, key: &KeyEvent) {
    match key.code {
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
    }
}

fn handle_settings(app: &mut App, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('q') => {
            app.input_mode = InputMode::Editing;
        }
        KeyCode::Char('h') | KeyCode::Char('l') => {
            app.is_spelling_fix_enabled = !app.is_spelling_fix_enabled;
        }
        _ => {}
    }
}
