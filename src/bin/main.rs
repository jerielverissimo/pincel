use pincel::core::application::{ApplicationConfig, ApplicationState};

fn main() {
    ApplicationState::create(ApplicationConfig {
        start_pos_x: 0,
        start_pos_y: 0,
        start_width: 1366,
        start_height: 768,
    });
    ApplicationState::run();
}
