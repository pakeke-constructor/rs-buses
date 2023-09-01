
use std::vec::Vec;
use std::option::Option;



struct Event<T> {
    name: String,
    description: String,
    callbacks: Vec<fn(&T) -> ()>
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




struct Question<T, A> {
    name: String,
    description: String,
    answers: Vec<fn(&T) -> A>,
    reducer: fn(A,A) -> A
}



impl<T,A> Question<T, A> {
    pub fn ask(&self, data: &T) -> Option<A> {
        if self.answers.len() <= 0 {
            return Option::None;
        }

        let ans_func = self.answers.get(0).unwrap();
        let mut ret = ans_func(data);

        for answer in self.answers.iter() {
            let ans = answer(data);
            ret = (self.reducer)(ret, ans);
        }

        return Option::Some(ret);
    }

    pub fn answer(&mut self, answer: fn(&T) -> A) {
        self.answers.push(answer);
    }
}



pub fn newEvent(name: String, description: String) -> Event {
    let vec = Vec::new();
    let mut ev: Event = Event {
        name: name,
        description: description,
        callbacks: vec
    };

    ev
}





#[cfg(test)]
mod tests {
    #[test]
    fn test_events() {

        let result = 4;
        assert_eq!(result, 4);
    }


    #[test]
    fn test_questions() {
    }
}

