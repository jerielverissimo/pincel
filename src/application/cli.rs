pub struct Cli {
    pattern: Option<String>,
}

impl Cli {
    pub fn new() -> Self {
        let pattern = std::env::args().nth(1);
        Self { pattern }
    }
}
