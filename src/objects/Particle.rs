pub struct Particle {
    x: f64,
    y: f64,
    x_speed: f64,
    y_speed: f64,
    x_acceleration: f64,
    y_acceleration: f64,
    mass: f64,
    //forces: Vec,
}

impl PhysicsObject for Particle {
    fn simulate(&mut self, delta_time: i64) -> Result<(), String> {
        let new_x_speed = self.x_speed + delta_time as f64 * self.x_acceleration;
        let new_y_speed = self.y_speed + delta_time as f64 * self.y_acceleration;
        let new_x = self.x + delta_time as f64 * self.x_speed;
        let new_y = self.y + delta_time as f64 * self.y_speed;
        self.x = new_x;
        self.y = new_y;
        self.x_speed = new_x_speed;
        self.y_speed = new_y_speed;
        Ok(())
    }
}