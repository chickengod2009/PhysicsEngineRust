use crate::Physics::vars::index_get;



pub enum LinVar{
    V,
    Vx,
    Vy,
    M,
    P,
    Px,
    Py,
    Ang
}

impl Clone for LinVar{
    fn clone(&self) -> Self {
        match self {
            &Self::V => Self::V,
            &Self::Vx => Self::Vx,
            &Self::Vy => Self::Vy,
            &Self::M => Self::M,
            &Self::P => Self::P,
            &Self::Px => Self::Px,
            &Self::Py => Self::Py,
            &Self::Ang => Self::Ang,

        }
    }
}

impl index_get for LinVar  {
    fn as_usize(&self)-> usize {
        match self {
            &Self::V => 0,
            &Self::Vx => 1,
            &Self::Vy => 2,
            &Self::M => 3,
            &Self::P => 4,
            &Self::Px => 5,
            &Self::Py => 6,
            &Self::Ang => Self::7,
        }
    }
}