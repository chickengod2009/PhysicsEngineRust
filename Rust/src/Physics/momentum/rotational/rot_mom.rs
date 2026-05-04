use crate::Physics::{Vector, momentum::{linear::var::LinVar, rotational::var::RotVar}, unit, vars::Var};



pub type RotationalMomentum = Var<RotVar, 7>;


impl RotationalMomentum{

    fn create()->Self{
        Self { index: RotVar::I, elements: [None;7], where_i: 0, size: 7 }
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