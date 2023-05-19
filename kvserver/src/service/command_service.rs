use crate::{
    CommandResponse, CommandService, Hexist, Hget, Hgetall, Hset, KvError, Storage, Value,
};

impl CommandService for Hget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hexist {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.contains(&self.table, &self.key) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hgetall {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get_all(&self.table) {
            Ok(paris) => paris.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match self.pair {
            Some(pair) => match store.set(&self.table, pair.key, pair.value.unwrap_or_default()) {
                Ok(Some(v)) => v.into(),
                Ok(None) => Value::default().into(),
                Err(e) => e.into(),
            },
            None => Value::default().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use http::StatusCode;

    use super::*;
    use crate::{dispatch, CommandRequest, Kvpair, MemTable};

    #[test]
    fn hset_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        let res = dispatch(cmd.clone(), &store);
        assert_eq!(res.values, &[Value::default()]);
        assert_eq!(res.pairs, &[]);
    }

    #[test]
    fn hget_work() {
        let store = MemTable::new();
        let set_cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        dispatch(set_cmd.clone(), &store);

        let cmd = CommandRequest::new_hget("t1", "hello");
        let res = dispatch(cmd.clone(), &store);
        assert_eq!(res.values, &["world".into()]);
        assert_eq!(res.pairs, &[]);
    }

    #[test]
    fn hget_non_exists() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hget("t1", "hello");
        let res = dispatch(cmd.clone(), &store);
        assert_eq!(res.status, StatusCode::NOT_FOUND.as_u16() as u32);
        assert_eq!(res.values, &[]);
    }

    #[test]
    fn hget_all_work() {
        let store = MemTable::new();
        let cmds = vec![
            CommandRequest::new_hset("t1", "hello", "world".into()),
            CommandRequest::new_hset("t1", "nice", "day".into()),
        ];
        for cmd in cmds {
            dispatch(cmd.clone(), &store);
        }

        let hget_all_cmd = CommandRequest::new_hgetall("t1");
        let res = dispatch(hget_all_cmd.clone(), &store);

        assert_eq!(res.status, StatusCode::OK.as_u16() as u32);
        assert_eq!(res.values, &[]);
        assert_eq!(
            res.pairs,
            &[
                Kvpair::new("nice", "day".into()),
                Kvpair::new("hello", "world".into()),
            ]
        );
    }
}
