

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn write(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        state.as_ref().unwrap().content(&self)
    }

    pub fn request(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request())
        }
    }

    pub fn approve(&mut self) {
        if Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str { "" }
}

struct Draft {}

impl State for Draft {
    fn request(self: Box<Draft>) -> Box<dyn State> { Box::new(PendingReview {}) }
    fn approve(self: Box<Draft>) -> Box<dyn State> { self }
}

struct PendingReview {}

impl State for PendingReview {
    fn request(self: Box<Self>) -> Box<dyn State> { self }
    fn approve(self: Box<Self>) -> Box<dyn State> { Box::new(Published {}) }
}

struct Published {}

impl State for Published {
    fn request(self: Box<Self>) -> Box<dyn State> { self }
    fn approve(self: Box<Self>) -> Box<dyn State> { self }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
