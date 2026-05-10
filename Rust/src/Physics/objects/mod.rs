use std::hash::{self, Hash, Hasher};

use crate::Physics::{Energy::{kinetic::KE, mechanical::ME}, Vector, force::{forceing::{Force, TempForce}, torque::Torque, variable::ForceIndex }, momentum::{linear::{linear_mom::LinearMomentum, var::LinVar}, rotational::{rot_mom::RotationalMomentum, var::RotVar}}, objects::polygons::{Point, Polygon, Vect}, time_frame, unit};
use self::polygons::{Translation2d, RotSinCos, Rotational2d};

pub mod polygons;
#[derive(Clone)]
pub struct Object{
    
    kinetic : KE,
    all_forces: Vec<(TempForce, Torque)>,
    momentum: LinearMomentum,
    momentum_rot: RotationalMomentum,
    net_force : Force,
    com: Point,
    moment_inertia: unit,
    easy_access_vel : Option<unit>,
    easy_access_dir: Option<unit>,
    central_mass: unit,
    non_moving:bool,
    collidable:bool,
    body: Polygon,
    
    




}

impl Object{


	pub fn new(body: &Polygon, mas: unit, rig: bool, col: bool) -> Self{
        
                
    	let pass = Self{
            
            
            
            kinetic: KE::new(mas),
            all_forces : Vec::new(),
            momentum : LinearMomentum::create().with_mass(mas),
            momentum_rot : RotationalMomentum::create(mas),
            easy_access_dir : None,
            easy_access_vel : None,
    		net_force : Force::new_force(mas),
    		com: body.find_cent(),
    		moment_inertia: 0.0,

    		central_mass: mas,
    		non_moving: rig,
    		collidable: col,
    		body: body.clone(),
            
            
        };

        pass
  
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
            net+= &i.1;
        }

        self.momentum_rot.impulse(&net, time_frame);
        
    }
    pub fn apply_forces(&mut self){
        let mut net : Force = Force::new_force(self.central_mass);
        for i in self.all_forces.iter(){
            net+= i.0.force();
        }
        self.momentum.apply_impulse_x(&net, time_frame);
        self.momentum.apply_impulse_y(&net, time_frame);
        self.momentum.calc_v();
        self.momentum.calc_angle();
        self.kinetic.calc_new_v(self.momentum.v().unwrap());
        
    }
    pub fn manage(&mut self, others : &mut [Object]){
        let mut s: usize =0;
        let mut amount: Vec<usize> = Vec::with_capacity(self.all_forces.len());
        for i in self.all_forces.iter_mut(){
            
            if let Some(a) = i.0.tick(){
                if a <=0{
                    amount.push(s);
                }
            }
            s+=1;
        }
        for i in amount.into_iter().rev(){
            self.all_forces.remove(i);
        }
        let k: f64 = 0.005 as unit;
        let damp_cof = 0.80 as unit;
        for i in others.iter_mut(){
            if let Some(a) = self.body.collision(&mut i.body){
                let vec : Vect  = Vect::new(self.momentum.vx().unwrap(), self.momentum.vy().unwrap());
                let vn : unit = vec.dot(&a.normal);
                let force_mag = (k * a.depth* 0.2 - damp_cof*vn).max(0.0);
                let mut force : Force = Force::new_force(self.central_mass);
                
                force.set(ForceIndex::F, force_mag);
                force.set(ForceIndex::Ang, a.normal.angle());
                force.calc_x();
                force.calc_y();
                let r = Vect::new((self.com.x() -a.point.x()), (self.com.y()-a.point.y()));
                self.all_forces.push((TempForce::new(force.clone(), 1, false, a.point.clone()), Torque::new_with_force(force.clone(), r, self.moment_inertia)));
                                
            }
        }
    }

    pub fn tick(&mut self,  others: &mut[Object]){
        
        self.manage(others);
        self.apply_forces();
        self.apply_torques();
        self.apply_v_to_trans2d();
        self.apply_w_to_rot2d();
        
        
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
