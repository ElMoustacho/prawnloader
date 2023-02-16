pub struct EventManager<T> {
    callbacks: Vec<Box<dyn Fn(&T) -> () + Send>>,
}

impl<T> EventManager<T> {
    pub fn new() -> EventManager<T> {
        EventManager {
            callbacks: Vec::new(),
        }
    }

    pub fn add_callback<F>(&mut self, callback: F)
    where
        F: Fn(&T) -> () + Send + 'static,
    {
        self.callbacks.push(Box::new(callback));
    }

    pub(crate) fn emit_event(&self, event: T) {
        for callback in self.callbacks.iter() {
            (callback)(&event);
        }
    }
}
