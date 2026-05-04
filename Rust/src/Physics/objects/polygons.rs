use crate::Physics::unit;

pub struct Point{
    x: unit, y:unit
}
pub struct Line{
    a:Point, b:Point,
    slope: unit,
    k: u64,
    is_vert: bool
}
pub struct Ray{
    a:Point, dir:unit
}
pub struct Angle{
    a:Line,b:Line,
    shared_point: Point,
    angle: unit
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

impl Line {
    fn new(a: Point, b: Point) -> Self{
        let mut h =Self { a:a, b:b, slope:0 as unit, k: 0, is_vert:false };
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

        self.slope=slope;
        Some(slope)

    }



}

impl Clone for Point{
    fn clone(&self) -> Self {
        Self{x:self.x, y:self.y}
    }
}

impl Angle{

    fn new(a: Line, b: Line)->Option<Self>{

        if let Some(g) = a.connected(&b){
            let mut g: Angle =Self{a:a,b:b, shared_point: g, angle: 0 as unit };
            g.calcAngle();
            Some(g)
        }else{
            None
        }


    }

    fn calcAngle(&mut self) -> unit{
        
        let tan : unit = self.a.slope as unit;
        let angle: unit = tan.atan2(self.b.slope as unit);
        self.angle = angle;
        angle

    }


}