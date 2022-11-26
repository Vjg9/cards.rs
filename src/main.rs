use cards::ui;
use cards::config;

fn main() {
    config::init();
    ui::run_ui().unwrap();
}
