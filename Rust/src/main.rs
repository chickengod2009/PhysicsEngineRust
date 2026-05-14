
mod Physics;

use iced::widget::{button, column as c};
use iced::window::frames;
use iced::{
    widget::canvas,
    widget::Canvas,
    Color,
    Element,
    Length,
    //Point,
    Rectangle,
    Renderer,
    Settings,
    Theme,
};

use iced::widget::canvas::{
    Cache, Frame, Geometry, Path, Stroke
};

use crate::Physics::Vector;
use crate::Physics::control::system::{self, System};
use crate::Physics::force::forceing::{Force, TempForce};
use crate::Physics::force::variable::ForceIndex;
use crate::Physics::objects::Object;
use crate::Physics::objects::polygons::{Point, Polygon, Vect};

fn main() -> iced::Result {
   //et mut final_state : State;
   let mut imput_final : i32=0;
   let mut imput : String = String::new();
   loop{
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
         if imput_final >= 3{
            continue;
         }else {
            break;
         }
      };

      println!("{:?}, not a selectable Number", a);
      imput.clear();

   }
   let syst : State;

   let mut size : Size = Size::new(1024.0, 768.0);
   match imput_final{

      1=> (syst, size) = (set_up_ptojectile(), Size::new(1450.0, 768.0)),

      2 => syst = State::test(),

      _ => panic!()
   };
   
   
   
    iced::application("A counter", update, view)
        .subscription(subscription).window_size(size).run_with(|| (syst,Task::none()))




}
impl Default for State{
      fn default() -> Self {
         Self { system: System::default(), cache: canvas::Cache::default(), read: false }
      }
   }

//#[derive(Default)]
struct State{
   system : System,
   cache : canvas::Cache,
   read :bool
}

impl State{
   pub fn test() -> Self{

      let [t,l,r,b] = Self::frame();
      
      let poly : Polygon = Polygon::new(vec![Point::new(400.0,400.0),Point::new(430.0,400.0),Point::new(415.0,430.0),]);
      let poly2 : Polygon = Polygon::new(vec![Point::new(500.0,400.0),Point::new(530.0,400.0),Point::new(515.0,430.0),]);
      let obj1 : Object = Object::new(&poly, 2.0, false, true, 1, 2.0).with_starting_v(Vect::new(-70.0,0.0));
      let obj2 : Object = Object::new(&poly2, 2.0, false, true, 1, 2.0);
      let syst : System = System::new(vec![obj1,obj2,t,l,r,b], false, true, true);
      Self { system: syst, cache: Cache::new(), read: false }
   }

   pub fn projectile(angle: f64, mag : f64, air_res : Option<f64>, mass: f64) -> Self {
       //let [t,l,r,b] = Self::frame();
       let [t,_l,_r,_b] = Self::frame();
       let offset :f64  =-10.0;
      let poly : Polygon = Polygon::new(vec![
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
      let obj1 : Object = Object::new(&poly, mass, false, true, 1, 10.0)
         .with_starting_v(v);
      let syst : System = System::new(vec![obj1,t], false, true, air_res.is_some());
      
      Self { system: syst, cache: Cache::new(), read: false }
   }
   pub fn frame() -> [Object; 4]{
      let thickness = 30.0;
let w = 1024.0;
let h = 768.0;

// Bottom wall (y=0 is bottom in physics coords)
let frame_bottom = Object::new(&Polygon::new(vec![
    Point::new(0.0,       0.0),
    Point::new(w,         0.0),
    Point::new(w,         thickness),
    Point::new(0.0,       thickness),
]), 1.0, true, true, 60, 0.20);

// Top wall
let frame_top = Object::new(&Polygon::new(vec![
    Point::new(0.0,       h - thickness),
    Point::new(w,         h - thickness),
    Point::new(w,         h),
    Point::new(0.0,       h),
]), 1.0, true, true, 61, 0.20);

// Left wall
let frame_left = Object::new(&Polygon::new(vec![
    Point::new(0.0,       0.0),
    Point::new(thickness, 0.0),
    Point::new(thickness, h),
    Point::new(0.0,       h),
]), 1.0, true, true, 62,0.20);

// Right wall
let frame_right = Object::new(&Polygon::new(vec![
    Point::new(w - thickness, 0.0),
    Point::new(w,             0.0),
    Point::new(w,             h),
    Point::new(w - thickness, h),
]), 1.0, true, true, 63,0.20);
      [frame_top,frame_left,frame_right,frame_bottom]
   }
}

#[derive(Debug, Clone)]
enum Message {
   NotReady,
   Tick,Start,RequestLog
}




fn update(state: &mut State, message: Message) {
   match message{
      Message::Tick => {
         
         state.cache.clear();
         
         state.system.tick();
         // for i in state.system.objs_mut().iter_mut(){
         //    if i.com().y()>=800.0{
         //       i.reverse_v();
               
         //    }
         // }
        
         


      }
      Message::NotReady => {state.read=false;},
      Message::Start => {state.read =true; state.system.start();},
      Message::RequestLog => {
         let log: Vec<Physics::objects::ObjectLog> = state.system.request_object_logs();
         let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("Log.txt")
            .expect("Could not open Log.txt");
         for i in log.into_iter(){
            file.write("g".as_bytes()).expect("l");
         }
      }
   }
}

fn view(state: &'_ State) -> Element<'_, Message> {
   if state.read{
    Canvas::new(MyCanvas{ system :& state.system, cache : &state.cache})
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
   }
   else{
      c![ button("+").on_press(Message::Start)].into()
   }     
      
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
    Size, Subscription, Task
};
use core::f64;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::num::ParseIntError;
use std::time::Duration;

fn subscription(
    state: &State,
) -> Subscription<Message> {

   if state.read{
   Subscription::batch(vec![
        iced::time::every(Duration::from_secs_f64(1.0 / 60.0))
            .map(|_| Message::Tick),
        iced::time::every(Duration::from_secs(1))
            .map(|_| Message::RequestLog),
        
    ])
   }else  {
      iced::time::every(Duration::from_secs_f64(1.0 / 60.0))
            .map(|_| Message::NotReady)
   }
}




pub fn set_up_ptojectile() -> State{
   let mut vars: [f64; 4] = [0_f64;4];

   let nar_name: [&str; 4] = ["magnitiude velocity of the ball (m/s)","launch angle (deg)", "mass of the ball (grams)", "air resistance (coef, with recommened max of one)"];
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

// Subscription::batch(vec![
//         iced::time::every(Duration::from_secs_f64(1.0 / 60.0))
//             .map(|_| Message::Tick),
//         iced::time::every(Duration::from_secs(1))
//             .map(|_| Message::SecondTick),
//         iced::time::every(Duration::from_millis(500))
//             .map(|_| Message::HalfSecondTick),
//     ])