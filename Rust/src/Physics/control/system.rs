use jni::objects;

use crate::Physics::{force::forceing::{Force, TempForce}, objects::{Object, polygons::Vect}, unit};

pub struct System{

    objs : Vec<Object>,
    m_e : unit,
    potential : unit,
    with_eath : bool,
    with_gravity : bool,
    with_air_res  : bool



}

impl System{
    pub fn tick(&mut self){
        for obj in self.objs.iter_mut() {
            obj.manage();
        }
        for i in 0..self.objs.len(){
            
            for y in (i+1)..self.objs.len(){
                let (left, right) = self.objs.split_at_mut(y);
                let obj_a: &mut Object = &mut left[i];
                let obj_b: &mut Object = &mut right[0];
                obj_a.collide(obj_b);
            }
            if self.with_air_res{
                let ob : &Object = &self.objs[i];
                let temp : TempForce = TempForce::new(Force::from(ob.vel_as_vect()).inverse(), 1, false, ob.com());
                self.objs[i].add_temp_force(temp);
            }
            
        }
        for i in self.objs.iter_mut(){
            i.apply_forces();
            i.apply_torques();
            i.apply_v_to_trans2d();
            i.apply_w_to_rot2d();
        }
    }
    pub fn start(&mut self){
        if self.with_gravity{
            for i in  self.objs.iter_mut(){
                let vec : Vect = Vect::new(0.0, -9.81);
                i.add_temp_force(TempForce::new(Force::from(vec), 0, true, i.com()));
            }
        }
    }
    pub unsafe fn  get_obj_unsafe_ptr(&mut self, i: usize) -> *mut Object{
        let ptr : *mut Object;
        
        ptr = &mut self.objs[i] as *mut Object;
        ptr
        
    }
}