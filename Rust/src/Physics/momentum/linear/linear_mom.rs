use std::fmt::Display;

use crate::Physics::{Vector, force::{forceing::Force}, momentum::linear::var::LinVar, unit, vars::{Var, index_get}};



pub type LinearMomentum = Var<LinVar, 8>;


impl LinearMomentum{

    pub fn create()->Self{
        Self { index: LinVar::M, elements: [None;8], where_i: 0, size: 8 }
    }

    pub fn with_mass(mut self, mass: unit)->Self{
        self.set(LinVar::M, mass).expect("Linear momenta with_mass");
        self
    }

    pub fn with_vx(mut self, velx: unit) -> Self{
        self.set(LinVar::Vx, velx).expect("Linear momenta with_vx");
        self
    }
    pub fn with_vy(mut self, vely: unit) -> Self{
        self.set(LinVar::Vy, vely).expect("Linear momenta with_vy");
        self
    }
    pub fn with_p(mut self, momenta: unit) -> Self{
        self.set(LinVar::P, momenta).expect("Linear momenta with_p");
        self
    }
    pub fn with_px(mut self, momentax: unit) -> Self{
        self.set(LinVar::Px, momentax).expect("Linear momenta with_px");
        self
    }
    pub fn with_v(mut self, vel: unit) -> Self{
        self.set(LinVar::V, vel).expect("Linear momenta with_v");
        self
    }
    pub fn with_py(mut self, momentay: unit) -> Self{
        self.set(LinVar::Py, momentay).expect("Linear momenta with_py");
        self
    }

    pub fn calc_p_over_v(&mut self){
        self[LinVar::P] = Some(self[LinVar::V].unwrap_or(0.0)*self[LinVar::M].unwrap_or(0.0));
    }
    pub fn calc_px_over_vx(&mut self){
        self[LinVar::Px] = Some(self[LinVar::Vx].unwrap_or(0.0)*self[LinVar::M].unwrap_or(0.0));
    }
    pub fn calc_py_over_vy(&mut self){
        self[LinVar::Py] = Some(self[LinVar::Vy].unwrap_or(0.0)*self[LinVar::M].unwrap_or(0.0));
        //println!("{}, {}", self[LinVar::V].unwrap(), self[LinVar::M].unwrap());
    }

    
    pub fn calc_v(&mut self) -> Result<unit, LinErr>{
        let mut o : Option<unit> = None;
        
        o = self[LinVar::Vx].clone().zip(self[LinVar::Vy].clone()).map(
                |(x, y)| self.solve_pyth(x,y)
                ).or_else(|| -> Option<unit>{
                    self[LinVar::Ang].clone().zip(self[LinVar::Vx].clone())
                    	.map(
                        |(ang,x)| -> unit{ self.x_over_cos(x,ang)}
                        	)
                     }       
                ).or_else(
                    || -> Option<unit>{
                    	self[LinVar::Ang].clone().zip(self[LinVar::Vy].clone())
                    	.map(
                        |(ang,y)| -> unit{self.y_over_sin(y,ang)}
                        	)
                      }      
                ).or_else(
                    ||-> Option<unit>{
                    	self[LinVar::M].clone().zip(self[LinVar::P].clone())
                        .map(
                            |(m, p)| ->unit{self.x_over_y(p,m)}
                            )
                      }      
                            
                );
                
            
        if let Some(a) = o{
            self.set(LinVar::V, a);
            return Ok(a);
        }    
                
        
        Err(LinErr{})        
            
            
    }
    pub fn calc_vx(&mut self) -> Result<unit, LinErr>{
        let mut o : Option<unit> = None;
        
        o = self[LinVar::V].clone().zip(self[LinVar::Vy]).map(
            	|(v, y)| -> unit{
             	   self.rev_pyth(v,y)
            	}
            ).or_else(
                || -> Option<unit>{
                    self[LinVar::V].clone().zip(self[LinVar::Ang]).map(
                        |(v, ang)| -> unit{
                            self.mag_times_cos(v, ang)
                        }
                    )
                 }
            ).or_else(
                || -> Option<unit>{
                    self[LinVar::M].clone().zip(self[LinVar::Px]).map(
                        |(m, px)| -> unit{
                            self.x_over_y(px, m)
                        }
                    )
                 }
            );                       
                
            
        if let Some(a) = o{
            self.set(LinVar::Vx, a);
            return Ok(a);
        }    
                
        
        Err(LinErr{})        
            
        
    }
    pub fn calc_vy(&mut self) -> Result<unit, LinErr>{
        let mut o : Option<unit> = None;
        
        o = self[LinVar::V].clone().zip(self[LinVar::Vx]).map(
            	|(v, x)| -> unit{
             	   self.rev_pyth(v,x)
            	}
            ).or_else(
                || -> Option<unit>{
                    self[LinVar::V].clone().zip(self[LinVar::Ang]).map(
                        |(v, ang)| -> unit{
                            self.mag_times_sin(v, ang)
                        }
                    )
                 }
            ).or_else(
                || -> Option<unit>{
                    self[LinVar::M].clone().zip(self[LinVar::Py]).map(
                        |(m, py)| -> unit{
                            self.x_over_y(py, m)
                        }
                    )
                 }
            );                       
                
            
        if let Some(a) = o{
            self.set(LinVar::Vy, a);
            return Ok(a);
        }    
                
        
        Err(LinErr{})        
            
        
    }

    pub fn v_calc_pyth(&mut self){
        let new = self[LinVar::Vx].unwrap_or(0.0).powi(2) + self[LinVar::Vy].unwrap_or(0.0).powi(2);
        let new = new.sqrt();
        self[LinVar::V] =Some(new);
    }

    pub fn apply_impulse_x(&mut self, imp : &Force, time : unit){
        let impulse = imp.x().unwrap()*time;
        let change_v = impulse/self.mass().unwrap();
        let mut v = self.vx().unwrap();
        v -= change_v;
        
        self.set(LinVar::Vx, v);
        //println!("{} {} {} {} {}, {}", change_v,v,self[LinVar::Vx].unwrap(), impulse , time, imp.x().unwrap());
    }   
    pub fn apply_impulse_y(&mut self, imp : &Force, time : unit){
        let impulse = imp.y().unwrap()*time;
        let change_v = impulse/self.mass().unwrap();
        let mut v = self.vy().unwrap();
        v -= change_v;
        self.set(LinVar::Vy, v);
    }   
    
               

    pub fn vx(&self)-> Option<unit>{
        self[LinVar::Vx].clone()
    }
    pub fn vy(&self)-> Option<unit>{
        self[LinVar::Vy].clone()
    }pub fn mass(&self) -> Option<unit>{
        self[LinVar::M].clone()
    }
    pub fn v(&self) -> Option<unit>{
        self[LinVar::V].clone()
    }
    

}

pub struct LinErr;

impl Vector for LinearMomentum{
    type Output =unit;

    type Error = LinErr;

    fn x(& self)-> Option<Self::Output> {
        todo!()
    }

    fn y(& self) -> Option<Self::Output> {
        todo!()
    }

    fn mag(& self) -> Option<Self::Output> {
        todo!()
    }

    fn calc_x(&mut self) -> Result<Self::Output, Self::Error> {
        let mut o : Option<unit> = None;
        
        o = self[LinVar::P].clone().zip(self[LinVar::Py]).map(
            	|(v, y)| -> unit{
             	   self.rev_pyth(v,y)
            	}
            ).or_else(
                || -> Option<unit>{
                    self[LinVar::P].clone().zip(self[LinVar::Ang]).map(
                        |(v, ang)| -> unit{
                            self.mag_times_cos(v, ang)
                        }
                    )
                 }
            ).or_else(
                || -> Option<unit>{
                    self[LinVar::M].clone().zip(self[LinVar::Px]).map(
                        |(m, px)| -> unit{
                            self.x_times_y(px, m)
                        }
                    )
                 }
            );                       
                
            
        if let Some(a) = o{
            self.set(LinVar::Px, a);
            return Ok(a);
        }    
                
        
        Err(LinErr{})        
            
    }

    fn calc_y(&mut self) -> Result<Self::Output, Self::Error> {
        
        
        let o: Option<f64> = self[LinVar::P].clone().zip(self[LinVar::Px]).map(
            	|(v, x)| -> unit{
             	   self.rev_pyth(v,x)
            	}
            ).or_else(
                || -> Option<unit>{
                    self[LinVar::P].clone().zip(self[LinVar::Ang]).map(
                        |(v, ang)| -> unit{
                            self.mag_times_sin(v, ang)
                        }
                    )
                 }
            ).or_else(
                || -> Option<unit>{
                    self[LinVar::M].clone().zip(self[LinVar::Vy]).map(
                        |(m, py)| -> unit{
                            self.x_times_y(py, m)
                        }
                    )
                 }
            );                       
                
            
        if let Some(a) = o{
            self.set(LinVar::Py, a);
            return Ok(a);
        }    
                
        
        Err(LinErr{})        
            
    }

    fn calc_mag(&mut self) -> Result<Self::Output, Self::Error> {
        let mut o : Option<unit> = None;
        
        o = self[LinVar::Px].clone().zip(self[LinVar::Py].clone()).map(
                |(x, y)| self.solve_pyth(x,y)
                ).or_else(|| -> Option<unit>{
                    self[LinVar::Ang].clone().zip(self[LinVar::Px].clone())
                    	.map(
                        |(ang,x)| -> unit{ self.x_over_cos(x,ang)}
                        	)
                     }       
                ).or_else(
                    || -> Option<unit>{
                    	self[LinVar::Ang].clone().zip(self[LinVar::Py].clone())
                    	.map(
                        |(ang,y)| -> unit{self.y_over_sin(y,ang)}
                        	)
                      }      
                ).or_else(
                    ||-> Option<unit>{
                    	self[LinVar::M].clone().zip(self[LinVar::V].clone())
                        .map(
                            |(m, p)| ->unit{self.x_times_y(p,m)}
                            )
                      }      
                            
                );
                
            
        if let Some(a) = o{
            self.set(LinVar::P, a);
            return Ok(a);
        }    
                
        
        Err(LinErr{})        
            
        
    }

    fn get_angle(&mut self) -> Option<Self::Output> {
        todo!()
    }

    fn calc_angle(&mut self) -> Result<Self::Output, Self::Error> {
        let res : Option<unit> = self[LinVar::Px].clone().zip(self[LinVar::Py].clone()).map(
            |(x,y)| -> unit{
                y.atan2(x)
            }
        ).or_else(
            || -> Option<unit>{
                self[LinVar::P].clone().zip(self[LinVar::Px].clone()).map(
                    |(f,fx)| -> unit{
                        (fx/f).acos()
                    }
                )
            }
        ).or_else(
            || -> Option<unit>{
                self[LinVar::P].clone().zip(self[LinVar::Py].clone()).map(
                    |(f,fy)| -> unit{
                        (fy/f).acos()
                    }
                )
            }
        ).or_else(
            || -> Option<unit>{
                self[LinVar::V].clone().zip(self[LinVar::Vx].clone()).map(
                    |(f,fx)| -> unit{
                        (fx/f).acos()
                    }
                )
            }
        ).or_else(
            || -> Option<unit>{
                self[LinVar::V].clone().zip(self[LinVar::Vy].clone()).map(
                    |(f,fy)| -> unit{
                        (fy/f).acos()
                    }
                )
            }
        );

        if let Some(a) = res{
            let ang = (a*(180 as unit))/(std::f64::consts::PI);
            self.set(LinVar::Ang, ang);
            return Ok(ang);
        }

        Err(LinErr)
    
    }
}


impl Display for LinearMomentum{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "P: {}\nPx: {}\nPy: {}\nV: {}\nVx: {}\nVy: {}", self[LinVar::P].unwrap_or(0.0), self[LinVar::Px].unwrap_or(0.0), self[LinVar::Py].unwrap_or(0.0), self[LinVar::V].unwrap_or(0.0), self[LinVar::Vx].unwrap_or(0.0), -self[LinVar::Vy].unwrap_or(0.0),)
    }
}