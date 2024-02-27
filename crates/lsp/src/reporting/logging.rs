use std::fmt::Display;
use tower_lsp::lsp_types::MessageType;
use crate::Backend;


impl Backend {
    pub async fn log_error<M: Display>(&self, message: M) {
        self.client.log_message(MessageType::ERROR, message).await;
    }

    pub async fn log_warning<M: Display>(&self, message: M) {
        self.client.log_message(MessageType::WARNING, message).await;
    }

    pub async fn log_info<M: Display>(&self, message: M) {
        self.client.log_message(MessageType::INFO, message).await;
    }
}