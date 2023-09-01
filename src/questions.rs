

pub struct Question<T, A> {
    pub name: String,
    pub description: String,
    answers: Vec<Box<dyn Fn(&T) -> A>>,
    reducer: fn(A,A) -> A
}



impl<T,A> Question<T, A> {
    pub fn ask(&self, data: &T) -> Option<A> {
        if self.answers.len() <= 0 {
            return Option::None;
        }

        let ans_func = self.answers.get(0).unwrap();
        let mut ret = ans_func(data);

        for answer in self.answers.iter().skip(1) {
            let ans = answer(data);
            ret = (self.reducer)(ret, ans);
        }

        return Option::Some(ret);
    }

    pub fn answer(&mut self, answer: impl 'static + Fn(&T) -> A) {
        self.answers.push(Box::new(answer));
    }
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

