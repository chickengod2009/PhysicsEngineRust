use core::f64;
use std::{fmt::{Debug, Display, write}, ops::{Add, AddAssign}};

use super::super::vars::Var;

use crate::Physics::{Vector, force::variable::ForceIndex, objects::polygons::{Point, Vect}, unit, vars::index_get};

pub type Force = Var<ForceIndex, 8>;

impl Force{
    pub fn new_force(mass : unit)->Self{
        let mut s =Self{
            index: ForceIndex::A,
            elements: [None; 8],
            where_i: 0,
            size :8
        };
        s.set(ForceIndex::M, mass);
        s

    }

    pub fn set_some_all(mut self) -> Self{
        let m = self.elements[ForceIndex::M.as_usize()];
        for i in self.elements.iter_mut(){
            *i = Some(0.0);
        }
        self.elements[ForceIndex::M.as_usize()] = m;
        self
    }

    pub fn inverse(mut self) -> Self{
        self.set(ForceIndex::Ang, self[ForceIndex::Ang].unwrap() + f64::consts::PI);
        self.calc_x();
        self.calc_y();
        self
    }

    pub fn ay_from_fy(&mut self){
        self[ForceIndex::Ay] = Some(self.x_over_y(self[ForceIndex::Fy].unwrap_or(0.0), self[ForceIndex::M].unwrap_or(0.0)));
    }

    
}

impl AddAssign<&Force> for Force{
    fn add_assign(&mut self, rhs: &Self) {
        self[ForceIndex::Ax] = self[ForceIndex::Ax].zip(rhs[ForceIndex::Ax]).map(
            |(x,xx)| -> unit{
                x+xx
            }
        );
        self[ForceIndex::Ay] = self[ForceIndex::Ay].zip(rhs[ForceIndex::Ay]).map(
            |(x,xx)| -> unit{
                x+xx
            }
        );
        self[ForceIndex::Fx] = self[ForceIndex::Fx].zip(rhs[ForceIndex::Fx]).map(
            |(x,xx)| -> unit{
                
                x+xx

            }
        );
        self[ForceIndex::Fy] = self[ForceIndex::Fy].zip(rhs[ForceIndex::Fy]).map(
            |(x,xx)| -> unit{
                x+xx
            }
            
        );
        self.calc_mag().expect("48 forceing");
        self.calc_acc().expect("49 forceing");
        self.calc_angle().expect("50 forceing");
    }
    
    
}

impl Vector for Force {
    type Output = unit;

    type Error = ForceErr;

    fn x(&self)-> Option<Self::Output> {
        self.elements[ForceIndex::Fx.as_usize()]
    }

    fn y(&self) -> Option<Self::Output> {
        self.elements[ForceIndex::Fy.as_usize()]
    }

    fn mag(&self) -> Option<Self::Output> {
        self.elements[ForceIndex::F.as_usize()]
    }

    fn calc_x(&mut self) -> Result<Self::Output, Self::Error> {
        let res: Option<unit> = self[ForceIndex::F].clone().zip(self[ForceIndex::Ang].clone()).map(
            
            |(f,fy)| -> unit{
                self.mag_times_cos(f, fy)
            }
        ).or_else(
            || -> Option<unit>{
                self[ForceIndex::F].clone().zip(self[ForceIndex::Fy].clone()).map(
                    |(f, ang)| -> unit{
                        self.rev_pyth(f, ang)
                    }
                )
            }
        ).or_else(
            ||-> Option<unit>{
                self[ForceIndex::Ax].clone().zip(self[ForceIndex::M].clone()).map(
                    |(ax,m)| -> unit{
                        self.x_times_y(ax, m)
                    }
                )
            }
        );
        if let Some(a) = res{
            self.set(ForceIndex::Fx, a);
            return Ok(a);
        }

        Err(ForceErr)
    }

    fn calc_y(&mut self) -> Result<Self::Output, Self::Error> {
        let res: Option<unit> = self[ForceIndex::F].clone().zip(self[ForceIndex::Ang].clone()).map(
            |(a,ay)| -> unit{
                self.mag_times_sin(a, ay)
            }
        ).or_else(
            || -> Option<unit>{
                self[ForceIndex::F].clone().zip(self[ForceIndex::Fx].clone()).map(
                    |(a, ang)| -> unit{
                        self.rev_pyth(a, ang)
                    }
                )
            }
        ).or_else(
            ||-> Option<unit>{
                self[ForceIndex::Ay].clone().zip(self[ForceIndex::M].clone()).map(
                    |(fx,m)| -> unit{
                        self.x_times_y(fx, m)
                    }
                )
            }
        );
        if let Some(a) = res{
            self.set(ForceIndex::Fy, a);
            return Ok(a);
        }

        Err(ForceErr)
    }

    fn calc_mag(&mut self) -> Result<Self::Output, Self::Error> {
        let res : Option<unit>= self[ForceIndex::Fx].clone().zip(self[ForceIndex::Fy].clone()).map(
            |(ax,ay)| -> unit{
                self.solve_pyth(ax, ay)
            }
        ).or_else(
            || -> Option<unit>{
                self[ForceIndex::Fx].clone().zip(self[ForceIndex::Ang].clone()).map(
                    |(ax,ang)|  -> unit{
                        self.x_over_cos(ax, ang)
                    }
                )
            }
        ).or_else(
            || -> Option<unit>{
                self[ForceIndex::Fy].clone().zip(self[ForceIndex::Ang].clone()).map(
                    |(ay, ang)| -> unit{
                        self.y_over_sin(ay, ang)
                    }
                )
            }
        ).or_else(
            || -> Option<unit>{
                self[ForceIndex::A].clone().zip(self[ForceIndex::M].clone()).map(
                    |(f,m)| -> unit{
                        self.x_times_y(f, m)
                    }
                )
            }    
        );

        if let Some(a) = res{
            self.set(ForceIndex::F, a);
            return Ok(a);
        }



        Err(ForceErr{})
    }

    fn get_angle(&mut self) -> Option<Self::Output> {
        self[ForceIndex::Ang]
    }

    fn calc_angle(&mut self) -> Result<Self::Output, Self::Error> {
        let res : Option<unit> = self[ForceIndex::Fx].clone().zip(self[ForceIndex::Fy].clone()).map(
            |(x,y)| -> unit{
                y.atan2(x)
            }
        ).or_else(
            || -> Option<unit>{
                self[ForceIndex::F].clone().zip(self[ForceIndex::Fx].clone()).map(
                    |(f,fx)| -> unit{
                        (fx/f).acos()
                    }
                )
            }
        ).or_else(
            || -> Option<unit>{
                self[ForceIndex::F].clone().zip(self[ForceIndex::Fy].clone()).map(
                    |(f,fy)| -> unit{
                        (fy/f).acos()
                    }
                )
            }
        ).or_else(
            || -> Option<unit>{
                self[ForceIndex::A].clone().zip(self[ForceIndex::Ax].clone()).map(
                    |(f,fx)| -> unit{
                        (fx/f).acos()
                    }
                )
            }
        ).or_else(
            || -> Option<unit>{
                self[ForceIndex::A].clone().zip(self[ForceIndex::Ay].clone()).map(
                    |(f,fy)| -> unit{
                        (fy/f).acos()
                    }
                )
            }
        );

        if let Some(a) = res{
            let ang = a;
            self.set(ForceIndex::Ang, ang);
            return Ok(ang);
        }

        Err(ForceErr)
    }


}

pub struct ForceErr;

impl Debug for ForceErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ForceErr").finish()
    }
}

impl Force{

    fn calc_acc(& mut self ) -> Result<unit, ForceErr>{
        let res : Option<unit>= self[ForceIndex::Ax].clone().zip(self[ForceIndex::Ay].clone()).map(
            |(ax,ay)| -> unit{
                self.solve_pyth(ax, ay)
            }
        ).or_else(
            || -> Option<unit>{
                self[ForceIndex::Ax].clone().zip(self[ForceIndex::Ang].clone()).map(
                    |(ax,ang)|  -> unit{
                        self.x_over_cos(ax, ang)
                    }
                )
            }
        ).or_else(
            || -> Option<unit>{
                self[ForceIndex::Ay].clone().zip(self[ForceIndex::Ang].clone()).map(
                    |(ay, ang)| -> unit{
                        self.y_over_sin(ay, ang)
                    }
                )
            }
        ).or_else(
            || -> Option<unit>{
                self[ForceIndex::F].clone().zip(self[ForceIndex::M].clone()).map(
                    |(f,m)| -> unit{
                        self.x_over_y(f, m)
                    }
                )
            }    
        );

        if let Some(a) = res{
            self.set(ForceIndex::A, a);
            return Ok(a);
        }



        Err(ForceErr{})
    }
    fn calc_acc_x(& mut self ) -> Result<unit, ForceErr>{
        let res: Option<unit> = self[ForceIndex::A].clone().zip(self[ForceIndex::Ay].clone()).map(
            |(a,ay)| -> unit{
                self.rev_pyth(a, ay)
            }
        ).or_else(
            || -> Option<unit>{
                self[ForceIndex::A].clone().zip(self[ForceIndex::Ang].clone()).map(
                    |(a, ang)| -> unit{
                        self.mag_times_cos(a, ang)
                    }
                )
            }
        ).or_else(
            ||-> Option<unit>{
                self[ForceIndex::Fx].clone().zip(self[ForceIndex::Ay].clone()).map(
                    |(fx,m)| -> unit{
                        self.x_over_y(fx, m)
                    }
                )
            }
        );
        if let Some(a) = res{
            self.set(ForceIndex::Ax, a);
            return Ok(a);
        }

        Err(ForceErr)
    }
    fn calc_acc_y(& mut self ) -> Result<unit, ForceErr>{
        let res: Option<unit> = self[ForceIndex::A].clone().zip(self[ForceIndex::Ax].clone()).map(
            |(a,ax)| -> unit{
                self.rev_pyth(a, ax)
            }
        ).or_else(
            || -> Option<unit>{
                self[ForceIndex::A].clone().zip(self[ForceIndex::Ang].clone()).map(
                    |(a, ang)| -> unit{
                        self.mag_times_sin(a, ang)
                    }
                )
            }
        ).or_else(
            ||-> Option<unit>{
                self[ForceIndex::Fy].clone().zip(self[ForceIndex::Ay].clone()).map(
                    |(fy,m)| -> unit{
                        self.x_over_y(fy, m)
                    }
                )
            }
        );
        if let Some(a) = res{
            self.set(ForceIndex::Ay, a);
            return Ok(a);
        }

        Err(ForceErr)
    }

    pub fn x_calc_cos(&mut self){
        self[ForceIndex::Fx] = Some(self[ForceIndex::F].unwrap_or(0.0)*(self[ForceIndex::Ang].unwrap_or(0.0)).cos());
    }
    pub fn y_calc_sin(&mut self){
        self[ForceIndex::Fy] = Some(self[ForceIndex::F].unwrap_or(0.0)*(self[ForceIndex::Ang].unwrap_or(0.0)).sin());
    }
    pub fn ax_calc_fx(&mut self){
        self[ForceIndex::Ax] = Some(self[ForceIndex::Fx].unwrap_or(0.0)/(self[ForceIndex::M].unwrap_or(0.0)));
    }
    pub fn a_calc_f(&mut self){
        self[ForceIndex::A] = Some(self[ForceIndex::F].unwrap_or(0.0)/(self[ForceIndex::M].unwrap_or(0.0)));
    }

}

#[derive(Clone)]
pub struct TempForce{
    force: Force,
    frames_left : i32,
    must_be_called_off: bool,
    point : Point
}
impl TempForce{
    pub fn new(force: Force, frames: i32, kill: bool, point : Point) -> Self{
        Self{
            force: force,
            frames_left:frames,
            must_be_called_off: kill,
            point: point
        }
    }

    pub fn tick(&mut self)-> Option<i32>{
        if self.must_be_called_off{
            return None;
        }
        self.frames_left-=1;
        if self.frames_left <=0{
            
            return Some(0);
        }
        return Some(self.frames_left);
    }
    pub fn must_be_called_off(&self) -> bool{
        self.must_be_called_off
    }
    pub fn call_off(&mut self){
        self.frames_left=0;
    }
    pub fn point(&self)-> &Point{
        &self.point
    }
   
    pub fn frames_left(&self)-> Option<i32>{
        if self.must_be_called_off{
            return None;
        }
        
        
        return Some(self.frames_left);
    }
    pub fn force(&self) -> &Force{
        &self.force
    }
    pub fn force_mut(&mut self) -> &mut Force{
        &mut self.force
    }
}
impl Ord for TempForce{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl PartialEq for TempForce{
    fn eq(&self, other: &Self) -> bool {
        self.frames_left == other.frames_left
    }
}
impl Eq for TempForce {
    
}
impl PartialOrd for TempForce {
    fn ge(&self, other: &Self) -> bool {
        self.frames_left>= other.frames_left
    }
    fn gt(&self, other: &Self) -> bool {
        self.frames_left> other.frames_left
    }
    fn le(&self, other: &Self) -> bool {
        self.frames_left<= other.frames_left
    }
    fn lt(&self, other: &Self) -> bool {
        self.frames_left < other.frames_left
    }
    
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        
        match self.frames_left.partial_cmp(&other.frames_left) {
            Some(core::cmp::Ordering::Equal) => {Some(std::cmp::Ordering::Equal)}
            ord => return ord,
        }
        
    }
    
}
impl From<Vect> for Force{
    fn from(value: Vect) -> Self {
        let mut ret : Force =Force::new_force(1.0);
        ret[ForceIndex::F] = Some(value.mag());
        ret[ForceIndex::Ang] = Some(value.angle());
        ret[ForceIndex::Fx] = Some(value.x());
        ret[ForceIndex::Fy] = Some(value.y());
        ret

    }
}

impl Display for Force{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "F: {}\nFx: {}\nFy: {}\nA: {}\nAx: {}\nAy: {}", self[ForceIndex::F].unwrap_or(0.0), self[ForceIndex::Fx].unwrap_or(0.0),self[ForceIndex::Fy].unwrap_or(0.0),self[ForceIndex::A].unwrap_or(0.0),self[ForceIndex::Ax].unwrap_or(0.0),self[ForceIndex::Ay].unwrap_or(0.0),)
    }
}

impl Display for TempForce{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.force)
    }
}