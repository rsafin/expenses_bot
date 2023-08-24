pub struct Config {
    pub telegram_api_token: String
}

impl Config {
    pub fn new() -> Self {
        Config {
            telegram_api_token: String::from("6548349280:AAFNQmLjJc-F7LhElIn3OJFn2oM_RLpa6vM")
        }
    }
}