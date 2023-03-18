pub struct BrowserHistory {
    history: Vec<String>,
    pos: i32,
}

impl BrowserHistory {
    pub fn new(homepage: String) -> Self {
        BrowserHistory {
            history: vec![homepage],
            pos: 0,
        }
    }

    pub fn visit(&mut self, url: String) {
        self.pos += 1;
        if self.pos != self.history.len() as i32 {
            self.history.truncate(self.pos as usize)
        }
        self.history.push(url);
    }

    pub fn back(&mut self, steps: i32) -> String {
        if (self.pos - steps) > 0 {
            self.pos -= steps;
        } else {
            self.pos = 0;
        }

        self.history[self.pos as usize].clone()
    }

    pub fn forward(&mut self, steps: i32) -> String {
        let len = self.history.len() as i32;
        if (self.pos + steps) < len {
            self.pos += steps;
        } else {
            self.pos = len - 1;
        }

        self.history[self.pos as usize].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browser_history() {
        let mut browser = BrowserHistory::new("leetcode.com".to_string());
        browser.visit("google.com".to_string());
        browser.visit("facebook.com".to_string());
        browser.visit("youtube.com".to_string());
        assert_eq!(browser.back(1), "facebook.com".to_string());
        assert_eq!(browser.back(1), "google.com".to_string());
        assert_eq!(browser.forward(1), "facebook.com".to_string());
        browser.visit("linkedin.com".to_string());
        assert_eq!(browser.forward(2), "linkedin.com".to_string());
        assert_eq!(browser.back(2), "google.com".to_string());
        assert_eq!(browser.back(7), "leetcode.com".to_string());
    }
}
