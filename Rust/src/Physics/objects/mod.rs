use std::{fmt::Display, hash::{self, Hash, Hasher}};

use iced::{Color, Renderer, widget::canvas::Frame};

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
    id: i32,
    stickyness : unit
    
    




}

impl Object{


	pub fn new(body: &Polygon, mas: unit, rig: bool, col: bool, id :i32, stick : unit) -> Self{
        assert!(mas !=0.0);
                
    	let pass = Self{
            
            
            
            kinetic: KE::new(mas),
            all_forces : Vec::new(),
            momentum : LinearMomentum::create().set_all_zero().with_mass(mas),
            momentum_rot : RotationalMomentum::create(mas*5.0),
    		net_force : Force::new_force(mas),
    		com: body.cent().clone(),
    		central_mass: mas,
    		non_moving: rig,
    		collidable: col,
    		body: body.clone(),
            id: id ,
            stickyness : stick
            
            
        };

        pass
  
	}

    pub fn with_starting_v(mut self, v: Vect) -> Self{
        self.momentum = self.momentum.with_vx(v.x()).with_vy(-v.y());
        self
    }

    pub fn mass(&self) -> unit{self.central_mass}

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
        self.com = self.body.find_cent().clone();
        
        
    }
    pub fn apply_w_to_rot2d(& mut self){
        if self.non_moving{
            return;
        }

        


          
        let trans : Rotational2d = Rotational2d::new(-self.momentum_rot.w()*time_frame);
        self.body.rotation(trans);
        
    }    

}   


impl Object{
    pub fn apply_torques(&mut self){
        let mut net : Torque = Torque::new_with_force(Force::new_force(0.0).set_all_zero(), Vect::new(0.0, 0.0), self.momentum_rot.moment_of_inertia());
        for i in self.all_forces.iter(){
            if i.torque().torque().abs() <= 1e-4{
                continue;
            }
            net+= &i.torque;
        }
        

        self.momentum_rot.impulse(&net, time_frame);
        if self.momentum_rot.w() >= 40.0{
            *self.momentum_rot.w_mut() = 40.0;
        }
        if self.momentum_rot.w() <= -40.0{
            *self.momentum_rot.w_mut() = -40.0;
        }
        
    }
    pub fn apply_forces(&mut self){
        let mut net : Force = Force::new_force(self.central_mass).set_all_zero();
        for i in self.all_forces.iter(){
            net+= i.temp.force();
            
        }
        //net.calc_mag();
        
        
        self.momentum.apply_impulse_x(&net, time_frame);
        self.momentum.apply_impulse_y(&net, time_frame);
        self.momentum.v_solve_pyth();
        self.momentum.calc_angle();
        self.kinetic.calc_new_v(self.momentum.v().unwrap();
        
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
    pub fn collide(&mut self, other: &mut Object) {
    let damp_cof = (self.stickyness + other.stickyness)/2.0;
    let k: f64 = 5 as unit;
    if !self.collidable || !other.collidable { return; }

    if let Some(a) = self.body.collision(&mut other.body) {
        let vec = Vect::new(self.momentum.vx().unwrap(), self.momentum.vy().unwrap());
        let vn = vec.dot(&a.normal);
        let force_mag = (k * a.depth - damp_cof * vn).max(0.0);

        let mut force = Force::new_force(self.central_mass);
        force.set(ForceIndex::F, force_mag);
        force.set(ForceIndex::Ang, a.normal.angle());
        force.calc_x_cos();
        force.calc_y_sin();

        
        let r  = Vect::new(a.point.x() - self.com.x(),  a.point.y() - self.com.y());
        let r2 = Vect::new(a.point.x() - other.com.x(), a.point.y() - other.com.y());

        
        let mom_self  = self.momentum_rot.moment_of_inertia();
        let mom_other = other.momentum_rot.moment_of_inertia();

        self.all_forces.push(TempAction::new(
            TempForce::new(force.clone().inverse(), 2, false, a.point.clone()),
            Torque::new_with_force(force.clone().inverse(), r, mom_self),
        ));
        other.all_forces.push(TempAction::new(
            TempForce::new(force.clone(), 2, false, a.point.clone()),
            Torque::new_with_force(force, r2, mom_other),
        ));
        //if other.id() == 60{
        //println!("{} {}", force.x().unwrap(), force.get_angle().unwrap());
        //}
    }
}

    pub fn tick(&mut self){
        
        self.manage();
        self.apply_forces();
        self.apply_torques();
        self.apply_v_to_trans2d();
        self.apply_w_to_rot2d();
        
        
    }

    pub fn add_torque(&mut self, torque:  Torque, frames: i32){
        self.all_forces.push(TempAction::new(TempForce::new(Force::from(Vect::new(0.0, 0.0)), frames, false, self.com()), torque));
    }

    pub fn add_temp_force(&mut self, temp : TempForce){
        let r = Vect::new((self.com.x()-temp.point().x()), (self.com.y()-temp.point().y()));
        let tor : Torque = Torque::new_with_force(temp.force().clone(), r, self.momentum_rot.moment_of_inertia());
        self.all_forces.push(TempAction::new(temp, tor));
        
    }
    pub fn add_temp_force_no_torque(&mut self, temp : TempForce){
        let r = Vect::new((self.com.x()-temp.point().x()), (self.com.y()-temp.point().y()));
        //let tor : Torque = Torque::new_with_force(temp.force().clone(), r, self.momentum_rot.moment_of_inertia());
        self.all_forces.push(TempAction::new(temp, Torque::new_with_force(Force::new_force(0.0).set_all_zero(), Vect::new(0.0, 0.0),9.0)));
        
    }
    pub fn com(&self)-> Point{
        self.com.clone()
    }
    pub fn vel_as_vect(&self)->Vect{
        let ret : Vect = Vect::new(self.momentum.vx().unwrap(), -self.momentum.vy().unwrap());
        ret
    }
    pub fn body(&self) ->&Polygon{
        &self.body
    }
    pub fn draw(&self, frame :  &mut Frame<Renderer>  ){
        self.body.draw(frame, Color::from_rgb8(20, 100, 100));
    }
    pub fn reverse_v(& mut self){
        self.momentum.set(LinVar::Vx, -self.momentum.vx().unwrap());
        self.momentum.set(LinVar::Vy, -self.momentum.vy().unwrap());
        //self.momentum.set(LinVar::Ang, self.momentum.get_angle().unwrap());
    }

    pub fn sendLog(&self) -> ObjectLog{
        ObjectLog::new(self)
    }


    
}


//Logging info
pub struct ObjectLog{
    ke :KE,
    mom : LinearMomentum,
    wmom: RotationalMomentum,
    forces: Vec<TempAction>,
    com: Point
}

impl ObjectLog{
    pub fn new(obj : &Object)-> Self{

        Self { ke: obj.kinetic.clone(), mom: obj.momentum.clone(), wmom: obj.momentum_rot.clone(), forces: obj.all_forces.clone(), com: obj.com.clone() }

    }
}

#[derive(Clone)]
pub struct TempAction{
    temp : TempForce,
    torque : Torque
}
impl Display for TempAction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Force:\n{}\nTorque:\n{}", self.temp, self.torque)
    }
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

/*
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
*/
