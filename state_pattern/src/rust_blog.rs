pub struct Post {
    content: String,
}
pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
    approvals: i8,
    approvals_required: i8,
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
    const APPROVALS_REQUIRED: i8 = 2;
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approvals: 0,
            approvals_required: Self::APPROVALS_REQUIRED,
        }
    }
}

impl PendingReviewPost {
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    pub fn approve(&mut self) -> Option<Post> {
        self.approvals += 1;
        if self.approvals >= self.approvals_required {
            Some(Post {
                content: self.content.clone(),
            })
        } else {
            None
        }
    }
}
