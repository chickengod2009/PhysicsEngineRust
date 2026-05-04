use std::{fmt::{Debug}};

pub mod vars;
pub mod force;
pub mod momentum;
pub mod Energy;
pub mod objects;

type unit = f64;
pub trait Vector{
    type Output;
    type Error; //where Self::Error:Debug;
    fn get_x(&mut self)-> Option<Self::Output>;
    fn get_y(&mut self) -> Option<Self::Output>;
    fn get_mag(&mut self) -> Option<Self::Output>;
    fn calc_x(&mut self) -> Result<Self::Output, Self::Error>;
    fn calc_y(&mut self) -> Result<Self::Output, Self::Error>;
    fn calc_mag(&mut self) -> Result<Self::Output, Self::Error>;
    fn get_angle(&mut self) -> Option<Self::Output>;
    fn calc_angle(&mut self) -> Result<Self::Output, Self::Error>;
}