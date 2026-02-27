use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

/// Résultat du traitement d'un événement clavier
pub enum KeyboardAction {
    Quit,
    SendCommand(String),
    UpdateInput(String),
    None,
    SpecialCommand(String),
}

/// Gère les événements du clavier et retourne l'action à effectuer
pub fn handle_keyboard_events(input: &mut String) -> Result<KeyboardAction, std::io::Error> {
    // Vérifier s'il y a un événement clavier (non-bloquant)
    if event::poll(Duration::from_millis(0))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                // Quitter avec Échap
                KeyCode::Esc => {
                    return Ok(KeyboardAction::Quit);
                }

                // Envoyer la commande avec Entrée
                KeyCode::Enter => {
                    if !input.is_empty() {
                        let command = input.clone();
                        input.clear();
                        if command.starts_with('/') {
                            return Ok(KeyboardAction::SpecialCommand(command));
                        } else {
                            return Ok(KeyboardAction::SendCommand(command));
                        }
                    }
                }

                // Effacer un caractère
                KeyCode::Backspace => {
                    input.pop();
                    return Ok(KeyboardAction::UpdateInput(input.clone()));
                }

                // Ajouter un caractère
                KeyCode::Char(c) => {
                    input.push(c);
                    return Ok(KeyboardAction::UpdateInput(input.clone()));
                }

                _ => {}
            }
        }
    }

    Ok(KeyboardAction::None)
}
