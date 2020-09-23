use async_trait::async_trait;
use std::collections::HashMap;

/// Describes synchronous config source.
pub trait Source {
    /// Synchronous function to collect source into key value pairs.
    fn collect(&self) -> HashMap<String, String>;
}

#[async_trait]
/// Describes asynchronous config source.
pub trait AsyncSource {
    /// Asynchronous function to collect source into key value pairs.
    async fn collect(&self) -> HashMap<String, String>;
}
