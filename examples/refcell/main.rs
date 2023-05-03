trait Messager {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: Messager> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messager,
{
    fn new(sender: &'a T, max: usize) -> Self {
        LimitTracker {
            messager: sender,
            value: 0,
            max: max,
        }
    }

    fn set_value(&mut self, val: usize) {
        self.value = val;
        let p = self.value as f64 / self.max as f64;

        if p >= 1.0 {
            self.messager.send("Error: out of quota");
        } else if p >= 0.9 {
            self.messager.send("Warn: used up 90% of your quota");
        } else if p >= 0.75 {
            self.messager.send("Warn: used up 75% of your quota");
        }
    }
}

fn main() {}

// self.messages.push(msg.to_string());
//   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessager {
        messages: Vec<String>,
    }

    impl MockMessager {
        fn new() -> Self {
            MockMessager { messages: vec![] }
        }
    }

    impl Messager for MockMessager {
        fn send(&self, msg: &str) {
            self.messages.push(msg.to_string());
        }
    }

    #[test]
    fn test_send_over_75_percent_warn_message() {
        let mock_message = MockMessager::new();
        let mut limit_tracker = LimitTracker::new(&mock_message, 100);
        limit_tracker.set_value(100);

        assert_eq!(mock_message.messages.len(), 1);
    }
}
