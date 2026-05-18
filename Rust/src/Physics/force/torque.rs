use std::{fmt::Display, ops::AddAssign};

use crate::Physics::{Vector, force::forceing::Force, objects::polygons::{Point, Vect}, unit, vars::{Var, index_get}};
#[derive(Clone)]
pub struct Torque{
    force : Force,
    r: Vect,
    torque: unit,
    alpha : unit,
    moment : unit
}





impl Torque {
    pub fn new_with_force(force : Force, r: Vect, mom : unit) -> Self{
        let mut ret = Self { force: force, r: r, alpha : 0.0, moment: mom, torque:0.0 };
        ret.torque_by_force();
        ret.calc_alpha();
        ret
    }
    pub fn torque(&self) ->unit{
        self.torque
    }
    pub fn torque_by_force(&mut self){
        let tor = (self.r.x()*self.force.y().unwrap())*0.001-(self.force.x().unwrap()*self.r.y())*0.001;
        self.torque=tor;

    }
    pub fn alpha(&self) -> unit{
        self.alpha
    }
    pub fn calc_alpha(&mut self){
        self.alpha = self.torque/self.moment;
        
    }
    pub fn i(&self)-> unit{
        self.moment
    }
}

impl AddAssign<&Torque> for Torque{
    fn add_assign(&mut self, rhs: &Torque) {
        self.torque+=rhs.torque;
        self.calc_alpha();
    }
}

impl Display for Torque{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "T: {}\nAlpha: {}\nI: {}", self.torque, self.alpha, self.moment)
    }
}