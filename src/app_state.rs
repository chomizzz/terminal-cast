use std::process::Output;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::Mutex;

pub struct AppState {
    pub ip_cast: String,
    pub lien_http: String,
    pub logs: Vec<(String, bool)>,
    pub should_quit: bool,
    pub input: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            ip_cast: "192.168.1.16:9999".to_string(),
            lien_http: "...".to_string(),
            logs: vec![("En attente ...".to_string(), false)],
            should_quit: false,
            input: String::new(),
        }
    }
    pub async fn start_streaming_yt(&mut self, lien_http: &str, state: Arc<Mutex<AppState>>) {
        let ip = self.ip_cast.clone();
        let lien = lien_http.to_string();

        tokio::spawn(async move {
            let mut output = Command::new("ssh")
                .arg(format!("rpi5@{}", ip))
                .arg(format!(
                    "DISPLAY=:0 mpv --ytdl-format='bestvideo+bestaudio' '{}'",
                    lien
                ))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Impossible de lancer la commande");

            // NOTE: Ca se trouve mpv va rendre ici plutôt

            // let stderr = child.stderr.take().unwrap();
            // let reader = BufReader::new(stderr);
            let stdout = output.stdout.take().expect("Pas de stdout");
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                let mut app = state.lock().await;

                app.logs.push((line, false)); // stdout

                // éviter que ça grossisse à l'infini
                if app.logs.len() > 500 {
                    app.logs.remove(0);
                }
            }

            let _ = output.wait().await;
        });
    }
}
