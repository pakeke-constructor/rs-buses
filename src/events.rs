
use std::vec::Vec;


pub struct Event<T> {
    name: String,
    description: String,
    callbacks: Vec<dyn Fn(&T) -> ()>
}


impl<T> Event<T> {
    pub fn call(&self, data: &T) {
        for func in self.callbacks.iter() {
            func(data);
        }
    }

    pub fn on(&mut self, callback: fn(&T) -> ()) {
        self.callbacks.push(callback);
    }
}


