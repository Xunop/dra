use indicatif::{ProgressBar, ProgressStyle};

use crate::cli::color::Color;

// NOTE: spinner ticks and duration are from
// https://github.com/sindresorhus/cli-spinners/blob/00de8fbeee16fa49502fa4f687449f70f2c8ca2c/spinners.json#L2-L16
pub const TICKS: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
pub const TICK_DURATION: u64 = 80;

pub struct Spinner {
    pb: ProgressBar,
    end_message: String,
}

impl Spinner {
    pub fn new(message: String, end_message: String) -> Self {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(TICKS)
                .template("{spinner:.blue} {msg}"),
        );
        pb.set_message(message);
        Self { pb, end_message }
    }

    pub fn start(&self) {
        self.pb.enable_steady_tick(TICK_DURATION);
    }

    pub fn stop(&self) {
        self.pb.finish_and_clear();
        println!("{}", &self.end_message);
    }

    pub fn stop_with_message(&self, message: &str) {
        self.pb.finish_and_clear();
        println!("{}", message);
    }

    pub fn install() -> Spinner {
        Spinner::new(
            "Installing".into(),
            format!("{}", Color::new("Installation completed!").green()),
        )
    }

    pub fn no_messages() -> Spinner {
        Spinner::new(String::new(), String::new())
    }
}
