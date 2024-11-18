pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingPost {
        PendingPost {
            content: self.content,
        }
    }
}

pub struct PendingPost {
    content: String,
}
impl PendingPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

struct Pos {
    x: i32,
    y: i32,
}

fn main() {
    let mut post = Post::new();

    post.content.push_str("remark1");

    let post = post.request_review();

    let post = post.approve();

    // println!("contents {}", post.content);

    let _a = Some(5);

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(5);

    let slice = &mut [5];

    let ptr = slice.as_mut_ptr();
}
