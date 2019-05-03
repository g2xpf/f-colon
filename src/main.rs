extern crate cgmath;
extern crate ecs_rs;
extern crate f_colon;
extern crate glium;
extern crate image;

use ecs_rs::World;
use f_colon::systems::*;
use f_colon::types::*;

fn main() {
    use glium::glutin;

    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("GLWindow")
        .with_dimensions(glutin::dpi::LogicalSize::new(600.0, 600.0));
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut world = World::new();

    world.push_global_resource(RenderContext::new(display));
    world.push_global_resource(EventContext::new(events_loop));
    world.push_global_resource(WorldStatus::new());

    world.register_system::<ClearColor>();
    world.register_system::<Finalize>();
    world.register_system::<PollEvents>();

    world.run();
}
