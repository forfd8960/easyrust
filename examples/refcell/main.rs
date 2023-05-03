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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessager {
        messages: RefCell<Vec<String>>,
    }

    impl MockMessager {
        fn new() -> Self {
            MockMessager {
                messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messager for MockMessager {
        fn send(&self, msg: &str) {
            self.messages.borrow_mut().push(msg.to_string());
        }
    }

    #[test]
    fn test_send_over_75_percent_warn_message() {
        let mock_message = MockMessager::new();
        let mut limit_tracker = LimitTracker::new(&mock_message, 100);
        limit_tracker.set_value(75);

        let vec_str = mock_message.messages.borrow();
        let err_msg = vec_str.get(0);
        assert_eq!(vec_str.len(), 1);
        assert_eq!(
            err_msg,
            Some(&"Warn: used up 75% of your quota".to_string())
        );
    }

    #[test]
    fn test_send_over_90_percent_warn_message() {
        let mock_message = MockMessager::new();
        let mut limit_tracker = LimitTracker::new(&mock_message, 100);
        limit_tracker.set_value(100);

        let vec_str = mock_message.messages.borrow();
        let err_msg = vec_str.get(0);

        assert_eq!(err_msg, Some(&"Error: out of quota".to_string()));
    }
}
