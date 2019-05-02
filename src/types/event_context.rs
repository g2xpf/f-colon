use glium::glutin::EventsLoop;

pub struct EventContext {
    pub events_loop: EventsLoop,
}

impl EventContext {
    pub fn new(events_loop: EventsLoop) -> Self {
        EventContext { events_loop }
    }
}
