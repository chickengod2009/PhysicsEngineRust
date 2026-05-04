use crate::Physics::{Energy::{kinetic::KE, mechanical::ME}, force::forceing::Force, momentum::{linear::linear_mom::LinearMomentum, rotational::rot_mom::RotationalMomentum}, objects::polygons::{Point, Polygon}, unit};


pub mod polygons;
pub struct Object{

    energy : ME,
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