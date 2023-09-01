
use std::vec::Vec;


pub struct Event<T> {
    pub name: String,
    pub description: String,
    callbacks: Vec<Box<dyn Fn(&T) -> ()>>
}


impl<T> Event<T> {
    pub fn call(&self, data: &T) {
        for func in self.callbacks.iter() {
            func(data);
        }
    }

    pub fn on(&mut self, callback: impl 'static + Fn(&T) -> ()) {
        self.callbacks.push(Box::new(callback));
    }
}



pub fn new_event<T>(name: &str, description: &str) -> Event<T> {
    let vec = Vec::new();
    let ev: Event<T> = Event {
        name: name.to_string(),
        description: description.to_string(),
        callbacks: vec
    };

    ev
}



