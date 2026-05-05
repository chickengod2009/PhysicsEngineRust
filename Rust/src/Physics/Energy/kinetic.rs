use crate::Physics::unit;

pub struct KE{


    mass : unit,
    value : unit,
    velocity : unit,



}
//f

impl KE{
	fn new(mas: unit) -> Self{
        Self{
            mass: mas,
            value: 0 as unit,
            velocity: 0 as unit
            }
    }
    fn with_vel(mut self, vel: unit) -> Self{
    	self.velocity = vel;
        self.value = vel*vel*self.mass*(.5 as unit);
        self
    }
    fn change_vel(&mut self, vel:unit) -> unit{
    	self.velocity = vel;
        self.value = vel*vel*self.mass*(.5 as unit);
        self.velocity
    }
    fn mass(&self) -> unit{
        self.mass.copy()
    }
    fn velocity(&self) -> unit{
        self.velocity.copy()
    }
    fn value(&self)->unit{
        self.value.copy()
    }
    
        

}