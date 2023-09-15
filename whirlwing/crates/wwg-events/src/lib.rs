#[derive(Debug)]
pub struct Event {
    event_type: EventType,
    category: EventCategory,
}

impl Event {
    pub fn new(event_type: EventType, category: EventCategory) -> Self {
        Event {
            event_type,
            category,
        }
    }

    pub fn event_type(&self) -> EventType {
        self.event_type
    }

    pub fn event_category(&self) -> EventCategory {
        self.category
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventType {
    ApplicationExit,
    KeyPressed { key: char, repeats: i32 },
    KeyReleased { key: char },
    MouseMoved { x_offset: i32, y_offset: i32 },
    LeftMouseButtonPressed { x: i32, y: i32 },
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct EventCategory : u32 {
        const WindowEvent   = 0b00000001;
        const MouseEvent    = 0b00000010;
        const KeyboardEvent = 0b00000100;
    }
}
