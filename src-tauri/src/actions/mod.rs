pub trait Action
{
    async fn run<T: serde::de::DeserializeOwned + Clone, R>(&self, f: Box<dyn FnOnce(T) -> Result<R, String>>) -> Result<R, String>;
}