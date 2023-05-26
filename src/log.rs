use serde::{de::DeserializeOwned, Serialize};

#[derive(Serialize)]
pub struct LogEntry<'r, USER, REQ, RES>
where
    USER: Serialize + DeserializeOwned,
    REQ: Serialize + DeserializeOwned,
    RES: Serialize + DeserializeOwned,
{
    pub user: Option<&'r USER>,
    pub request: &'r REQ,
    pub response: &'r RES,
}
