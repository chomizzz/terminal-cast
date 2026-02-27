// Ici les les crate importé sont pour activier et desactiver la souris, et execute pour créer des
// macro
// Raw mode permet de capturer directement les input du clavier.
// Alternate screen permet de de ne pas écraser le terminal d'avant
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
// module d'entrée et sortie pour lire et ecrire
use std::io;
// bibliotheque pour créer des interfaces dynamique
use tui::{backend::CrosstermBackend, Terminal};

/// Initialise le terminal en mode TUI, retourne un Terminal ou une erreur
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    enable_raw_mode()?;
    // Ici mutable car c'est le robinet vers la sortie, vers le terminal pour afficher des
    // choses dedans
    let mut stdout = io::stdout();
    // On passe en mode écran alternatif pour le terminal, et on écoute les évenements de la
    // souris
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    // crée un terminal et return le terminal créé
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

/// Restaure le terminal à son état normal
pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()
}
