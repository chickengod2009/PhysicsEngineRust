use crate::Physics::vars::index_get;

pub enum ForceIndex{

    F,A,M,Fx,Fy,Ax,Ay


}

impl index_get for ForceIndex{
    fn as_usize(&self)-> usize {
        match self {
            &ForceIndex::F => 0,
            &ForceIndex::M=> 1,
            &ForceIndex::A=> 2,
            &ForceIndex::Ax => 3,
            &ForceIndex::Ay=> 4,
            &ForceIndex::Fx=> 5,
            &ForceIndex::Fy=> 6,
        }
    }
}
impl Clone for ForceIndex {
    fn clone(&self) -> Self {
        match self {
            &ForceIndex::F => ForceIndex::F,
            &ForceIndex::M=> ForceIndex::M,
            &ForceIndex::A=> ForceIndex::A,
            &ForceIndex::Ax => ForceIndex::Ax,
            &ForceIndex::Ay=> ForceIndex::Ay,
            &ForceIndex::Fx=> ForceIndex::Fx,
            &ForceIndex::Fy=> ForceIndex::Fy,
        }
    }
}

