use crate::Physics::vars::index_get;



pub enum RotVar{
    W,
    
    I,
    L,
    
}

impl Clone for RotVar{
    fn clone(&self) -> Self {
        match self {
            &Self::W => Self::W,
            // &Self::Wx => Self::Wx,
            // &Self::Wy => Self::Wy,
            &Self::I => Self::I,
            &Self::L => Self::L,
            // &Self::Lx => Self::Lx,
            // &Self::Ly => Self::Ly,
        }
    }
}

impl index_get for RotVar  {
    fn as_usize(&self)-> usize {
        match self {
            &Self::W => 0,
            // &Self::Wx => 1,
            // &Self::Wy => 2,
            &Self::I => 1,
            &Self::L => 2,
            // &Self::Lx => 5,
            // &Self::Ly => 6,
        }
    }
}