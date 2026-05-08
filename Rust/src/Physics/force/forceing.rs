.use core::f64;
use std::{fmt::Debug, ops::Add};

use super::super::vars::Var;

use crate::Physics::{Vector, force::variable::ForceIndex, unit, vars::index_get};

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

    
}

impl Add for Force{
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            index: ForceIndex::A,
            elements: {
                let mut lg: [Option<unit>; 8] = [None; 8];
                for i in 0..8{
                    if let Some(a) =rhs.elements[i] && let Some(b) = self.elements[i] {
                        lg[i] = Some(a+b);
                    } else if let Some(a) = rhs.elements[i]{
                        lg[i] = Some(a);
                    } else if let Some(a) = self.elements[i]{
                        lg[i] = Some(a);
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
        self.elements[ForceIndex::Fx.as_usize()]
    }

    fn get_y(&mut self) -> Option<Self::Output> {
        self.elements[ForceIndex::Fy.as_usize()]
    }

    fn get_mag(&mut self) -> Option<Self::Output> {
        self.elements[ForceIndex::F.as_usize()]
    }

    fn calc_x(&mut self) -> Result<Self::Output, Self::Error> {
        let res: Option<unit> = self[ForceIndex::F].clone().zip(self[ForceIndex::Fy].clone()).map(
            |(f,fy)| -> unit{
                self.rev_pyth(f, fy)
            }
        ).or_else(
            || -> Option<unit>{
                self[ForceIndex::F].clone().zip(self[ForceIndex::Ang].clone()).map(
                    |(f, ang)| -> unit{
                        self.mag_times_cos(f, ang)
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
        let res: Option<unit> = self[ForceIndex::F].clone().zip(self[ForceIndex::Fx].clone()).map(
            |(a,ay)| -> unit{
                self.rev_pyth(a, ay)
            }
        ).or_else(
            || -> Option<unit>{
                self[ForceIndex::F].clone().zip(self[ForceIndex::Ang].clone()).map(
                    |(a, ang)| -> unit{
                        self.mag_times_sin(a, ang)
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
            self.set(ForceIndex::A, a);
            return Ok(a);
        }



        Err(ForceErr{})
    }

    fn get_angle(&mut self) -> Option<Self::Output> {
        todo!()
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
            let ang = (a*(180 as unit))/(f64::consts::PI);
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

}