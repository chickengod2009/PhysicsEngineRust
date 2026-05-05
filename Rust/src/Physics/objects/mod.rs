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

/*
	fn new(body: Polygon, mas: unit, rig: bool, col: bool) -> Self{
    	let pass = Self{
            
            kinetic: KE{},
            all_forces : Vec::new()::<Force>,
            momentum : LinearMomentum: 0,
            momentum_rot : RotationalMomentum 0,
            easy_access_dir : None,
            easy_acces_dir : None,
    		net_force : Force::new(blank),
    		com: Point,
    		moment_inertia: Area^3/12*mas,

    		central_mass: mas,
    		rigid: rig,
    		collidable: col,
    		body: body
            
    }
*/    
