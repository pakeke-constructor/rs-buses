
mod questions;
mod events;




#[cfg(test)]
mod tests {
    use crate::{events::new_event, questions::new_question};

    #[test]
    fn test_events() {
        struct Pos {
            x: i32,
            y: i32
        }
        let mut my_ev = new_event::<Pos>("explosion", "when an explosion happens");

        my_ev.on(| pos | -> () {
            // Todo, this doesn't actually test anything!
            // because rust hates state, so we can't really check :P
            println!("Hello {}, {}", pos.x, pos.y);
        });

        my_ev.call(&Pos {
            x: 20,
            y: 12
        });
    }


    #[test]
    fn test_question_normal() {
        struct Entity {
            health: i32,
            size: i32
        }

        fn mul(a: i32, b: i32) -> i32 {
            a * b
        }
        let mut my_q = new_question::<Entity, i32>(mul, "get damage multiplier", "gets damage multiplier");

        my_q.answer(| e | -> i32 {
            e.health
        });
        my_q.answer(| e | -> i32 {
            e.size
        });

        let dmg_mult = my_q.ask(&Entity {
            health: 20,
            size: 12
        }).unwrap();

        assert_eq!(dmg_mult, 240);
    }
}

