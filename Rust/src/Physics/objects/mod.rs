use std::hash::{self, Hash, Hasher};

use crate::Physics::{Energy::{kinetic::KE, mechanical::ME}, Vector, force::{forceing::{Force, TempForce}, torque::{self, Torque}, variable::ForceIndex }, momentum::{linear::{linear_mom::LinearMomentum, var::LinVar}, rotational::{rot_mom::RotationalMomentum, var::RotVar}}, objects::polygons::{Point, Polygon, Vect}, time_frame, unit};
use self::polygons::{Translation2d, RotSinCos, Rotational2d};

pub mod polygons;
#[derive(Clone)]
pub struct Object{
    
    kinetic : KE,
    all_forces: Vec<TempAction>,
    momentum: LinearMomentum,
    momentum_rot: RotationalMomentum,
    net_force : Force,
    com: Point,
    central_mass: unit,
    non_moving:bool,
    collidable:bool,
    body: Polygon,
    id: i32
    
    




}

impl Object{


	pub fn new(body: &Polygon, mas: unit, rig: bool, col: bool, id :i32) -> Self{
        
                
    	let pass = Self{
            
            
            
            kinetic: KE::new(mas),
            all_forces : Vec::new(),
            momentum : LinearMomentum::create().set_all_to_zero().with_mass(mas),
            momentum_rot : RotationalMomentum::create(mas),
    		net_force : Force::new_force(mas),
    		com: body.find_cent(),
    		central_mass: mas,
    		non_moving: rig,
    		collidable: col,
    		body: body.clone(),
            id: id 
            
            
        };

        pass
  
	}

    pub fn id(&self)-> i32{
        self.id
    }
    
    
    
    pub fn apply_v_to_trans2d(& mut self){
        if self.non_moving{
            return;
        }

        let (Some(vx), Some(vy)) =  (self.momentum.vx(), self.momentum.vy()) else {
            panic!()
        };   
        let trans : Translation2d = Translation2d::new(vx*time_frame, vy*time_frame);
        self.body.translation(trans);
        self.com = self.body.find_cent_mut();
        
        
    }
    pub fn apply_w_to_rot2d(& mut self){
        if self.non_moving{
            return;
        }

          
        let trans : Rotational2d = Rotational2d::new(self.momentum_rot.w()*time_frame);
        self.body.rotation(trans);
        
    }    

}   


impl Object{
    pub fn apply_torques(&mut self){
        let mut net : Torque = Torque::new_with_force(Force::new_force(0.0), Vect::new(0.0, 0.0), self.moment_inertia);
        for i in self.all_forces.iter(){
            if net.torque() <= 1e-4{
                continue;
            }
            net+= &i.torque;
        }

        self.momentum_rot.impulse(&net, time_frame);
        
    }
    pub fn apply_forces(&mut self){
        let mut net : Force = Force::new_force(self.central_mass);
        for i in self.all_forces.iter(){
            net+= i.temp.force();
        }
        self.momentum.apply_impulse_x(&net, time_frame);
        self.momentum.apply_impulse_y(&net, time_frame);
        self.momentum.calc_v();
        self.momentum.calc_angle();
        self.kinetic.calc_new_v(self.momentum.v().unwrap());
        
    }
    pub fn manage(&mut self){
        let mut s: usize =0;
        let mut amount: Vec<usize> = Vec::with_capacity(self.all_forces.len());
        for i in self.all_forces.iter_mut(){
            
            if let Some(a) = i.temp.tick(){
                if a <=0{
                    amount.push(s);
                }
            }
            s+=1;
        }
        for i in amount.into_iter().rev(){
            self.all_forces.remove(i);
        }
        
        
    }
    pub fn collide(&mut self, other : &mut Object){
		
        let damp_cof = 0.80 as unit;
        let k: f64 = 0.005 as unit;
		if !self.collidable || !other.collidable{
			return;
		}	
        
        if let Some(a) = self.body.collision(&mut other.body){
            let vec : Vect  = Vect::new(self.momentum.vx().unwrap(), self.momentum.vy().unwrap());
            let vn : unit = vec.dot(&a.normal);
            let force_mag = (k * a.depth* 0.2 - damp_cof*vn).max(0.0);
            let mut force : Force = Force::new_force(self.central_mass);
            
            force.set(ForceIndex::F, force_mag);
            force.set(ForceIndex::Ang, a.normal.angle());
            force.calc_x();
            force.calc_y();
            let r = Vect::new((self.com.x() -a.point.x()), (self.com.y()-a.point.y()));
            self.all_forces.push(TempAction::new(TempForce::new(force.clone(), 1, false, a.point.clone()), Torque::new_with_force(force.clone(), r, self.moment_inertia)));
                            
        }
    }

    pub fn tick(&mut self){
        
        self.manage();
        self.apply_forces();
        self.apply_torques();
        self.apply_v_to_trans2d();
        self.apply_w_to_rot2d();
        
        
    }

    pub fn add_temp_force(&mut self, temp : TempForce){
        let r = Vect::new((self.com.x()-temp.point().x()), (self.com.y()-temp.point().y()));
        let tor : Torque = Torque::new_with_force(temp.force().clone(), r, self.moment_inertia);
        self.all_forces.push(TempAction::new(temp, tor));
        self.all_forces.sort();
    }
    pub fn com(&self)-> Point{
        self.com.clone()
    }
    pub fn vel_as_vect(&self)->Vect{
        let ret : Vect = Vect::new(self.momentum.vx().unwrap(), self.momentum.vy().unwrap());
        ret
    }
    pub fn body(&self) ->&Polygon{
        &self.body
    }
    
}

//Logging info
struct ObjectLog{
    ke :KE,
    v : unit,
    w: unit,
    forces : Vec<TempForce>,
    torques : Vec<Torque>,
    com: Point
}
#[derive(Clone)]
pub struct TempAction{
    temp : TempForce,
    torque : Torque
}

impl TempAction {
   pub fn new (force: TempForce, torque: Torque) -> Self{
        Self { temp: force, torque : torque }
   }
   pub fn temp(&self)-> &TempForce{
        &self.temp
   }
   pub fn torque(&self)-> &Torque{
        &self.torque
   }
}


impl Ord for TempAction{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl PartialEq for TempAction{
    fn eq(&self, other: &Self) -> bool {
        self.temp().frames_left() == other.temp().frames_left()
    }
}
impl Eq for TempAction {
    
}
impl PartialOrd for TempAction {
    fn ge(&self, other: &Self) -> bool {
        self.temp().frames_left()>= other.temp().frames_left()
    }
    fn gt(&self, other: &Self) -> bool {
        self.temp().frames_left()> other.temp().frames_left()
    }
    fn le(&self, other: &Self) -> bool {
        self.temp().frames_left()<= other.temp().frames_left()
    }
    fn lt(&self, other: &Self) -> bool {
        self.temp().frames_left() <other.temp().frames_left()
    }
    
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        
        match self.temp().frames_left().partial_cmp(&other.temp().frames_left()) {
            Some(core::cmp::Ordering::Equal) => {Some(std::cmp::Ordering::Equal)}
            ord => return ord,
        }
        
    }
    
}
