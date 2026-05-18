

use crate::Physics::{force::{forceing::{Force, TempForce}, variable::ForceIndex}, objects::{Object, ObjectLog, polygons::{Point, Polygon, Vect}}, unit};
use iced::{self, Color};
pub struct System{

    objs : Vec<Object>,
    m_e : unit,
    potential : unit,
    with_eath : Option<Point>,
    with_gravity : bool,
    with_air_res  : bool,
    mass_sum : unit,
    ke: unit
    



}

impl System{

    pub fn new(obj : Vec<Object>, with_e : Option<Point>, with_grav : bool, with_air : bool) -> Self{
        Self { objs: obj, m_e: 0.0, potential: 0.0, with_eath: with_e, with_gravity: with_grav, with_air_res: with_air, mass_sum: 0.0, ke: 0.0}
    }
    pub fn tick(&mut self){
        self.ke =0.0;
        self.potential =0.0;
        for obj in self.objs.iter_mut() {
            obj.manage();
        }
        for i in 0..self.objs.len(){
            
            for y in (i+1)..self.objs.len(){
                let (left, right) = self.objs.split_at_mut(y);
                let obj_a: &mut Object = &mut left[i];
                let obj_b: &mut Object = &mut right[0];
                obj_a.collide(obj_b);
                obj_a.attractive_forces(obj_b);
            }
            if self.with_air_res{
                let ob : &Object = &self.objs[i];
                let kl = ob.vel_as_vect().normalized();
                //let w = kl.clone().perp();
                let vec = Vect::new(kl.x()*0.3, kl.y()*0.3 );
                let temp : TempForce = TempForce::new(Force::from(vec), 1, false, ob.com());
                //let temp2 = TempForce::new(Force::from(w), 1, false, Point::new(ob.com().x() + 5.9, ob.com().y() +5.0));
                self.objs.get_mut(i).unwrap().add_temp_force(temp);
                
            }
            
        }
        for i in self.objs.iter_mut(){
            i.apply_forces();
            i.apply_torques();
            i.apply_v_to_trans2d();
            i.apply_w_to_rot2d();
            self.ke+=i.kinetic().value();
            if let Some(pot) = self.with_eath.clone() && !i.rigid(){
                let height = pot.y()-i.com().y();
                //println!("{}, {}, {}", height, pot.y(), i.com().y());
                self.potential+= height*0.0981*i.mass();
            }
            
        }
        self.m_e = self.potential+self.ke;
    }
    pub fn start(&mut self){
        if self.with_gravity{
            for i in  self.objs.iter_mut(){
                let vec : Vect = Vect::new(0.0, -9.81*i.mass());
                let mut f =Force::from(vec);
                f.set(ForceIndex::M, i.mass());
                i.add_temp_force_no_torque(TempForce::new(f, 0, true, i.com()));
            }
        }
        for i in self.objs.iter(){
            self.mass_sum+=i.mass();
        }
    }
    pub fn mass_sum(&self) -> unit{
        self.mass_sum
    }
    pub unsafe fn  get_obj_unsafe_ptr(&mut self, i: usize) -> *mut Object{
        let ptr : *mut Object;
        
        ptr = &mut self.objs[i] as *mut Object;
        ptr
        
    }
    pub fn objs(&self) -> &Vec<Object>{
        &self.objs
    }
    pub fn objs_mut(&mut self) -> &mut Vec<Object>{
        &mut self.objs
    }

    pub fn request_object_logs(&mut self) -> Vec<ObjectLog>{
        let mut ret : Vec<ObjectLog> = Vec::new();
        for i in self.objs.iter_mut(){
            if i.logged(){
                ret.push(i.sendLog());
            }
        }
        
        ret
    }

    pub fn check_for_col_suggestion(&mut self) -> Vec<ObjectLog>{
        let mut ret: Vec<ObjectLog> = Vec::new();
        for i in self.objs.iter_mut(){
            if let Some(a) = i.should_I_suggest_log_col(){
                ret.push(a);
            }
        }
        ret
    }

    pub fn me(&self) -> &unit{
        &self.m_e
    }
    pub fn ke(&self) -> &unit{
        &self.ke
    }
    pub fn pot(&self) -> &unit{
        &self.potential
    }
    pub fn has_earth(&self) -> bool{
        self.with_eath.is_some()
    }


}

impl Default for System{
    fn default() -> Self {
        System::new(vec![Object::new(&mut Polygon::default(), 1.0, false, true, 0, 3.0, true, Color::from_rgb8(200, 0, 200), String::from("l")),Object::new(&mut Polygon::new(vec![Point::new(100.0,100.0),Point::new(500.0,200.0),Point::new(200.0,100.1),]),
         1.0, false, true, 0, 
         3.0, true, Color::from_rgb8(200, 0, 200), String::from("h")) ] , None, false, false)
    }
}
