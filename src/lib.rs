#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod piano_test;
pub use app::TemplateApp;
pub use piano_test::play_test;
pub use piano_test::play_sink_test;
