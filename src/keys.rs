use crate::client;
use crate::models::{app::App, errors::Error, input_mode::InputMode};
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use tui_input::backend::crossterm::EventHandler;

pub(crate) fn key_handler(app: &mut App, key: KeyEvent) -> Result<(), Error> {
    match app.input_mode {
        InputMode::Normal => handle_normal(app, &key),
        InputMode::Editing => handle_editing(app, &key),
        InputMode::SelectPartOfSpeech => handle_select_pos(app, &key),
        InputMode::SelectDefinition => handle_select_definition(app, &key),
        InputMode::Settings => handle_settings(app, &key),
    }
}

fn handle_normal(app: &mut App, key: &KeyEvent) -> Result<(), Error> {
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

    Ok(())
}

fn handle_editing(app: &mut App, key: &KeyEvent) -> Result<(), Error> {
    match key.code {
        KeyCode::Enter => {
            app.input_mode = InputMode::Normal;
            let results = client::look_up(&app.input.to_string())?;
            app.results = results.0;
            App::update_all(app);
        }
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        _ => {
            app.input.handle_event(&Event::Key(*key));
        }
    }

    Ok(())
}

fn handle_select_pos(app: &mut App, key: &KeyEvent) -> Result<(), Error> {
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

    Ok(())
}

fn handle_select_definition(app: &mut App, key: &KeyEvent) -> Result<(), Error> {
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

    Ok(())
}

fn handle_settings(app: &mut App, key: &KeyEvent) -> Result<(), Error> {
    match key.code {
        KeyCode::Char('q') => {
            app.input_mode = InputMode::Editing;
        }
        KeyCode::Char('h') | KeyCode::Char('l') => {
            app.is_spelling_fix_enabled = !app.is_spelling_fix_enabled;
        }
        _ => {}
    }

    Ok(())
}
