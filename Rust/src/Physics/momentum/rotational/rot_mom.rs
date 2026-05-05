use crate::Physics::{Vector, momentum::{rotational::var::RotVar}, unit, vars::Var};



pub type RotationalMomentum = Var<RotVar, 3>;


impl RotationalMomentum{

    pub fn create()->Self{
        Self { index: RotVar::I, elements: [None;3], where_i: 0, size: 3 }
    }

    pub fn with_mass(mut self, mass: unit)->Self{
        self.set(RotVar::I, mass).expect("Linear momenta with_mass");
        self
    }

    
    pub fn with_l(mut self, momenta: unit) -> Self{
        self.set(RotVar::L, momenta).expect("Linear momenta with_p");
        self
    }
    
    pub fn with_w(mut self, vel: unit) -> Self{
        self.set(RotVar::W, vel).expect("Linear momenta with_v");
        self
    }
    

    
    fn calc_w(&mut self) -> Result<unit, RotErr>{
        todo!()
    }
    fn calc_wx(&mut self) -> Result<unit, RotErr>{
        todo!()
    }
    fn calc_wy(&mut self) -> Result<unit, RotErr>{
        todo!()
    }

    

}

pub struct RotErr;

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
}