pub struct Window {
    event_queue: VecDeque<Event>,
}

use std::collections::VecDeque;

use wwg_events::*;

impl Window {
    pub fn new() -> Self {
        Window { event_queue: VecDeque::new() }
    }

    pub fn add_event(&mut self, event: Event) {
        self.event_queue.push_back(event);
    }

    pub fn get_event(&mut self) -> Option<Event> {
        self.event_queue.pop_front()
    }
}