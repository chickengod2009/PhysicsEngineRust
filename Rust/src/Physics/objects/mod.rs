use crate::Physics::{Energy::{kinetic::KE, mechanical::ME}, force::forceing::Force, momentum::{linear::linear_mom::LinearMomentum, rotational::rot_mom::RotationalMomentum}, objects::polygons::{Point, Polygon}, unit};


pub mod polygons;
pub struct Object{
    
    kinetic : KE,
    all_forces: Vec<Force>,
    momentum: LinearMomentum,
    momentum_rot: RotationalMomentum,
    net_force : Force,
    com: Point,
    moment_inertia: Option<unit>,
    easy_access_vel : Option<unit>,
    easy_access_dir: Option<unit>,
    central_mass: unit,
    rigid:bool,
    collidable:bool,
    body: Polygon




}

impl Object{


	fn new(body: Polygon, mas: unit, rig: bool, col: bool) -> Self{
    	let pass = Self{
            
            kinetic: KE::new(mas),
            all_forces : Vec::new(),
            momentum : LinearMomentum::create().with_mass(mas),
            momentum_rot : RotationalMomentum::create().with_mass(mas),
            easy_access_dir : None,
            easy_access_vel : None,
    		net_force : Force::new_force(mas),
    		com: Point::new(0_f64, 0_f64),
    		moment_inertia: None,

    		central_mass: mas,
    		rigid: rig,
    		collidable: col,
    		body: body
            
    };

    pass
  
    }

}   
