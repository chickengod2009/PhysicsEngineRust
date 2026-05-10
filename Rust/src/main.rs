use jni::JNIEnv;
use jni::objects::JObject;
use jni::sys::{jint, jlong};

mod Physics;
use crate::Physics::force::{variable::ForceIndex, forceing::Force};
use crate::Physics::objects::Object;
use crate::Physics::objects::polygons::{Point, Polygon};
use crate::Physics::vars::*;

#[unsafe(no_mangle)]
pub extern "system" fn Java_App_hello(
    _env: JNIEnv,
    _obj: JObject,
){

    println!("OI");
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_App_close(
    _env: JNIEnv,
    _obj: JObject,
    _handle: jlong,
) {

    unsafe {
        // Convert raw pointer back into Box to drop it
        // This will free the heap memory
        drop(Box::from_raw(_handle as *mut App));
    }

}

struct App{
    yo: i32
}


#[unsafe(no_mangle)]
pub extern "system" fn Java_App_getStructPointer(
    _env: JNIEnv,
    _obj: JObject,
) -> jlong {
    let hg =Box::new(App{yo: 7 as i32});
    Box::into_raw(hg) as jlong

}

#[unsafe(no_mangle)]
pub extern "system" fn Java_App_doSomething(
    _env: JNIEnv,
    _obj: JObject,
    _i: jlong,
    hi: jint
){
    
    let jh : *mut App = _i as *mut App;

    unsafe{

        (*jh).yo =hi;

    }

}
fn main(){

    println!("k");
    let mut g : Var<usize, 5>= Var::new(5);
    g.set(0, 55_f64).expect("k");
    g.set(1, 4_f64).expect("l");

    let l=g.get(0).expect("k").unwrap();

    for i in g{
        if let Some(a) =i{
            println!("{}", a);
        }else{
            println!("None");
        }
    }


    let bb : Force = Var::new(ForceIndex::A);

    let q: Vec<Point> = vec![Point::new(0_f64,0_f64),Point::new(0_f64,4_f64),Point::new(4_f64,4_f64),Point::new(4_f64,0_f64)];

    let gio = Polygon::new(q);

    for i in gio.angles_by_ref().iter(){
        println!("{}", &i);
    }

    //let bm = Object::new(gio, 50 as f64, true, true);

    

    


    


}
// [lib]
// crate-type = ["cdylib"]
//java "-Djava.library.path=." App

impl index_get for usize{
    fn as_usize(&self)-> usize {
        *self
    }
}

