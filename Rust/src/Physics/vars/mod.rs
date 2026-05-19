use std::{clone, fmt::{Debug, write}};
use std::ops::{Index, IndexMut};

//This File is mostly just for an easier to use and acces container for variables
use super::unit;
#[derive(Clone)]
pub struct Var<T:index_get, const N:usize>
where T:Clone{

    
    pub(in super::super)index : T,
    pub(in super::super)elements : [Option<unit>; N],
    pub(in super::super)where_i: usize,
    pub(in super::super)size: usize




}

pub trait index_get{
    fn as_usize(&self)-> usize;
}

impl<T:index_get, const N:usize> Var<T,N>
where T:Clone{
    pub fn is_safe(&self, inde : usize) ->bool{
        inde< N
    }
    pub fn get(&self, inde : T)->Result<Option<unit>, VarErr>{
        if !self.is_safe(inde.as_usize()){return Err(VarErr);}
        
        Ok(self.elements[inde.as_usize()])
    }
    pub fn can_I_solve(&self, oh: &[T] ) -> Option<bool>{
        for i in oh{
            if i.as_usize()>=N {return None;}
            match self.elements[i.as_usize()] {
                None => {return Some(false);},
                Some(_a) => {continue;}
            }
            
        };
        Some(true)
    }
    

    pub fn new(j: T)->Self{
        Self { elements: [None;N], index : j, where_i:0, size: N}
    }

    pub fn set(&mut self, j: T, i: f64)-> Result<Option<unit>, VarErr>{
        self.elements[j.as_usize()] = Some(i);
        Ok(self.elements[j.as_usize()])
    }
    
    pub(in super::super)fn solve_pyth(&self, x: unit, y:unit)-> unit{
        (x*x+y*y).sqrt()
     }
    pub(in super::super)fn x_over_cos(&self, x:unit, ang:unit)-> unit{
        //let x = self[LineVar::Vy].unwrap();
        //let ang = b*(3.14 as unit)/180 as unit;
		(x)/(ang.cos())
    }
    pub(in super::super)fn y_over_sin(&self, y:unit, angle:unit)-> unit{
        //let x = self[LineVar::Vy].unwrap();
        //let ang = angle*(3.14 as unit)/180 as unit;
		(y)/(angle.sin())
    }
    pub(in super::super)fn x_over_y(&self, a:unit, b:unit) -> unit{
        a/b
    }  
    pub(in super::super)fn x_times_y(&self, a:unit, b:unit)-> unit{
        a*b
    }    
    pub(in super::super)fn rev_pyth(&self, mag:unit, other:unit)-> unit{
        if mag<other { panic!();}
        (mag*mag-other*other).sqrt()
    }
    pub(in super::super)fn mag_times_cos(&self, x:unit, ang:unit)-> unit{
        //let x = self[LineVar::Vy].unwrap();
        //let ang = b*(3.14 as unit)/180 as unit;
		(x)*(ang.cos())
    }
    pub(in super::super)fn mag_times_sin(&self, y:unit, ang:unit)-> unit{
        //let x = self[LineVar::Vy].unwrap();
       // let ang = angle*(3.14 as unit)/180 as unit;
		(y)*(ang.sin())
    }  

	pub (in super::super) fn set_all_zero(mut self) -> Self{
		for i in self.elements.iter_mut(){
			*i = Some(0.0)
		}
		self
	}	

    
}

pub struct VarErr;

impl<T,const N:usize>  Iterator for Var<T,N>
where T:index_get, T:Clone{
    type Item = Option<unit>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.where_i >= N{
            self.where_i =0;
            None
        }else{
            let h = Some(self.elements[self.where_i].clone());
            self.where_i = self.where_i+1;
            h
            
            
        }
    }
}

impl Debug for VarErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "There was an error")
    }
}

impl<T,const N: usize> Index<T> for Var<T,N> 
where T:Clone+index_get{
	type Output = Option<unit>;
    
	fn index(&self, index: T) -> &Self::Output {
        let g = index.as_usize();
        if g>=N {return &None;}
        &self.elements[g] 
    }
}   
impl<T,const N: usize> IndexMut<T> for Var<T,N> 
where T:Clone+index_get{

    
	fn index_mut(& mut self, index: T) -> &mut Self::Output {
        let g = index.as_usize();
        if g>=N {panic!()}
        &mut self.elements[g] 
    }
}                
