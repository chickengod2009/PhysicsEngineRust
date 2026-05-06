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
    k: unit,
    is_vert: bool,
    angle_to_horz: unit
}
pub struct Ray{
    a:Point, dir:unit
}
pub struct Angle{
    a:Line,b:Line,
    shared_point: Point,
    angle: unit,
    angle_to_horz: unit
}

pub struct Polygon{
    points: Vec<Point>,
    lines: Vec<Line>,
    angles: Vec<Angle>,
    center: Point,
    area: Option<unit>
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
        let mut h =Self { a:a, b:b, slope:0 as unit, k: 0 as unit, is_vert:false, angle_to_horz: 0 as unit};
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
        if x.abs() <= 1e-8 as unit{
            self.is_vert =true;
            self.slope =0 as unit;
            self.k =self.b.x;
            return None;
        }
        let slope: unit = y/x;
        self.k = y-x*slope;

        self.angle_to_horz = y.atan2(x);
        
        

        self.slope=slope;
        Some(slope)

    }


    fn pass_through(&self, ot: &Self ) -> Option<Point>{

        if self.is_vert && ot.is_vert {
            if self.k != ot.k{
                return None;
            }
            else{
                if self.a.y != ot.a.y{
                    return Some(self.a.clone());
                }else if self.a.y != ot.b.y{
                    return Some(self.a.clone());
                }else if self.b.y != ot.b.y{
                    return Some(self.b.clone());
                }else if self.b.y != ot.a.y{
                    return Some(self.b.clone());
                }else {return None;}
            }
            
            
        }
        if (self.slope -ot.slope).abs() <= 1e-8_f64{
                if self.a.x != ot.a.x{
                    return Some(self.a.clone());
                }else if self.a.x != ot.b.x{
                    return Some(self.a.clone());
                }else if self.b.x != ot.b.x{
                    return Some(self.b.clone());
                }else if self.b.x != ot.a.x{
                    return Some(self.b.clone());
                }else {return None;}
            }
        let go: f64 = self.slope-ot.slope;
        let gr: f64 = ot.k-self.k;
        let go: f64 = gr/go;  
        if go>= self.a.x.min(self.b.x) && go<= self.b.x.max(self.a.x) && go>= ot.a.x.min(ot.b.x) && go<= ot.a.x.max(ot.b.x){
            return Some(Point::new(go, go*self.slope+self.k));
        }
        None    


    }



}

impl Clone for Point{
    fn clone(&self) -> Self {
        Self{x:self.x, y:self.y}
    }
}
impl Clone for Line{
  fn clone(&self)->Self{
    
    Self{a:self.a.clone(), b:self.b.clone(), slope:self.slope, is_vert:self.is_vert,k:self.k, angle_to_horz:self.angle_to_horz}
    
  }
}
impl Angle{

    fn new(a: Line, b: Line)->Option<Self>{

        if let Some(g) = a.connected(&b){
            let mut g: Angle =Self{a:a,b:b, shared_point: g, angle: 0 as unit, angle_to_horz:0 as unit };
            g.calcAngle();
            Some(g)
        }else{
            None
        }


    }

    fn calcAngle(&mut self) -> unit{
        
        let mut a = self.a.angle_to_horz*180 as unit/f64::consts::PI;
        let mut b = self.b.angle_to_horz*180 as unit/f64::consts::PI;

        let diff = (b - a).abs();

         


        


        self.angle_to_horz = Self::signed_angle_deg(a, b);


        self.angle = diff.min(360.0 - diff);

        self.angle


        

    }
    fn signed_angle_deg(a: f64, b: f64) -> f64 {
        let a_rad = a.to_radians();
        let b_rad = b.to_radians();

    // Unit vectors
        let ax = a_rad.cos();
        let ay = a_rad.sin();

        let bx = b_rad.cos();
        let by = b_rad.sin();

    // Dot and cross
        let dot = ax * bx + ay * by;
        let cross = ax * by - ay * bx;

    // Signed angle in degrees
        cross.atan2(dot).to_degrees()
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
    lines.push(Line::new(q[qs-1].clone(), q[0].clone()));
    for i in 0..(qs-1){
      ang.push(Angle::new(lines[i].clone(), lines[i+1].clone()).unwrap());
    }
    ang.push(Angle::new(lines[qs-1].clone(), lines[0].clone()).unwrap());
    let ret =Self{points: q, lines: lines, angles: ang, center: Point{x:0_f64,y:0_f64}, area: None};
    //ret.area();
    ret
  }
  
  
  
}
impl Polygon{


    pub fn angles_by_ref(&self) -> &Vec<Angle>{
        &self.angles
    }


}

impl Display for Angle{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, vert :{}", self.angle, self.angle_to_horz)
    }
}


impl Polygon{

    fn collision(&self, ot: &Self) -> Option<Point>{


        for i in self.points.iter(){
            let mut l: u16=0; 

            let rayline: Line = Line::new(i.clone(), Point { x: i.x.clone()+1000 as unit, y: i.y.clone() });

            for q in ot.lines.iter(){
                if let Some(_a) = rayline.pass_through(q) {
                    l+=1;
                    if q.slope.abs() <=1e-8 as unit{
                        l+=1;
                    }
                }
                

            }
            if l%2!=0{
                return Some(i.clone());
            }


        }

        for i in ot.points.iter(){
              let mut l: u16=0; 

              let rayline: Line = Line::new(i.clone(), Point { x: i.x.clone()+1000 as unit, y: i.y.clone() });

              for q in self.lines.iter(){
                  if let Some(_a) = rayline.pass_through(q) {
                      l+=1;
                      if q.slope.abs() <=1e-8 as unit{
                          l+=1;
                      }
                  }
                

              }
              if l%2!=0{
                  return Some(i.clone());
              }


        }
        let q=0;
        for i in self.lines.iter(){

          if let Some(A) =i.pass_through(&ot.lines[0]){
            return Some(A);
          }  
        }    


        

        None
    }
    
    pub fn area(&mut self)->unit{
      if let None = self.area{
      	let mut pos :unit= 0 as unit;
      	let mut neg:unit =0 as unit;
      	let len = self.points.len();
  			for i in 0..(len){
        	let wrap = (i+1)%len;
        	pos+= self.points[i].x*self.points[wrap].y;
        	neg += self.points[wrap].x*self.points[i].y;
      	}
      
      	let sum = ((pos-neg).abs())/(2.0 as unit);
        self.area = Some(sum);
      }
          
      self.area.unwrap()
      
      
            

		}
}
