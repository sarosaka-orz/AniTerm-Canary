use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestType {
    Command,
    FetchGreeting,
    SwitchPersona,
    GetStatus,
    Chat,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IpcRequest {
    pub request_type: RequestType,
    pub command: String,
    pub output: String,
    pub system_info: Option<String>,
    pub force_env: bool,
    pub force_holiday: Option<String>, // 新增：強制指定的節日
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IpcResponse {
    pub message: String,
    pub persona_name: String,
}
