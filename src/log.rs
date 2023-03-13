use serde::{de::DeserializeOwned, Serialize};

#[derive(Serialize)]
pub struct LogEntry<'r, REQ, RES>
where
    REQ: Serialize + DeserializeOwned,
    RES: Serialize + DeserializeOwned,
{
    pub request: &'r REQ,
    pub response: &'r RES,
}
