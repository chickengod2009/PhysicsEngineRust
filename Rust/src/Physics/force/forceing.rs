use std::{fmt::Debug, ops::Add};

use super::super::vars::Var;

use crate::Physics::{Vector, force::variable::ForceIndex, unit};

pub type Force = Var<ForceIndex, 7>;

impl Force{
    pub fn new_force(mass : unit)->Self{
        let mut s =Self{
            index: ForceIndex::A,
            elements: [None; 7],
            where_i: 0,
            size :7
        };
        s.set(ForceIndex::M, mass);
        s

    }

    
}

impl Add for Force{
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            index: ForceIndex::A,
            elements: {
                let mut lg: [Option<unit>; 7] = [None; 7];
                for i in 0..7{
                    if let Some(a) =rhs.elements[i] && let Some(b) = self.elements[i] {
                        lg[i] = Some(a+b);
                    }
                };

                lg

            },
            where_i:0,
            size :5

        }
    }
    
    type Output= Force;
}

impl Vector for Force {
    type Output = unit;

    type Error = ForceErr;

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

pub struct ForceErr;

impl Debug for ForceErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ForceErr").finish()
    }
}

impl Force{

    fn calc_acc(& mut self ) -> Result<i32, ForceErr>{
        todo!()
    }
    fn calc_acc_x(& mut self ) -> Result<i32, ForceErr>{
        todo!()
    }
    fn calc_acc_y(& mut self ) -> Result<i32, ForceErr>{
        todo!()
    }
    fn calc_acc_ang(& mut self ) -> Result<i32, ForceErr>{
        todo!()
    }

}