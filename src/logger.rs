pub  struct Logger {

}

impl Logger {
    pub fn log(&self, message: String) {
        println!("LOGGER: {}", message);
    }
}