pub trait PhysicsObject {
    fn simulate(&mut self, delta_time: i64) -> Result<(), String>;
}