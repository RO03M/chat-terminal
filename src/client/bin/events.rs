use crossterm::event;

pub trait EventHandler {
    fn on_event(&mut self, _event: event::Event) {}
    fn on_key_press(&mut self, _key_event: event::KeyEvent) {}
    fn on_scroll(&mut self, _mouse_event: event::MouseEvent, _kind: event::MouseEventKind) {}
}
