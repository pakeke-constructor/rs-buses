

# rs-buses

Implements basic [event-buses and question-buses](https://github.com/pakeke-constructor/ideas/blob/master/question_buses.md) in Rust.

I'm not sure how useful these will be, (given how much Rust hates state,) but I still thought it was fun to make.



### Event buses:
```rust
struct Explosion {
    x: i32,
    y: i32,
    magnitude: f32
}

let mut my_ev = new_event::<Explosion>("explosion", "when an explosion happens");

my_ev.on(| pos | -> () {
    println!("BOOM", pos.x, pos.y);
});

my_ev.call(&Pos {
    x: 20,
    y: 12,
    magnitude: 100.0
});
```


### Question buses:
```rust
struct Entity {
    health: i32,
    size: i32
}

fn mul(a: i32, b: i32) -> i32 {
    a * b
}
let mut my_q = new_question::<Entity, i32>(mul, "get damage multiplier", "gets damage multiplier");

my_q.answer(| e | -> i32 {
    e.health // damage is multiplied by health
});
my_q.answer(| e | -> i32 {
    e.size // damage is multiplied by size
});

let ans = my_q.ask(&Entity {
    health: 20,
    size: 12
})

let dmg_mult = ans.unwrap_or_default(1);
// default damage multiplier is 1
```

## Diagrams:

![event_bus](images/event_bus.png)

![question_bus](images/question_bus.png)




