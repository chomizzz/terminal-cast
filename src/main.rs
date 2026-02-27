mod app_state;
mod tui;
use app_state::AppState;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tui::KeyboardAction;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = tui::setup_terminal()?;

    let app_state = Arc::new(Mutex::new(AppState::new()));

    loop {
        {
            let app = app_state.lock().await;

            if app.should_quit {
                break;
            }

            terminal.draw(|f| {
                tui::draw_ui(f, &app.input, &app.logs);
            })?;
        }

        tokio::select! {
            _ = tokio::time::sleep(Duration::from_millis(10)) => {
                let mut app = app_state.lock().await;
                match tui::handle_keyboard_events(&mut app.input)? {
                    KeyboardAction::Quit => app.should_quit = true,
                    KeyboardAction::SendCommand(command) => {
                        let app_clone = Arc::clone(&app_state);
                        app.start_streaming_yt(&command, app_clone).await;
                    }
                _ => {}
                }
            }
        }
    }
    println!("Au revoir ! 👋");

    Ok(())
}
