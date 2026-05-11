use winit::{
   event::{Event, WindowEvent},
   event_loop::{ControlFlow, EventLoop},
   window::WindowBuilder,
};
fn main() {
   let event_loop = EventLoop::new();
   let _window = WindowBuilder::new()
       .with_title("Hello, Winit!")
       .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
       .build(&event_loop)
       .unwrap();
   event_loop.run(move |event: Event<'_, ()>, _, control_flow: &mut ControlFlow| {
       *control_flow = ControlFlow::Wait;
       if let Event::WindowEvent { event, .. } = event {
           if let WindowEvent::CloseRequested = event {
               *control_flow = ControlFlow::Exit;
           }
       }
   });
}