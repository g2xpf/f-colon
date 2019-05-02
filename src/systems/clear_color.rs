use crate::types::RenderContext;
use ecs_rs::MutGR;
use glium::Surface;

impl_system_for!(ClearColor {
    fn run(render_context: MutGR<RenderContext>) {
        let mut frame = render_context.display.draw();
        frame.clear_color(1.0, 1.0, 1.0, 0.0);
        render_context.frame = Some(frame);
    }
});
