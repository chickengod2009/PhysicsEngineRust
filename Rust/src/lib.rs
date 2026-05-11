use jni::{
    JNIEnv, objects::{JClass, JObject, JObjectArray}, sys::{
        jdouble,
        jdoubleArray,
        jint,
        jlong,
        jlongArray,
        jobjectArray,
    }
};
mod Physics;

use std::ptr;

// ---------------------------------------------------
// YOUR RUST TYPES
// ---------------------------------------------------



// ---------------------------------------------------
// RustPhys.Object
// ---------------------------------------------------

#[unsafe(no_mangle)]
pub extern "system" fn Java_RustPhys_Object_create_1obj(
    mut env: JNIEnv,
    _class: JClass,
    polygon_ptr: jlong,
) -> jlong {

    let polygon = unsafe {
        &*(polygon_ptr as *mut Polygon)
    };

    let obj = Box::new(
        Object::new(polygon, 1.0, false, true, 0)
    );

    Box::into_raw(obj) as jlong
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_RustPhys_Object_get_1polygon(
    mut env: JNIEnv,
    _class: JClass,
    obj_ptr: jlong,
) -> jlong {

    let obj: &Object = unsafe {
        &*(obj_ptr as *mut Object)
    };

    let poly = Box::new(obj.body().clone());

    Box::into_raw(poly) as jlong
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_RustPhys_Object_destroy(
    mut env: JNIEnv,
    _class: JClass,
    obj_ptr: jlong,
) {

    if obj_ptr == 0 {
        return;
    }

    unsafe {
        drop(Box::from_raw(obj_ptr as *mut Object));
    }
}

// ---------------------------------------------------
// RustPhys.Shapes.Points
// ---------------------------------------------------

#[unsafe(no_mangle)]
pub extern "system" fn Java_RustPhys_Shapes_Points_make_1point(
    mut env: JNIEnv,
    _class: JClass,
    x: jdouble,
    y: jdouble,
) -> jlong {

    let point = Box::new(
        Point::new(x, y)
    );

    Box::into_raw(point) as jlong
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_RustPhys_Shapes_Points_destroy_1point(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {

    if ptr == 0 {
        return;
    }

    unsafe {
        drop(Box::from_raw(ptr as *mut Point));
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_RustPhys_Shapes_Points_get_1cord(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdoubleArray {

    let point = unsafe {
        &*(ptr as *mut Point)
    };

    let arr = env
        .new_double_array(2)
        .unwrap();

    env.set_double_array_region(
        &arr,
        0,
        &[point.x(), point.y()],
    )
    .unwrap();

    arr.into_raw()
}

// ---------------------------------------------------
// RustPhys.Shapes.Polygon
// ---------------------------------------------------
use jni::objects::JLongArray;

use crate::Physics::objects::{Object, polygons::{Point, Polygon}};
#[unsafe(no_mangle)]
pub extern "system" fn Java_RustPhys_Shapes_Polygon_create_1polygon(
    mut env: JNIEnv,
    _class: JClass,
    points_arr: jlongArray,
) -> jlong {

    let points_arr = unsafe {
        JLongArray::from_raw(points_arr)
    };

    let len = env
        .get_array_length(&points_arr)
        .unwrap();

    let mut raw_ptrs = vec![0_i64; len as usize];

    env.get_long_array_region(
        &points_arr,
        0,
        &mut raw_ptrs,
    )
    .unwrap();

    let mut points = Vec::new();

    for ptr in raw_ptrs {

        let point = unsafe {
            &*(ptr as *mut Point)
        };

        points.push(point.clone());
    }

    let poly = Box::new(
        Polygon::new(points)
    );

    Box::into_raw(poly) as jlong
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_RustPhys_Shapes_Polygon_get_1points(
    mut env: JNIEnv,
    _class: JClass,
    poly_ptr: jlong,
) -> jlongArray {

    let poly = unsafe {
        &*(poly_ptr as *mut Polygon)
    };

    let arr = env
        .new_long_array(poly.points().len() as i32)
        .unwrap();

    let mut ptrs = Vec::new();

    for p in poly.points().iter() {

        let boxed = Box::new(p.clone());

        ptrs.push(
            Box::into_raw(boxed) as jlong
        );
    }

    env.set_long_array_region(
        &arr,
        0,
        &ptrs,
    )
    .unwrap();

    arr.into_raw()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_RustPhys_Shapes_Polygon_get_1points_1double_1arr_1arr(
    mut env: JNIEnv,
    _class: JClass,
    poly_ptr: jlong,
) -> jobjectArray {

    let poly = unsafe {
        &*(poly_ptr as *mut Polygon)
    };

    let double_arr_class = env
        .find_class("[D")
        .unwrap();

    let outer = env
        .new_object_array(
            poly.points().len() as jint,
            double_arr_class,
            JObject::null(),
        )
        .unwrap();

    for (i, p) in poly.points().iter().enumerate() {

        let inner = env
            .new_double_array(2)
            .unwrap();

        env.set_double_array_region(
            &inner,
            0,
            &[p.x(), p.y()],
        )
        .unwrap();

        env.set_object_array_element(
            &outer,
            i as jint,
            inner,
        )
        .unwrap();
    }

    outer.into_raw()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_RustPhys_Shapes_Polygon_destroy(
    mut env: JNIEnv,
    _class: JClass,
    poly_ptr: jlong,
) {

    if poly_ptr == 0 {
        return;
    }

    unsafe {
        drop(Box::from_raw(poly_ptr as *mut Polygon));
    }
}