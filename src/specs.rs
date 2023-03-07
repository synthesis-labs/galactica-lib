use serde::{Deserialize, Serialize};

use crate::{auth::DiscordAccessToken, errors::Error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstructionRequest {
    pub token: DiscordAccessToken,
    pub instruction: Instruction,
    pub n: u32,
    pub history: Vec<HistoryEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Agent {
    User,
    Galactica,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HistoryEntry {
    pub agent: Agent,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstructionResponse {
    pub content: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstructionChunk {
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorResponse {
    pub error: Error,
}

type InstructionText = String;
type InstructionReference = String;

type EmailContent = String;
type SnippetStart = usize;
type SnippetLength = usize;
type Tone = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Instruction {
    GenerateCode(InstructionText),
    GenerateCodeWithReference(InstructionText, InstructionReference),
    Conversation(InstructionText),
    ConversationWithReference(InstructionText, InstructionReference),
    RetoneEmailParagraph(EmailContent, SnippetStart, SnippetLength, Tone),
}
