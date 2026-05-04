use crate::Physics::{Vector, momentum::linear::var::LinVar, unit, vars::Var};



pub type LinearMomentum = Var<LinVar, 7>;


impl LinearMomentum{

    fn create()->Self{
        Self { index: LinVar::M, elements: [None;7], where_i: 0, size: 7 }
    }

    
    fn calc_v(&mut self) -> Result<unit, LinErr>{
        todo!()
    }
    fn calc_vx(&mut self) -> Result<unit, LinErr>{
        todo!()
    }
    fn calc_vy(&mut self) -> Result<unit, LinErr>{
        todo!()
    }

    

}

pub struct LinErr;

impl Vector for LinearMomentum{
    type Output =unit;

    type Error = LinErr;

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