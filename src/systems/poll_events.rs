use crate::types::{EventContext, WorldStatus};
use ecs_rs::MutGR;
use glium::glutin;

impl_system_for!(PollEvents {
    fn run(event_context: MutGR<EventContext>, world_status: MutGR<WorldStatus>) {
        event_context.events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => world_status.window_should_close = true,
                glutin::WindowEvent::KeyboardInput {
                    input:
                        glutin::KeyboardInput {
                            scancode: keycode,
                            state: glutin::ElementState::Pressed,
                            ..
                        },
                        ..
                } => {
                    world_status.key.insert(keycode);
                },
                glutin::WindowEvent::KeyboardInput {
                    input:
                        glutin::KeyboardInput {
                            scancode: keycode,
                            state: glutin::ElementState::Released,
                            ..
                        },
                        ..
                } => {
                    world_status.key.remove(keycode);
                },
                        _ => (),
            },
            _ => (),
        });
    }
});
