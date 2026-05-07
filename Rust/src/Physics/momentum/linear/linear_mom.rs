use crate::Physics::{Vector, momentum::linear::var::LinVar, unit, vars::Var};



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

    
    pub fn calc_v(&mut self) -> Result<unit, LinErr>{
        let mut o : Option<unit> = None;
        if self.can_I_solve(&[LinVar::Vx,LinVar::Vy]){
            let x = self[LineVar::Vx].unwrap();
            let y = self[LineVar::Vy].unwrap();
            
            o =Some(x*x+y*y).sqrt());
            
            
        } else if self.can_I_solve(&[LinVar::Vx, LinVar::Ang]){
        	let x = self[LineVar::Vx].unwrap();
            let ang = self[LineVar::Ang].unwrap()*(3.14 as unit)/180 as unit;
            o = Some((x)/(ang.cos());
        } else if self.can_I_solve(&[LinVar::Vy, LinVar::Ang]){
        	let x = self[LineVar::Vy].unwrap();
            let ang = self[LineVar::Ang].unwrap()*(3.14 as unit)/180 as unit;
            o = Some((x)/(ang.sin());
        } else if self.can_I_solve(&[LinVar::M, LinVar::P]){
        	let p = self[LineVar::P].unwrap();
            let m = self[LineVar::M].unwrap();
            o = Some(p/m);
        }    
        if let Some(a) = o{
            self.set(LinVar::V, a);
            return Some(a);
        }    
                
        
        Err(LinErr{})        
            
            
    }
    pub fn calc_vx(&mut self) -> Result<unit, LinErr>{
        todo!()
    }
    pub fn calc_vy(&mut self) -> Result<unit, LinErr>{
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