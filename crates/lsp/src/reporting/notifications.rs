use std::fmt::Display;
use tower_lsp::lsp_types::MessageType;
use crate::Backend;


impl Backend {
    pub async fn show_error_notification<M: Display>(&self, message: M) {
        self.client.show_message(MessageType::ERROR, message).await
    }

    pub async fn show_warning_notification<M: Display>(&self, message: M) {
        self.client.show_message(MessageType::WARNING, message).await
    }

    pub async fn show_info_notification<M: Display>(&self, message: M) {
        self.client.show_message(MessageType::INFO, message).await
    }
}