use clap::Parser;
use std::process;
use std::sync::{Arc, Mutex};
use tokio::process::Command;

// Module declarations
mod protocols;
mod wayland_client;
mod wayland_object;
mod terminal_window;

// Use statements
use wayland_client::WaylandClient;
use terminal_window::{TerminalWindow, AppState};

/// Represents the command-line arguments.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "wayland-1")]
    wayland_display_name: String,
    #[arg(long, default_value = "/bin/bash")]
    shell: String,
    #[arg(allow_hyphen_values = true)]
    positionals: Vec<String>,
}

/// This is a conceptual loop for handling input from stdin.
async fn input_loop(_app_state: Arc<Mutex<AppState>>) {
    println!("Input loop started (stub).");
    // This loop would read stdin and call methods on app_state to dispatch events.
}


// --- Main Application Logic ---

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // The shared state for the entire application.
    let app_state = Arc::new(Mutex::new(AppState {
        clients: Vec::new(),
    }));

    // Create and run the main rendering window.
    let mut terminal_window = TerminalWindow::new(Arc::clone(&app_state));
    let terminal_handle = tokio::spawn(async move {
        terminal_window.main_loop().await;
    });

    // This is the main server loop for accepting new Wayland clients.
    let server_handle = tokio::spawn({
        let app_state = Arc::clone(&app_state);
        async move {
            println!("Wayland server starting on socket '{}' (simulated).", &args.wayland_display_name);
            // In a real app, this would be a loop calling `accept()`.
            // For now, we simulate one client connecting.
            let mut client = WaylandClient::new(1, 99);
            app_state.lock().unwrap().clients.push(Arc::new(Mutex::new(client)));
            // In a real app, we would spawn a task for each client's main_loop.
            println!("Client handler spawned (stub).");
        }
    });

    // Spawn the conceptual input loop
    let input_handle = tokio::spawn(input_loop(Arc::clone(&app_state)));

    println!("All main loops (server, renderer, input) are running.");

    if !args.positionals.is_empty() {
        let mut command = Command::new(&args.shell);
        command.arg("-c").arg(args.positionals.join(" "));
        command.env("WAYLAND_DISPLAY", &args.wayland_display_name);

        match command.spawn() {
            Ok(child) => println!("Successfully spawned command with PID: {:?}", child.id()),
            Err(e) => {
                eprintln!("Failed to spawn command: {}", e);
                process::exit(1);
            }
        }
    }

    println!("Compositor running. Waiting for tasks to finish.");
    let _ = tokio::try_join!(server_handle, terminal_handle, input_handle);
    println!("Compositor shutting down.");
}
