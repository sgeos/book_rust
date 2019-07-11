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

  pub fn request_review(self) -> PendingReviewPost {
    PendingReviewPost {
      content: self.content,
      approved: false,
    }
  }
}

pub struct PendingReviewPost {
  content: String,
  approved: bool,
}

impl PendingReviewPost {
  pub fn approve(mut self) -> (Option<PendingReviewPost>, Option<Post>) {
    if self.approved {
      (
        None,
        Some(
          Post {
            content: self.content,
          }
        )
      )
    } else {
      self.approved = true;
      (Some(self), None)
    }
  }

  pub fn reject(self) -> DraftPost {
    DraftPost {
      content: self.content,
    }
  }
}

#[cfg(test)]
mod tests {
//  use super::*;

  #[test]
  fn success() {
    assert!(true);
  }
}

