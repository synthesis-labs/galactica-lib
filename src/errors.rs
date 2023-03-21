use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Error {
    GeneralError(String),
    DiscordOAuthError(String),
    DiscordApiError(String),
    OpenAIError(String),
    UnableToSerialize(String),
    UnableToDeserialize(String, String),
    ConfigError(String, String),
    UnableToLog(String),
    ParsingError(String),
    GithubError(String),
    NotImplemented,
}
