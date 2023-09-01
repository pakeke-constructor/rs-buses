

pub struct Question<T, A> {
    name: String,
    description: String,
    answers: Vec<dyn Fn(&T) -> A>,
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

