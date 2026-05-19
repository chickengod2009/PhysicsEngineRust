use std::{fmt::{Display, write}, hash::{self, Hash, Hasher}};

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
    stickyness : unit,
    name : String,
    color :  Color,
    logged: bool,
    suggest_log_from_collision: i8,
    am_i_attractive: bool
    




}

impl Object{


	pub fn new(body: &mut Polygon, mas: unit, rig: bool, col: bool, id :i32, stick : unit, logged: bool, color : Color, name : String) -> Self{
        assert!(mas !=0.0);
                
    	let pass = Self{
            
            
            
            kinetic: KE::new(mas),
            all_forces : Vec::new(),
            momentum : LinearMomentum::create().set_all_zero().with_mass(mas),
            momentum_rot : RotationalMomentum::create(mas*10.0),
    		net_force : Force::new_force(mas),
    		com: body.find_cent().clone(),
    		central_mass: mas,
    		non_moving: rig,
    		collidable: col,
    		body: body.clone(),
            id: id ,
            stickyness : stick,
            logged: logged,
            color : color,
            name :name,
            suggest_log_from_collision : 0i8,
            am_i_attractive: false

            
            
        };

        pass
  
	}

    pub fn yes_i_am_attactive(mut self) -> Self{
        self.am_i_attractive = true;
        self
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
        if self.non_moving{return;}
        let mut net : Torque = Torque::new_with_force(Force::new_force(0.0).set_all_zero(), Vect::new(0.0, 0.0), self.momentum_rot.moment_of_inertia());
        for i in self.all_forces.iter(){
            
            net+= &i.torque;
        }
        

        self.momentum_rot.impulse(&net, time_frame);
        
        
    }
    pub fn apply_forces(&mut self){
        if self.non_moving{self.kinetic.calc_new_v(0.0);return;}
        let mut net : Force = Force::new_force(self.central_mass).set_all_zero();
        for i in self.all_forces.iter(){
            net+= i.temp.force();
        
        }
        //net.calc_mag();
        
        
        self.momentum.apply_impulse_x(&net, time_frame);
        self.momentum.apply_impulse_y(&net, time_frame);
        self.body.find_cent();
        self.momentum.v_calc_pyth();
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
            i.temp.force_mut().ay_from_fy();
            i.temp.force_mut().a_calc_f();
            i.temp.force_mut().ax_calc_fx();

            
            s+=1;
        }
        for i in amount.into_iter().rev(){
            self.all_forces.remove(i);
        }
        //1000000000000000000
        
    }
    pub fn with_starting_w(mut self, value: unit) -> Self{
        *self.momentum_rot.w_mut() = value;
        self
    }
    pub fn rot_mom(&self) -> &RotationalMomentum{
        &self.momentum_rot
    }
    pub fn momentum(&self) -> &LinearMomentum{
        &self.momentum
    }
    pub fn body_mut(&mut self) -> &mut Polygon{
        &mut self.body
    } 
    pub fn attractive_forces(&mut self, other: &mut Self){
        if !self.am_i_attractive || !other.am_i_attractive{
            return;
        }
        let r = Vect::new(other.com().x()-self.com.x(), other.com.y()-self.com.y());
        if r.mag()<=1.0{
            return;
        }
        let mag : unit = (6.67e-13*self.central_mass)*(other.central_mass/r.mag().powi(2));
        let normal = r.normalized();
        let x = mag*normal.x();
        let y = mag*normal.y();
        let mut f_self = Force::new_force(self.central_mass);
        f_self.set(ForceIndex::Fx, -x);
        f_self.set(ForceIndex::Fy, -y);
        f_self.calc_mag();
        f_self.calc_angle();
        f_self.a_calc_f();
        f_self.ay_from_fy();
        f_self.ax_calc_fx();


        let mut f_other = Force::new_force(other.central_mass);
        f_other.set(ForceIndex::Fx,x);
        f_other.set(ForceIndex::Fy, y);
        
        f_other.calc_mag();
        f_other.calc_angle();
        f_other.a_calc_f();
        f_other.ay_from_fy();
        f_other.ax_calc_fx();

        

        let self_temp  = TempForce::new(f_self,  1, false, self.body.find_cent().clone());
        let other_temp = TempForce::new(f_other, 1, false, other.body.find_cent().clone());
        //println!("{}", self_temp.force());
        self.add_temp_force_no_torque(self_temp);
        other.add_temp_force_no_torque(other_temp);
        
       

    }
    pub fn collide(&mut self, other: &mut Object) {
        let red_mass = (self.central_mass * other.central_mass) / (self.central_mass + other.central_mass);
    let damp_cof = (self.stickyness + other.stickyness) * red_mass;
    let k: f64 = 12.0 *red_mass as unit;
    if !self.collidable || !other.collidable { return; }
        //Treets all the objects like a spring with cutsom coeffs and surface knowledge for torque
    if let Some(a) = self.body.collision(&mut other.body) {
        let vec = Vect::new(self.momentum.vx().unwrap(), self.momentum.vy().unwrap());
        let vn = vec.dot(&a.normal);
        let force_mag = (k * a.depth - damp_cof * vn).max(0.0);

        let mut force = Force::new_force(self.central_mass);
        force.set(ForceIndex::F, force_mag);
        force.set(ForceIndex::Ang, a.normal.angle());
        force.x_calc_cos();
        force.y_calc_sin();
        force.a_calc_f();
        force.ay_from_fy();
        force.ax_calc_fx();
        
        

        
        let r  = Vect::new(a.point.x() - self.com.x(),  a.point.y() - self.com.y());
        let r2 = Vect::new(a.point.x() - other.com.x(), a.point.y() - other.com.y());

        
        //let mom_self  = self.momentum_rot.moment_of_inertia();
        //let mom_other = other.momentum_rot.moment_of_inertia();
        //println!("{}", mom_self);

        self.all_forces.push(TempAction::new(
            TempForce::new(force.clone().inverse(), 2, false, a.point.clone()),
            Torque::new_with_force(force.clone(), r, 1.0),
        ));
        //What's inversed is kind of messed up because of window's flipped Cordinates
        other.all_forces.push(TempAction::new(
            TempForce::new(force.clone(), 1, false, a.point.clone()),
            Torque::new_with_force(force, r2, 1.0),
        ));
        self.suggest_log_from_collision+=1;
        other.suggest_log_from_collision+=1;
        //if other.id() == 60{
        //println!("{} {}", force.x().unwrap(), force.get_angle().unwrap());
        //}
    }
}
    //for just one object alone
    pub fn tick(&mut self){
        
        self.manage();
        self.apply_forces();
        self.apply_torques();
        self.apply_v_to_trans2d();
        self.apply_w_to_rot2d();
        
        
    }
    //To make collisions recorded
    pub fn should_I_suggest_log_col(&mut self) -> Option<ObjectLog>{
        if self.suggest_log_from_collision >= 5 && self.logged{
            self.suggest_log_from_collision =0;
            //println!("{}", self.name);
            
            Some(self.sendLog())
        } else{ 
            if !self.logged{
                self.suggest_log_from_collision= 0;
            }
            None
            
        }
        
    }
    //never eded up using it
    pub fn add_torque(&mut self, torque:  Torque, frames: i32){
        self.all_forces.push(TempAction::new(TempForce::new(Force::from(Vect::new(0.0, 0.0)), frames, false, self.com()), torque));
    }

    pub fn add_temp_force(&mut self, temp : TempForce){
        let r = Vect::new((self.com.x()-temp.point().x()), (self.com.y()-temp.point().y()));
        let mut tor : Torque = Torque::new_with_force(temp.force().clone(), r, self.momentum_rot.moment_of_inertia());
        tor.calc_alpha();
        self.all_forces.push(TempAction::new(temp, tor));
        
    }

    //For stuff like friction and air resistance
    pub fn add_temp_force_no_torque(&mut self, temp : TempForce){
        let r = Vect::new((self.com.x()-temp.point().x()), (self.com.y()-temp.point().y()));
        //let tor : Torque = Torque::new_with_force(temp.force().clone(), r, self.momentum_rot.moment_of_inertia());
        self.all_forces.push(TempAction::new(temp, Torque::new_with_force(Force::new_force(0.0).set_all_zero(), Vect::new(0.0, 0.0),self.momentum_rot.moment_of_inertia())));
        
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
    //calls polygon
    pub fn draw(&self, frame :  &mut Frame<Renderer>  ){
        self.body.draw(frame, self.color);
    }
    //For perfect elastic. Not really acheivable with my method of collision
    pub fn reverse_v(& mut self){
        self.momentum.set(LinVar::Vx, -self.momentum.vx().unwrap());
        self.momentum.set(LinVar::Vy, -self.momentum.vy().unwrap());
        //self.momentum.set(LinVar::Ang, self.momentum.get_angle().unwrap());
    }

    pub fn kinetic(&self) -> &KE{
        &self.kinetic
    }

    //Sends A log of the object's data
    pub fn sendLog(&mut self) -> ObjectLog{
        self.momentum.calc_p_over_v();
        self.momentum.calc_px_over_vx();
        self.momentum.calc_py_over_vy();
        ObjectLog::new(self)
    }

    pub fn logged(&self) -> bool{
        self.logged
    }
    pub fn rigid(&self) -> bool{
        self.non_moving
    }


    
}


//Logging info
pub struct ObjectLog{
    ke :KE,
    mom : LinearMomentum,
    wmom: RotationalMomentum,
    forces: Vec<TempAction>,
    com: Point,
    name : String
}

impl ObjectLog{
    pub fn new(obj : &Object)-> Self{
        //Almost all the data
        Self { ke: obj.kinetic.clone(), mom: obj.momentum.clone(), wmom: obj.momentum_rot.clone(), forces: obj.all_forces.clone(), com: obj.com.clone(), name : obj.name.clone() }

    }
}
//This structure is so that I can control force lifetime, yet i usually only used 1 or forever
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

impl Display for ObjectLog{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.name).expect("msg");
        for i in self.forces.iter(){
            
            write!(f, "{}\n", i).expect("Write failed in Objet Log");
        }
        write!(f, "\n{}\n{}\n{}\n{}\n",self.mom, self.wmom, self.ke, self.com)
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
