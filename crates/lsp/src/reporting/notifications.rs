use std::fmt::Display;
use tower_lsp::lsp_types::MessageType;
use super::Reporter;


impl Reporter {
    pub async fn show_error_notification<M: Display>(&self, message: M) {
        self.client.show_message(MessageType::ERROR, &message).await;
        self.client.log_message(MessageType::ERROR, &message).await;
    }

    pub async fn show_warning_notification<M: Display>(&self, message: M) {
        self.client.show_message(MessageType::WARNING, &message).await;
        self.client.log_message(MessageType::WARNING, &message).await;
    }

    pub async fn show_info_notification<M: Display>(&self, message: M) {
        self.client.show_message(MessageType::INFO, &message).await;
        self.client.log_message(MessageType::INFO, &message).await;
    }
}