use serde::{de::DeserializeOwned, Serialize};

#[derive(Serialize)]
struct LogEntry<'r, REQ, RES>
where
    REQ: Serialize + DeserializeOwned,
    RES: Serialize + DeserializeOwned,
{
    request: &'r REQ,
    response: &'r RES,
}
