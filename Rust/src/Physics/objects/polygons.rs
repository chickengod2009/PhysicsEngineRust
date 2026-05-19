use core::f64;
use std::{fmt::{Display, write}, ops::{Add, AddAssign}};
use iced::{
    Color, Element, Length, Rectangle, Renderer, Settings, Theme, widget::{Canvas, canvas::{self, Path}}
};
use iced::Point as IcedPoint;

use crate::Physics::unit;
//type unit = f64;

#[derive(PartialEq)]
pub struct Point{
    x: unit, y:unit
}
#[derive(PartialEq)]
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
#[derive(PartialEq, Clone)]
pub struct Angle{
    a:Line,b:Line,
    shared_point: Point,
    angle: unit,
    angle_to_horz: unit,
}
#[derive(PartialEq, Clone)]
pub struct Polygon{
    points: Vec<Point>,
    lines: Vec<Line>,
    angles: Vec<Angle>,
    center: Point,
    area: Option<unit>,
    r: unit
}

impl Point{
  pub fn new(d: f64, b: f64) ->Self{
    Self{x:d, y:b}
  }
  pub fn x(&self)->unit{
    self.x
  }
  pub fn y(&self)-> unit{
    self.y
  }
}
impl Line {
    pub fn new(a: Point, b: Point) -> Self{
        let mut h =Self { a:a, b:b, slope:0 as unit, k: 0 as unit, is_vert:false, angle_to_horz: 0 as unit};
        h.calc_slope();
        h
    }

    pub fn length(&self) -> unit{
        let ax = self.a.x;
        let ay = self.a.y;
        let bx = self.b.x;
        let by = self.b.y;
        ((ax.max(bx) - ax.min(bx))*(ax.max(bx) - ax.min(bx)) + (ay.max(by) - ay.min(by))*(ay.max(by) - ay.min(by))).sqrt()
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
    let mut ret =Self{points: q, lines: lines, angles: ang, center: Point { x: 0.0, y: 0.0 }, area: None, r:0.0};
    ret.find_r();
    //ret.area();
    
    ret
  }

  pub fn find_r(&mut self){
    let mut rr :unit =0.0;
    for i in self.lines.iter(){
        rr.max(i.length());
    }
    self.r = rr;
  }
  pub fn r(&self) -> unit{
    self.r
  }
    pub fn points(&self) -> &Vec<Point>{
        &self.points
    }
  
    pub fn find_cent(&mut self) -> &Point{
          	        let mut x : unit =0 as unit;
            let mut y: unit =0 as unit;
            let s: unit=self.points.len() as unit;
            for i in self.points.iter(){
                x+=i.x;
                y+=i.y
            }
            let ret: Point = Point::new(x/s, y/s);
            self.center = (ret);
        
        &self.center
    }
    pub fn cent(&self) -> &Point{
        &self.center
        
    }
    pub fn cent_mut(&mut self) -> &mut Point{
        &mut self.center
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

    //I found some SAT Collision online that use the normals of the sides to fin overlapping acceses and collision

    pub fn collision(&mut self, ot: &mut Self) -> Option<Collision> {

    let mut smallest_overlap = f64::MAX as unit;

    let mut smallest_axis = Vect::new(0 as unit, 0 as unit);

    // test all axes from both polygons
    for poly in [&self.lines, &ot.lines] {

        for line in poly.iter() {

            // edge direction
            let dx = line.b.x - line.a.x;
            let dy = line.b.y - line.a.y;

            // perpendicular axis
            let axis = Vect::new(-dy, dx).normalized();

            let (min_a, max_a) = self.project_axis(&axis);
            let (min_b, max_b) = ot.project_axis(&axis);

            // separating axis found
            if max_a < min_b || max_b < min_a {
                return None;
            }

            // overlap amount
            let overlap =
                unit::min(max_a, max_b)
                -
                unit::max(min_a, min_b);

            // smallest overlap determines collision normal
            if overlap < smallest_overlap {

                smallest_overlap = overlap;
                smallest_axis = axis;
            }
        }
    }

    // ensure normal points self -> other
    let center_dir = Vect::new(
        ot.find_cent().x - self.cent().x,
        ot.cent().y - self.cent().y,
    );

    if center_dir.dot(&smallest_axis) > 0 as unit {

        smallest_axis = smallest_axis * (-1 as unit);
    }

    // approximate contact point
    let contact = ot.points.iter()
    .min_by(|a, b| {
        let da = a.x * smallest_axis.x() + a.y * smallest_axis.y();
        let db = b.x * smallest_axis.x() + b.y * smallest_axis.y();
        da.partial_cmp(&db).unwrap()
    })
    .unwrap();

    let point = Point::new(contact.x, contact.y);

    Some(Collision {
        point,
        normal: smallest_axis,
        depth: smallest_overlap,
    })
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


    pub fn translation(&mut self, trans: Translation2d){
      	self.cent_mut().add_assign(&trans);
        for i in self.points.iter_mut(){
        
            i.add_assign(&trans);
        }
        for i in self.lines.iter_mut(){
            i.add_assign(&trans);
        }
        for i in self.angles.iter_mut(){
            i.add_assign(&trans);
            
            
        }
        
    }
    pub fn rotation(&mut self, trand: Rotational2d ){

        let trnn = Rotational2d::new(trand.angle*50.0);
        //Smoother rotation
      	
        let trans : RotSinCos = RotSinCos::new(trnn);
        
		for i in self.points.iter_mut(){
        
            i.add_assign((&trans,&self.center));
        }
        for i in self.lines.iter_mut(){
            i.add_assign((&trans,&self.center));
        }
        for i in self.angles.iter_mut(){
            i.add_assign((&trans,&self.center));
            
            
        }
    }

    fn project_axis(&self, axis: &Vect) -> (unit, unit) {

    let mut min =
        self.points[0].x * axis.x()
        +
        self.points[0].y * axis.y();

    let mut max = min;

    for p in self.points.iter() {

        let proj =
            p.x * axis.x()
            +
            p.y * axis.y();

        if proj < min {
            min = proj;
        }

        if proj > max {
            max = proj;
        }
    }

    (min, max)
}
    
    



}



#[derive(Clone)]
pub struct Rotational2d{
    angle: unit
}
#[derive(Clone)]
pub struct RotSinCos{
  	sin : unit,
    cos : unit
}
impl RotSinCos{
	pub fn new(deg : Rotational2d) -> Self{
  	    let (sin, cos) = (deg.angle*std::f64::consts::PI/(180 as unit)).sin_cos();
        Self{
            sin :sin,
            cos: cos
        }
    } 
}
impl Rotational2d{
    pub fn new(angle : unit) -> Self{
        Self{angle: angle}
    }
}


#[derive(Clone,Debug,PartialEq)]
pub struct Translation2d{
    x: unit,
    y: unit
}
impl Translation2d{
    pub fn new(x: unit, y: unit) -> Self{
        Self { x:x, y:y }
    }
}
//Direction is negated because window flips the cordinates
impl Add<&Translation2d> for Point{
    type Output = Self;

    fn add(self, rhs: &Translation2d) -> Self::Output {
        
        Self{x: self.x+rhs.x, y: self.y+rhs.y}
        
    }

}
impl AddAssign<&Translation2d> for Point {
    fn add_assign(&mut self, rhs: &Translation2d) {
        self.x+= rhs.x*10.0;
        self.y+=rhs.y*10.0;
        //For more smoother movement
    }
}
impl AddAssign<&Translation2d> for Line {
    fn add_assign(&mut self, rhs: &Translation2d) {
        self.a.add_assign(rhs);
        self.b.add_assign(rhs);
    }
}
impl AddAssign<&Translation2d> for Angle {
    fn add_assign(&mut self, rhs: &Translation2d) {
        self.a.add_assign(rhs);
        self.b.add_assign(rhs);
        self.shared_point.add_assign(rhs);
    }
}
impl AddAssign<(&RotSinCos, &Point)> for Point {
    fn add_assign(&mut self, rhs: (&RotSinCos, &Point)) {
      	
    	let tempx : unit = self.x-rhs.1.x;
			let tempy : unit = self.y-rhs.1.y;
      
      let (sin, cos) = (rhs.0.sin, rhs.0.cos);
      let newx :unit = (tempx*cos-tempy*sin);
      let newy :unit = (tempx*sin+tempy*cos);
      self.x =  rhs.1.x+newx;
      self.y = rhs.1.y+newy;
        
    }
}
impl AddAssign<(&RotSinCos, &Point)> for Line {
    fn add_assign(&mut self, rhs: (&RotSinCos, &Point)) {
        self.a.add_assign(rhs);
        self.b.add_assign(rhs);
    }
}
impl AddAssign<(&RotSinCos, &Point)> for Angle {
    fn add_assign(&mut self, rhs: (&RotSinCos, &Point)) {
        self.a.add_assign(rhs);
        self.b.add_assign(rhs);
        self.shared_point.add_assign(rhs);
    }
}

#[derive(Clone)]
pub struct Collision {
    pub point: Point,
    pub normal: Vect,
    pub depth: unit,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vect {
    x: unit,
    y: unit,
}

impl Vect {

    pub fn new(x: unit, y: unit) -> Self {
        Self { x, y }
    }

    // -------------------------
    // getters
    // -------------------------

    pub fn x(&self) -> unit {
        self.x
    }

    pub fn y(&self) -> unit {
        self.y
    }

    // -------------------------
    // magnitude
    // -------------------------

    pub fn mag(&self) -> unit {

        (self.x * self.x + self.y * self.y).sqrt()
    }

    // -------------------------
    // normalization
    // -------------------------

    pub fn normalized(&self) -> Self {

        let m = self.mag();

        // avoid divide by zero
        if m.abs() <= 1e-8 as unit {

            return Self::new(0 as unit, 0 as unit);
        }

        Self {
            x: self.x / m,
            y: self.y / m,
        }
    }

    // -------------------------
    // dot product
    // -------------------------

    pub fn dot(&self, other: &Self) -> unit {

        self.x * other.x
        +
        self.y * other.y
    }

    // -------------------------
    // 2D cross product
    // returns z-component
    // -------------------------

    pub fn cross(&self, other: &Self) -> unit {

        self.x * other.y
        -
        self.y * other.x
    }

    // -------------------------
    // perpendicular vector
    // -------------------------

    pub fn perp(&self) -> Self {

        Self {
            x: -self.y,
            y: self.x,
        }
    }

    // -------------------------
    // angle from horizontal
    // -------------------------

    pub fn angle(&self) -> unit {

        self.y.atan2(self.x)
    }

    // -------------------------
    // scalar multiply
    // -------------------------

    pub fn scale(&self, s: unit) -> Self {

        Self {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

// ---------------------------------
// operators
// ---------------------------------

use std::ops::{
    
    Sub,
    SubAssign,
    Mul,
    Div,
};

impl Add for Vect {

    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {

        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vect {

    fn add_assign(&mut self, rhs: Self) {

        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vect {

    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {

        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vect {

    fn sub_assign(&mut self, rhs: Self) {

        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<unit> for Vect {

    type Output = Self;

    fn mul(self, rhs: unit) -> Self::Output {

        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<unit> for Vect {

    type Output = Self;

    fn div(self, rhs: unit) -> Self::Output {

        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}


impl Polygon{

	pub fn draw(&self, frame: & mut canvas::Frame, color : Color){
		let path : Path = Path::new(|builder| {
      if let Some(first) = self.points.first() {
          builder.move_to(IcedPoint::new(first.x() as f32 , first.y() as f32));
          for p in &self.points[1..] {
             builder.line_to(IcedPoint::new(p.x as f32, p.y as f32));
          }
          builder.close();
      }
    });
    frame.fill(&path, color); 
  }  

}

impl Default for Polygon{
    fn default() -> Self {
        Polygon::new(vec![
            Point::new(100.0, 100.0),
            Point::new(200.0, 100.0),
            Point::new(200.0, 200.0),
            Point::new(100.0, 200.0),
        ])
    }
}

impl Display for Point{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "X: {}\nY: {}", self.x, self.y)
    }
}