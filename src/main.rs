mod cli;
mod config;
mod r#const;
mod tmux_connector;

fn main() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let cli = cli::root::Cli::new();
    cli.execute();
}
