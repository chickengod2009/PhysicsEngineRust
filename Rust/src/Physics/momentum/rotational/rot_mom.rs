use std::fmt::Display;

use crate::Physics::{Vector, force::torque::Torque, momentum::rotational::var::RotVar, unit, vars::Var};


#[derive(Clone)]
pub struct RotationalMomentum{
    l: unit,
    i: unit,
    w: unit
}


impl RotationalMomentum{

    pub fn create(moment : unit)->Self{
        if moment ==0.0{panic!()}
        Self { l:0.0, i: moment, w: 0.0 }
    }

    

    
    pub fn with_l(mut self, momenta: unit) -> Self{
        self.l = momenta;
        self
    }
    
    pub fn with_w(mut self, vel: unit) -> Self{
        self.w= vel;
        self
    }

    pub fn w_mut(&mut self) -> &mut unit{
        &mut self.w
    }
    

    
    fn calc_w(&mut self) {
        
        self.w = self.l/self.i;
                
    }
    fn cacl_rot_moment(&mut self){
        self.l = self.w*self.i;
    }   
    pub fn w(&self) -> unit{
        self.w
    }

    pub fn impulse(&mut self, torque : &Torque, time : unit){
        
        let impulse = torque.torque()*time;
        let change_v = impulse/self.i;
        self.w-=change_v;
    }

    pub fn moment_of_inertia(&self) -> unit{
        self.i
    }

    

}

pub struct RotErr;
/*
impl Vector for RotationalMomentum {
    type Output=unit;

    type Error = RotErr;

    fn get_x(&mut self)-> Option<Self::Output> {
        todo!()
    }

    fn get_y(&mut self) -> Option<Self::Output> {
        todo!()
    }

    fn get_mag(&mut self) -> Option<Self::Output> {
        todo!()
    }

    fn calc_x(&mut self) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn calc_y(&mut self) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn calc_mag(&mut self) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn get_angle(&mut self) -> Option<Self::Output> {
        todo!()
    }

    fn calc_angle(&mut self) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}*/

impl Display for RotationalMomentum{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "L: {}\nhI: {}\nW: {}", self.l, self.i, self.w)
    }
}