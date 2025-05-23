use crate::client::spellcheck;
use crate::models::{app::App, errors::Error, input_mode::InputMode};
use apply::Apply;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use tui_input::backend::crossterm::EventHandler;

pub(crate) fn key_handler(app: &mut App, key: KeyEvent) -> Result<(), Error> {
    match app.input_mode {
        InputMode::Normal => handle_normal(app, &key),
        InputMode::Insert => handle_editing(app, &key),
        InputMode::SelectPartOfSpeech => handle_select_pos(app, &key),
        InputMode::SelectDefinition => handle_select_definition(app, &key),
    }
}

fn handle_normal(app: &mut App, key: &KeyEvent) -> Result<(), Error> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => App::quit(app),
        KeyCode::Char('j') | KeyCode::Char('k') if !app.results.is_empty() => {
            app.input_mode = InputMode::SelectPartOfSpeech
        }
        KeyCode::Char('l') | KeyCode::Char('h') if app.part_of_speech_list.items.len() == 1 => {
            app.input_mode = InputMode::SelectDefinition;
        }
        KeyCode::Char('/') => {
            app.input_mode = InputMode::Insert;
            app.input.reset();
        }
        _ => {}
    }

    Ok(())
}

fn handle_editing(app: &mut App, key: &KeyEvent) -> Result<(), Error> {
    match key.code {
        KeyCode::Enter => {
            app.input_mode = InputMode::Normal;
            let word = app.input.to_string().apply_ref(|w| spellcheck(w))?;
            let results = crate::client::look_up(&word)?;
            // NOTE: This fixes the appearance of the word in the search bar if the word is
            // misspelled. I'm not sure if this would be the preferred approach for everyone, so
            // let's leave it as it is for now. An option that allows users to select other
            // variants would be nice.
            app.input = word.into();
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
        KeyCode::Char('j') | KeyCode::Down => {
            app.part_of_speech_list.down();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.part_of_speech_list.up();
        }
        KeyCode::Char('q') | KeyCode::Esc => {
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
        KeyCode::Char('l') | KeyCode::Right | KeyCode::Char('j') | KeyCode::Down => {
            app.definition_list.down();
            App::update_synonym_list(app);
        }
        KeyCode::Char('h') | KeyCode::Left | KeyCode::Char('k') | KeyCode::Up => {
            app.definition_list.up();
            App::update_synonym_list(app);
        }
        KeyCode::Char('q') | KeyCode::Esc => {
            app.input_mode = InputMode::SelectPartOfSpeech;
            app.definition_list.state.select(Some(0));
            App::update_synonym_list(app);
        }
        KeyCode::Char('/') => {
            app.input_mode = InputMode::Insert;
            app.input.reset();
        }
        _ => {}
    }

    Ok(())
}
