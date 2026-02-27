use std::process::Output;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

// Dessine l'interface utilisateur complète
// fonction public qui implemente Backend, qui permet de dessiner gérer le cursuer et rafraichir
// l'écran
pub fn draw_ui<B: Backend>(f: &mut Frame<B>, input: &str, logs: &Vec<(String, bool)>) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Max(5),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(f.size());

    draw_logs_section(f, main_chunks[0], logs);
    draw_input_section(f, main_chunks[1], input);
}

fn draw_logs_section<B: Backend>(
    f: &mut Frame<B>,
    area: tui::layout::Rect,
    logs: &[(String, bool)],
) {
    let items: Vec<ListItem> = logs
        .iter()
        .map(|(line, is_error)| ListItem::new(Span::raw(line.clone())))
        .collect();
    let logs_widget = List::new(items).block(
        Block::default()
            .title("Runtime")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(logs_widget, area)
}

/// Dessine la section de saisie
fn draw_input_section<B: Backend>(f: &mut Frame<B>, area: tui::layout::Rect, input: &str) {
    let input_widget = Paragraph::new(input)
        .block(
            Block::default()
                .title("⌨️  Commande (Entrée pour envoyer, Échap pour quitter)")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Green)),
        )
        .style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(input_widget, area);
}
