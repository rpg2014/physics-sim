pub trait Force {
    fn update(&mut self) -> Result<(), String>;
    fn get_force_value() -> Result<f64, String>;
}