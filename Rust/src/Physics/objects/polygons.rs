use core::f64;
use std::fmt::{Display, write};

use crate::Physics::unit;
//type unit = f64;
fn main(){
  
  let g: Vec<Point>= vec![Point::new(0_f64,0_f64),Point::new(4_f64,0_f64),Point::new(4_f64,4_f64), Point::new(0_f64,4_f64)];
  let h = Polygon::new(g);
  
  for i in h.angles.iter(){
    println!("{}",i.angle);
  }
  
  
}
pub struct Point{
    x: unit, y:unit
}
pub struct Line{
    a:Point, b:Point,
    slope: unit,
    k: u64,
    is_vert: bool,
    ang_to_vert: unit
}
pub struct Ray{
    a:Point, dir:unit
}
pub struct Angle{
    a:Line,b:Line,
    shared_point: Point,
    angle: unit,
    angle_to_vert: unit
}

pub struct Polygon{
    points: Vec<Point>,
    lines: Vec<Line>,
    angles: Vec<Angle>,
    center: Point,
}

impl PartialEq for Point{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Point{
  pub fn new(d: f64, b: f64) ->Self{
    Self{x:d, y:b}
  }
}
impl Line {
    pub fn new(a: Point, b: Point) -> Self{
        let mut h =Self { a:a, b:b, slope:0 as unit, k: 0, is_vert:false, ang_to_vert: 0 as unit};
        h.calc_slope();
        h
    }


    fn connected (&self, l:&Line)->Option<Point>{

        
        let g: Option<Point> = if self.a==l.a {
            Some(self.a.clone())
        } else if self.a==l.b  {
            Some(self.a.clone())
        } else if self.b==l.a  {
            Some(self.b.clone())
        } else if self.b==l.b{
            Some(self.b.clone())
        }else{
        None
        };
        g

    }

    fn calc_slope(&mut self)->Option<unit>{
        let x: unit = self.b.x-self.a.x;
        let y:unit = self.b.y-self.a.y;
        if y.abs() <= 1e-8 as unit{
            self.is_vert =true;
            return None;
        }
        let slope: unit = x/y;

        self.ang_to_vert = y.atan2(x);
        
        

        self.slope=slope;
        Some(slope)

    }



}

impl Clone for Point{
    fn clone(&self) -> Self {
        Self{x:self.x, y:self.y}
    }
}
impl Clone for Line{
  fn clone(&self)->Self{
    
    Self{a:self.a.clone(), b:self.b.clone(), slope:self.slope, is_vert:self.is_vert,k:self.k, ang_to_vert:self.ang_to_vert}
    
  }
}
impl Angle{

    fn new(a: Line, b: Line)->Option<Self>{

        if let Some(g) = a.connected(&b){
            let mut g: Angle =Self{a:a,b:b, shared_point: g, angle: 0 as unit, angle_to_vert:0 as unit };
            g.calcAngle();
            Some(g)
        }else{
            None
        }


    }

    fn calcAngle(&mut self) -> unit{
        
        self.angle = f64::abs(self.b.ang_to_vert-self.a.ang_to_vert);
        self.angle_to_vert = -(self.b.ang_to_vert+self.a.ang_to_vert);
        self.angle = self.angle*180_f64/f64::consts::PI;
        self.angle_to_vert = self.angle_to_vert*180_f64/f64::consts::PI;
        self.angle


        

    }


}

impl Polygon{
  
  pub fn new(q: Vec<Point>)->Self{
    let qs = q.len();
    let mut lines : Vec<Line> = Vec::with_capacity(qs.clone());
    let mut ang : Vec<Angle> = Vec::with_capacity(qs.clone());
    for i in 0..(qs-1){
      
      lines.push(Line::new(q[i].clone(), q[i+1].clone()));
      
    }
    lines.push(Line::new(q[0].clone(), q[qs-1].clone()));
    for i in 0..(qs-1){
      ang.push(Angle::new(lines[i].clone(), lines[i+1].clone()).unwrap());
    }
    ang.push(Angle::new(lines[0].clone(), lines[qs-1].clone()).unwrap());
    Self{points: q, lines: lines, angles: ang, center: Point{x:0_f64,y:0_f64}}
  }
  
  
  
}
impl Polygon{


    pub fn angles_by_ref(&self) -> &Vec<Angle>{
        &self.angles
    }


}

impl Display for Angle{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, vert :{}", self.angle, self.angle_to_vert)
    }
}