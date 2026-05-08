use crate::Physics::unit;

pub struct KE{


    mass : unit,
    value : unit,
    velocity : unit,



}
//f

impl KE{
	pub fn new(mas: unit) -> Self{
        Self{
            mass: mas,
            value: 0 as unit,
            velocity: 0 as unit
            }
    }
    pub fn with_vel(mut self, vel: unit) -> Self{
    	self.velocity = vel;
        self.value = vel*vel*self.mass*(0.5);
        self
    }
    pub fn change_mass(&mut self, mas: unit) -> unit{
    	self.mass = mas;
        self.value = self.velocity*self.velocity*self.mass*(0.5 as unit);
        self.mass.clone()
    }
    pub fn change_vel(&mut self, vel:unit) -> unit{
    	self.velocity = vel;
        self.value = vel*vel*self.mass*(0.5 as unit);
        self.velocity.clone()
    }
    pub fn mass(&self) -> unit{
        self.mass.clone()
    }
    pub fn velocity(&self) -> unit{
        self.velocity.clone()
    }
    pub fn value(&self)->unit{
        self.value.clone()
    }

    pub fn calc(&mut self)->unit{
        self.value = self.velocity*self.velocity*self.mass*(0.5 as unit);
        self.value.clone()
    }
    pub fn calc_new_v(&mut self, vel: unit) -> unit{
        self.velocity=vel;
        self.value = vel*vel*self.mass*(0.5 as unit);
        self.value.clone()
    }
    
        

}