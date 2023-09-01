



struct Event<T> {
    callbacks: Vec<fn(T) -> ()>
}

trait Call {
    fn call() -> ();
}


impl Call for Event<T> {
    fn
}




pub fn call(event: Event, data: T) {
    let funcs = 
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

