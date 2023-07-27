pub trait LanguageModel {
    fn run(&self, input: &str) -> f64;
}
