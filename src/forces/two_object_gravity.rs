pub struct Gravity {
    force: f64,

}

impl Force for Gravity {
    fn update(&mut self)
}

impl Gravity {
    pub fn get_force(object1: <T: PhysicsObject>, object2: <U: PhysicsObject>) -> Result<f64, String> {
        
    }
}