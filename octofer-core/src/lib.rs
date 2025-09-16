#[derive(Clone, Debug)]
pub struct Context {}

/// This type represents a function that handles an event
pub type EventHandlerFn = Box<
    dyn Fn(
            Context,
        )
            -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send>>
        + Send
        + Sync,
>;

/// Whichever function implements this trait, it's an event handler
pub trait EventHandler: Send + Sync {
    fn handle(
        &self,
        context: Context,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send>>;
}
