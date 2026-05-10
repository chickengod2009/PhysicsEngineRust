use std::ops::AddAssign;

use crate::Physics::{Vector, force::forceing::Force, objects::polygons::{Point, Vect}, unit, vars::{Var, index_get}};
#[derive(Clone)]
pub struct Torque{
    force : Force,
    r: Vect,
    torque: unit,
    alpha : unit,
    moment : unit
}

#[derive(Clone)]
pub enum VarTor{
    T,A,I
}
impl index_get for VarTor{
    fn as_usize(&self)-> usize {
        match self {
            &VarTor::A => 0,
            &VarTor::I => 1,
            &VarTor::T => 2
        }
    }
}
pub type TorqueVariable = Var<VarTor, 3>;
impl TorqueVariable{
    pub fn new_torque() -> Self{
        Self { index: VarTor::A, elements: [None; 3], where_i: 0, size: 3 }
    }
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
        let tor = (self.r.x()*self.force.y().unwrap())-(self.force.x().unwrap()*self.r.x());
        self.torque=tor;

    }
    pub fn alpha(&self) -> unit{
        self.alpha
    }
    pub fn calc_alpha(&mut self){
        self.alpha = self.torque/self.moment;
        
    }
}

impl AddAssign<&Torque> for Torque{
    fn add_assign(&mut self, rhs: &Torque) {
        self.torque+=rhs.torque;
        self.calc_alpha();
    }
}