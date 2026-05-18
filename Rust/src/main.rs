
mod Physics;

use iced::application::BootFn;
use iced::widget::{Column, button, column as c, row, text};
//use iced::window::frames;
use iced::{
    widget::canvas,
    widget::Canvas,
    //Color,
    Element,
    Length,
    //Point,
    Rectangle,
    Renderer,
    //Settings,
    Theme,

};
use iced::widget::progress_bar;

use iced::widget::canvas::{
    Cache, Geometry, 
};


use crate::Physics::control::system::{self, System};


use crate::Physics::objects::Object;
use crate::Physics::objects::polygons::{Point, Polygon, Vect};

fn main() -> iced::Result {
   //et mut final_state : State;
   
   
   
   
    //iced::application("A counter", update, view)
      //  .subscription(subscription).window_size(size)..run_with(|| (syst,Task::none()))
      iced::application(boot, update, view).subscription(subscription).resizable(true).window_size(Size::new(1400.0, 850.0)).run()


  

}

   fn boot() -> (State, Task<Message>) {
       
   let mut imput_final : i32=0;
   let mut imput : String = String::new();
   loop{
      imput.clear();
      println!("Type in th number for the coressponding test:\n1:Projetile\n2:Kinetic energy and Momentum");

      

      

      let res: Result<usize, std::io::Error> = std::io::stdin().read_line(&mut imput);
      if let Err(g) = res{
         panic!("{}", g);
      }
      if imput.trim() == "s"{
         panic!()
      }   
      //println!("{:?}", &imput);
      let imp_res: Result<i32, ParseIntError> = imput.trim().parse();
      let Err(a) = imp_res else{
         imput_final = imp_res.unwrap();
         if imput_final > 5{
            continue;
         }else {
            break;
         }
      };

      println!("{:?}, not a selectable Number", a);
      

   }
   let syst : State;

   let mut size : Size = Size::new(1024.0, 768.0);
   match imput_final{

      1=> syst = set_up_ptojectile(),

      2 => syst = set_up_planets(),
      3 => syst = State::bouncing_about(),
      4 => syst = State::bucket(),
      5 => syst = set_up_billards(),

      _ => panic!()
   };
      (syst, Task::done(Message::Start))
   }

impl Default for State{
      fn default() -> Self {
         State::new(System::default())
      }
   }

//#[derive(Default)]
struct State{
   system : System,
   cache : canvas::Cache,
   read :bool,
   file : File,
   col_file: File,
   bigTicks : u32
}

impl State{

   pub fn new(state : System) -> Self{
      Self { system: state, cache: Cache::new(), read: false , file  : OpenOptions::new()
            .create(true)
            .write(true).truncate(true)
            .open("Log.txt")
            .expect("Could not open Log.txt"),
         bigTicks : 0,
      col_file: OpenOptions::new().create(true).write(true).truncate(true).open("Collision_Log.txt").expect("Could not Open Collision Log")}
   }
   pub fn test() -> Self{

      //let [t,l,r,b] = Self::frame();
      
      let mut poly : Polygon = Polygon::new(vec![Point::new(200.0,400.0),Point::new(230.0,400.0),Point::new(215.0,430.0),]);
      let mut poly2 : Polygon = Polygon::new(vec![Point::new(500.0,400.0),Point::new(530.0,400.0),Point::new(515.0,430.0),]);
      let obj1 : Object = Object::new(&mut poly, 2.0e18, false, true, 1, 2.0, true, Color::from_rgb8(150, 0, 150), String::from("One")).with_starting_v(Vect::new(0.0,33.0)).yes_i_am_attactive();
      let obj2 : Object = Object::new(&mut poly2, 2.0e18, false, true, 1, 2.0, true, Color::from_rgb8(150, 0, 150), String::from("One")).with_starting_v(Vect::new(0.0,-3.5)).yes_i_am_attactive();
      let syst : System = System::new(vec![obj1,obj2], None, true, false);
      Self::new(syst)
   }

   pub fn projectile(angle: f64, mag : f64, air_res : Option<f64>, mass: f64) -> Self {
       //let [t,l,r,b] = Self::frame();
       let [t,_l,_r,_b] = Self::frame();
       let offset :f64  =40.0;
      let mut poly : Polygon = Polygon::new(vec![
         Point::new(0.0,740.0+offset),
         Point::new(10.0,745.0+offset),
         Point::new(20.0,740.0+offset),
         Point::new(25.0,730.0+offset),
         Point::new(20.0,720.0+offset),
         Point::new(10.0,715.0+offset),
         Point::new(0.0,720.0+offset),
         Point::new(-5.0,730.0+offset),
         ]);
      let mag = mag*2.0;   
      let angle  = ((angle*f64::consts::PI)/180.0);
      let v : Vect = Vect::new(mag*angle.cos(), mag*angle.sin());
      let obj1 : Object = Object::new(&mut poly, mass, false, true, 1, 5.0,true, Color::from_rgb8(150, 0, 150), String::from("Ball"))
         .with_starting_v(v);
      let mut syst : System = System::new(vec![obj1,t], Some(Point::new(0.0, 800.0)), true, air_res.is_some());
      syst.start();
      
      Self::new(syst)
   }
   pub fn frame() -> [Object; 4]{
      let thickness = 30.0;
let w = 1400.0;
let h = 850.0;

// Bottom wall (y=0 is bottom in physics coords)
let frame_bottom = Object::new(&mut Polygon::new(vec![
    Point::new(0.0,       0.0),
    Point::new(w,         0.0),
    Point::new(w,         thickness),
    Point::new(0.0,       thickness),
]), 1.0e4, true, true, 60, 2.0,false, Color::from_rgb8(150, 0, 150), String::from("One"));

// Top wall
let frame_top = Object::new(&mut Polygon::new(vec![
    Point::new(0.0,       h - thickness),
    Point::new(w,         h - thickness),
    Point::new(w,         h),
    Point::new(0.0,       h),
]), 1.0e4, true, true, 61, 2.0, false, Color::from_rgb8(150, 0, 150), String::from("One"));

// Left wall
let frame_left = Object::new(&mut Polygon::new(vec![
    Point::new(0.0,       0.0),
    Point::new(thickness, 0.0),
    Point::new(thickness, h),
    Point::new(0.0,       h),
]), 1.0e4, true, true, 62,2.0, false, Color::from_rgb8(150, 0, 150), String::from("One"));

// Right wall
let frame_right = Object::new(&mut Polygon::new(vec![
    Point::new(w - thickness, 0.0),
    Point::new(w,             0.0),
    Point::new(w,             h),
    Point::new(w - thickness, h),
]), 1.0e4, true, true, 63,2.0,false, Color::from_rgb8(150, 0, 150), String::from("One"));
      [frame_top,frame_left,frame_right,frame_bottom]
   }



   pub fn planets(mass1: f64, mass2: f64, r:f64, v:f64) -> Self{


      let mut poly1 = Polygon::new(vec![

         Point::new(675.0, 425.0),
         Point::new(690.0,412.5),
         Point::new(700.0,400.0),
         Point::new(710.0,412.5),
         Point::new(715.0,425.0),
         Point::new(710.0,437.5),
         Point::new(700.0,450.0),
         Point::new(690.0,437.5),
         //Point::new(600.0,425.0),

         

      ]);
      let mut poly2 = Polygon::new(vec![

         Point::new(675.0-r, 425.0),
         Point::new(690.0-r,412.5),
         Point::new(700.0-r,400.0),
         Point::new(710.0-r,412.5),
         Point::new(715.0-r,425.0),
         Point::new(710.0-r,437.5),
         Point::new(700.0-r,450.0),
         Point::new(690.0-r,437.5),
         //Point::new(600.0,425.0),

         

      ]);
      //let v_outer  = (6.67e-13 * mass1 / mass2).sqrt(); // ~57
      let v_center = v * mass2 / mass1;         // ~0.057
      //2

 
      let obj1: Object = Object::new(&mut poly1, mass1, false, false, 0, 10.0, true, Color::from_rgb8(200, 30, 200), String::from("Center Planet")).yes_i_am_attactive().with_starting_v(Vect::new(0.0, v_center));
      let obj2: Object = Object::new(&mut poly2, mass2, false, false, 0, 10.0, true, Color::from_rgb8(200, 30, 200), String::from("Outer Planet")).with_starting_v(Vect::new(0.0, v)).yes_i_am_attactive();
      println!("center com: {}, {}", obj1.com().x(), obj1.com().y());
println!("outer  com: {}, {}", obj2.com().x(), obj2.com().y());
println!("actual r: {}", ((obj2.com().x()-obj1.com().x()).powi(2) + (obj2.com().y() - obj1.com().y()).powi(2)).sqrt());
      let syst = System::new(vec![obj1,obj2], None, false, false);
      State::new(syst)
   }

   pub fn bouncing_about() -> Self{
      let [t,l,r,b] = Self::frame();
      let mut poly : Polygon = Polygon::new(vec![
         Point::new(675.0, 350.0),
         Point::new(700.0, 200.0),
         Point::new(725.0, 350.0),
      ]);
      let mut poly2 : Polygon = Polygon::new(vec![
         Point::new(675.0, 650.0),
         Point::new(700.0, 500.0),
         Point::new(725.0, 650.0),
      ]);
      let mut poly3 : Polygon = Polygon::new(vec![
         Point::new(875.0, 350.0),
         Point::new(900.0, 200.0),
         Point::new(925.0, 350.0),
      ]);


      let obj1 : Object = Object::new(&mut poly, 10.0, false, true, 0, 10.0, true, Color::from_rgb8(20, 60, 120), String::from("2")).with_starting_v(Vect::new(10.0, 0.0));
      let obj2 : Object = Object::new(&mut poly2, 10.0, false, true, 0,10.0, true, Color::from_rgb8(180, 60, 120), String::from("3")).with_starting_v(Vect::new(0.0, 8.0));
      let obj3 : Object = Object::new(&mut poly3, 10.0, false, true, 0, 10.0, true, Color::from_rgb8(120, 160, 30), String::from("1")).with_starting_v(Vect::new(-10.0, 0.0));
      let syst : System = System::new(vec![t,b,l,r,obj1,obj2,obj3], None, true, false);
      State::new(syst)
   }


   pub fn bucket() -> Self{
      let mut poly : Polygon = Polygon::new(vec![
         Point::new(0.0,740.0),
         Point::new(10.0,745.0),
         Point::new(20.0,740.0),
         Point::new(25.0,730.0),
         Point::new(20.0,720.0),
         Point::new(10.0,715.0),
         Point::new(0.0,720.0),
         Point::new(-5.0,730.0),
         ]);

      let offset: f64 = 20.0;   
      let mut poly2 : Polygon = Polygon::new(vec![
         Point::new(1300.0,740.0+offset),
         Point::new(1310.0,745.0+offset),
         Point::new(1320.0,740.0+offset),
         Point::new(1325.0,730.0+offset),
         Point::new(1320.0,720.0+offset),
         Point::new(1310.0,715.0+offset),
         Point::new(1300.0,720.0+offset),
         Point::new(1295.0,730.0+offset),
         ]);  
      let mut polyb: Polygon = Polygon::new(vec![
         Point::new(300.0,500.0+offset),
         Point::new(400.0,500.0+offset),
         Point::new(400.0,600.0+offset),
         Point::new(300.0,600.0+offset),
         
         ]);    
      let obj1 : Object = Object::new(&mut poly, 10.0, false, true, 1, 10.0, true, Color::from_rgb8(60, 60, 20), String::from("Ball One")).with_starting_v(Vect::new(19.0, 22.0));   
      let obj2 : Object = Object::new(&mut poly2, 10.0, false, true, 1, 10.0, true, Color::from_rgb8(60, 60, 20), String::from("Ball Two")).with_starting_v(Vect::new(-19.0, 22.0));   
      let objb : Object = Object::new(&mut polyb, 10.0, true, true, 1, 10.0, false, Color::from_rgb8(60, 60, 20), String::from("Basket"));  
      let mut susy = System::new(vec![obj1,obj2,objb], Some(Point::new(0.0, 0.)), true, true) ;
      susy.start();
      State::new(susy)
   }

   pub fn billards(mass1: f64, mass2:f64, v: f64, and: f64) -> Self{
      let anng = and*f64::consts::PI/180.0;
      let ve = Vect::new(v*anng.cos(), v*anng.sin());
      let mut polyh =  Polygon::new(vec![
         Point::new(300.0,540.0),
         Point::new(310.0,545.0),
         Point::new(320.0,640.0),
         Point::new(325.0,530.0),
         Point::new(320.0,520.0),
         Point::new(310.0,415.0),
         Point::new(300.0,520.0),
         Point::new(295.0,530.0),
         ]);
      let mut polyt =  Polygon::new(vec![
         Point::new(500.0,540.0),
         Point::new(510.0,545.0),
         Point::new(520.0,640.0),
         Point::new(525.0,530.0),
         Point::new(520.0,520.0),
         Point::new(510.0,415.0),
         Point::new(500.0,520.0),
         Point::new(495.0,530.0),
         ]);   

      let objh :Object =Object::new(&mut polyh, mass2, false, true, 0, 2.0, true, Color::BLACK, String::from("Hitter")).with_starting_v(ve);   
      let objt :Object =Object::new(&mut polyt, mass1, false, true, 0, 2.0, true, Color::BLACK, String::from("Hitted"));
      let syst = System::new(vec![objh,objt], None, false, false);
      State::new(syst)
   }

}

#[derive(Debug, Clone)]
enum Message {
   
   Tick,RequestLog,Start
}




fn update(state: &mut State, message: Message) {
   match message{
      Message::Tick => {
         
         state.cache.clear();
         
         state.system.tick();
         for i in state.system.check_for_col_suggestion().iter(){

            let form : String = format!("\n\n//Collision//\n\n{}", i);
            state.col_file.write(form.as_bytes()).expect("203");
            
         }
         //let com = state.system.objs()[0].com();
            //println!("x: {}, y: {}", com.x(), com.y());
         // for i in state.system.objs_mut().iter_mut(){
         //    if i.com().y()>=800.0{
         //       i.reverse_v();
               
         //    }
         // }
        
         


      }
      
      Message::RequestLog => {
         state.bigTicks +=1;
         let log: Vec<Physics::objects::ObjectLog> = state.system.request_object_logs();
         state.file.write("\n\n\n".as_bytes()).expect("22");

         state.file.write(state.bigTicks.to_string().as_bytes()).expect("Error in writing bytes");
         
         for i in log.into_iter(){
            let form =format!("\n\n{}\n\n", i);
            state.file.write(form.as_bytes()).expect("l");
         }
      },
      Message::Start => state.read = true
   }
}

fn view(state: &'_ State) -> Element<'_, Message> {
   
      //println!("{}", state.system.me());
      let mut col: Column<'_, Message> = Column::new();
      if state.system.has_earth(){
         col = c![
            text("Kinetic Energy"),
               (progress_bar(0.0..=1000.0, (*state.system.ke()as f32)))
                  .vertical().length(200.0).girth(20.0),
               text("Mechanical Energy"),
               (progress_bar(0.0..=100.0, (*state.system.me()as f32)))
                  .vertical().length(200.0).girth(20.0), 
               text("Potential Energy"),
               (progress_bar(0.0..=1000.0, (*state.system.pot()as f32)))
                  .vertical().length(200.0).girth(20.0),    
                  ];       
                  
      }          
                
      row![
         Canvas::new(MyCanvas{ system :& state.system, cache : &state.cache})
        .width(Length::Fill)
        .height(Length::Fill),
        col

        
      ].into()
      
   
      
      
}

struct MyCanvas <'a>{
   system : & 'a System,
   cache : & 'a Cache
}

impl<'a> canvas::Program<Message> for MyCanvas<'a> {

    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {

      //   let mut frame = Frame::new(
      //       renderer,
      //       bounds.size(),
      //   );

        // Draw circle
        let geo = self.cache.draw(renderer, bounds.size(), 
        |frame| {
          
    
         for i in self.system.objs().iter(){
            i.draw(frame);
         }
    
    

        }
      );
        vec![geo]
    }
}

use iced::{
    Color, Size, Subscription, Task
};
use core::f64;
use std::fmt::Formatter;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::num::ParseIntError;
use std::time::Duration;

fn subscription(
    state: &State,
) -> Subscription<Message> {

   if !state.read {return Subscription::none();}
   Subscription::batch(vec![
        iced::time::every(Duration::from_secs_f64(1.0 / 60.0))
            .map(|_| Message::Tick),
        iced::time::every(Duration::from_secs(2))
            .map(|_| Message::RequestLog),
        
    ])
   
   
   
}




fn set_up_ptojectile() -> State{
   let mut vars: [f64; 4] = [0_f64;4];

   let nar_name: [&str; 4] = ["magnitiude velocity of the ball (m/s)","launch angle (deg)", "mass of the ball (kg)", "air resistance (coef, with recommened max of one)"];
   let mut input : String = String::new();
   let mut i: usize =0;
   loop{
      println!("Please select a valid number for the {}", nar_name[i]);
      std::io::stdin().read_line(&mut input).expect("std err");
      if input.trim() == "s"{
         panic!("Programed Stopped by input")
      }
      let Ok(a) = input.trim().parse::<f64>() else {
          println!("That is not a number! {:?}", input);
          continue;
      };

      vars[i] = a;
      i+=1;
      if i >3{
         break;
      }
      input.clear();

      

   };

   let [mag, ang, mass, air] = vars;
   let mut aiir = Some(air);
   if air.abs()<1e-10{
      aiir = None;
   }
   State::projectile(ang, mag, aiir, mass)
}


fn set_up_planets() -> State{
   let mut input : String = String::new();
   fn bad(s: &String){
      if s.trim() == "s"{
         panic!("Project was stopped")
      }   
   }
   let mut vars = [0f64;4];
   loop{
      input.clear();
      println!("What is the mass of your center object (kg)?");
      std::io::stdin().read_line(&mut input).expect("kj");
      bad(&input);
      if let Ok(a) = input.trim().parse::<f64>(){
         vars[0] = a;
         break;
      }else{
         println!("Not a number!");
      }
   
   }
   loop{
      input.clear();
      println!("What is the mass of your outside object (kg)?");
      std::io::stdin().read_line(&mut input).expect("kj");
      bad(&input);
      if let Ok(a) = input.trim().parse::<f64>(){
         vars[1] = a;
         break;
      }else{
         println!("Not a number!");
      }
      
   }
   loop{
      input.clear();
      println!("What is the distance between the two centers of masses (pixel= 100m)? Recomended 200");
      std::io::stdin().read_line(&mut input).expect("kj");
      bad(&input);
      if let Ok(a) = input.trim().parse::<f64>(){
         vars[2] = a;
         break;
      }else{
         println!("Not a number!");
      }
      
   }
   //let rec = (6.67e-13*(vars[0]/(vars[2]))).sqrt();
   loop{
      input.clear();
      println!("What is the velocity of your outside object?");
      std::io::stdin().read_line(&mut input).expect("kj");
      bad(&input);
      if let Ok(a) = input.trim().parse::<f64>(){
         vars[3] = a;
         break;
      }else{
         println!("Not a number!");
      }
      
   }

   //println!("rr {}", vars[2]);

   State::planets(vars[0], vars[1], vars[2], vars[3])

}
// Subscription::batch(vec![
//         iced::time::every(Duration::from_secs_f64(1.0 / 60.0))
//             .map(|_| Message::Tick),
//         iced::time::every(Duration::from_secs(1))
//             .map(|_| Message::SecondTick),
//         iced::time::every(Duration::from_millis(500))
//             .map(|_| Message::HalfSecondTick),
//     ])


fn set_up_billards() -> State{


   let mut input : String = String::new();
   fn bad(s: &String){
      if s.trim() == "s"{
         panic!("Project was stopped")
      }   
   }
   let mut vars = [0f64;4];
   loop{
      input.clear();
      println!("What is the mass target(kg)?");
      std::io::stdin().read_line(&mut input).expect("kj");
      bad(&input);
      if let Ok(a) = input.trim().parse::<f64>(){
         vars[0] = a;
         break;
      }else{
         println!("Not a number!");
      }
   
   }
   loop{
      input.clear();
      println!("What is the mass of your hitting object (kg)?");
      std::io::stdin().read_line(&mut input).expect("kj");
      bad(&input);
      if let Ok(a) = input.trim().parse::<f64>(){
         vars[1] = a;
         break;
      }else{
         println!("Not a number!");
      }
      
   }
   loop{
      input.clear();
      println!("What is the magnitude of the velocity of the hitting object");
      std::io::stdin().read_line(&mut input).expect("kj");
      bad(&input);
      if let Ok(a) = input.trim().parse::<f64>(){
         vars[2] = a;
         break;
      }else{
         println!("Not a number!");
      }
      
   }
   //let rec = (6.67e-13*(vars[0]/(vars[2]))).sqrt();
   loop{
      input.clear();
      println!("What is the angke of initial vel? (deg)?");
      std::io::stdin().read_line(&mut input).expect("kj");
      bad(&input);
      if let Ok(a) = input.trim().parse::<f64>(){
         vars[3] = a;
         break;
      }else{
         println!("Not a number!");
      }
      
   }

   //println!("rr {}", vars[2]);

   State::billards(vars[0], vars[1], vars[2], vars[3])


}