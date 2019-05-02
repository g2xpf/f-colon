use crate::types::RenderContext;
use ecs_rs::MutGR;

impl_system_for!(Finalize {
    fn run(render_context: MutGR<RenderContext>) {
        let mut frame = render_context.frame.as_mut().unwrap();
        frame.set_finish().unwrap();
    }
});
