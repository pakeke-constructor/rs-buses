
use std::vec::Vec;

mod questions;
mod events;




pub fn new_event<T>(name: &str, description: &str) -> Event<T, dyn Fn(&T) -> ()> {
    let vec = Vec::new();
    let ev: Event<T> = Event {
        name: name.to_string(),
        description: description.to_string(),
        callbacks: vec
    };

    ev
}


pub fn new_question<T,A>(reducer: fn(A,A) -> A, name: &str, description: &str) -> Question<T,A> {
    let vec = Vec::new();
    let question: Question<T,A> = Question {
        name: name.to_string(),
        description: description.to_string(),
        reducer: reducer,
        answers: vec,
    };

    question
}



#[cfg(test)]
mod tests {
    use crate::{new_event, new_question};

    #[test]
    fn test_events() {
        struct Pos {
            x: f32,
            y: f32
        }
        let my_ev = new_event::<Pos>("explosion", "when an explosion happens");

        let mut x = 0;
        my_ev.on(| pos | -> () {
            x = 10 + pos.x + pos.y;
        });

        my_ev.call(Pos {
            x: 20,
            y: 12
        });

        assert_eq!(x, 42);
    }


    #[test]
    fn test_question_normal() {
        struct Pos {
            x: f32,
            y: f32
        }

        fn mul(a: i32, b: i32) {
            a * b
        }
        let my_q = new_question::<Pos>(mul, "explosion", "when an explosion happens");

        let mut x = 0;
        my_q.answer(| pos | -> () {
            x = pos.x + pos.y;
        });
        my_q.answer(| pos | -> () {
            x = pos.x + pos.y;
        });

        let answer = my_q.ask(Pos {
            x: 20,
            y: 12
        });

        assert_eq!(x, 42);

    }
}

